package crawl

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
	"time"

	"loom/crawler/internal/contracts"
	"loom/crawler/internal/contracts/gen"
	"loom/crawler/internal/fetch"
	"loom/crawler/internal/pack"
)

const devdocsDir = "../../../../verticals/devdocs"

// miniSite is a scaled-down fixture site: the same hazards, few enough pages to
// assert about exactly. It mirrors infra/fixture-site so a bug found here is a
// bug that would have shown up in the real crawl.
func miniSite() http.Handler {
	page := func(title string, links ...string) string {
		var b strings.Builder
		fmt.Fprintf(&b, "<html><head><title>%s</title></head><body><h1>%s</h1><nav>", title, title)
		for _, l := range links {
			fmt.Fprintf(&b, `<a href="%s">%s</a>`, l, l)
		}
		b.WriteString("</nav></body></html>")
		return b.String()
	}

	mux := http.NewServeMux()
	serve := func(path, body string) {
		mux.HandleFunc(path, func(w http.ResponseWriter, r *http.Request) {
			w.Header().Set("Content-Type", "text/html; charset=utf-8")
			fmt.Fprint(w, body)
		})
	}

	mux.HandleFunc("/robots.txt", func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/plain")
		fmt.Fprint(w, "User-agent: *\nDisallow: /private/\n")
	})

	serve("/index.html", page("Index", "/js/array-map.html", "/js/array-filter.html",
		"/private/secret.html", "/calendar/day-0.html", "/redirect/a"))
	serve("/js/array-map.html", page("Array.prototype.map()", "/js/array-map-printable.html", "/index.html"))
	serve("/js/array-map-printable.html", page("Array.prototype.map() printable"))
	serve("/js/array-filter.html", page("Array.prototype.filter()", "/index.html"))
	serve("/private/secret.html", page("PRIVATE — fetching this means politeness is broken"))

	// Depth trap: each day links only to the next.
	mux.HandleFunc("/calendar/", func(w http.ResponseWriter, r *http.Request) {
		var day int
		if _, err := fmt.Sscanf(r.URL.Path, "/calendar/day-%d.html", &day); err != nil {
			http.NotFound(w, r)
			return
		}
		w.Header().Set("Content-Type", "text/html")
		fmt.Fprint(w, page(fmt.Sprintf("Day %d", day), fmt.Sprintf("/calendar/day-%d.html", day+1)))
	})

	mux.HandleFunc("/redirect/a", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "/js/array-map.html", http.StatusMovedPermanently)
	})

	return mux
}

// collectSink gathers records and validates each against the contract, the way
// the real segment writer will.
type collectSink struct {
	mu      sync.Mutex
	records []*gen.CrawlRecordV1
	err     error
}

func (s *collectSink) Write(rec *gen.CrawlRecordV1) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	raw, err := json.Marshal(rec)
	if err != nil {
		s.err = err
		return err
	}
	if err := contracts.Validate(contracts.CrawlRecordV1, raw); err != nil {
		s.err = err
		return err
	}
	s.records = append(s.records, rec)
	return nil
}

func (s *collectSink) urls() []string {
	s.mu.Lock()
	defer s.mu.Unlock()
	out := make([]string, 0, len(s.records))
	for _, r := range s.records {
		out = append(out, r.CanonicalUrl)
	}
	return out
}

func runMiniCrawl(t *testing.T) (Stats, *collectSink, *httptest.Server) {
	t.Helper()

	srv := httptest.NewServer(miniSite())
	t.Cleanup(srv.Close)

	base, err := pack.Load(devdocsDir)
	if err != nil {
		t.Fatalf("loading pack: %v", err)
	}
	p, err := pack.FixtureOverride(base, srv.URL)
	if err != nil {
		t.Fatalf("fixture override for %s: %v", srv.URL, err)
	}

	clock := fetch.SystemClock{}
	limiter, err := fetch.NewHostLimiter(clock, 1000, 1000) // pacing has its own tests
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}
	fetcher := &fetch.Fetcher{
		Client:       srv.Client(),
		UserAgent:    "LoomBot/0.1 (+https://github.com/samibk/loom)",
		MaxBodyBytes: 1 << 20,
		Limiter:      limiter,
		Clock:        clock,
		MaxRedirects: 5,
	}

	sink := &collectSink{}
	ctx, cancel := context.WithTimeout(context.Background(), 30*time.Second)
	defer cancel()

	stats, err := Run(ctx, p, Deps{
		Fetcher:   fetcher,
		Sink:      sink,
		Clock:     clock,
		Limiter:   limiter,
		UserAgent: "LoomBot",
	})
	if err != nil {
		t.Fatalf("Run: %v", err)
	}
	if sink.err != nil {
		t.Fatalf("a record failed contract validation: %v", sink.err)
	}
	return stats, sink, srv
}

// The politeness assertion. If this fails, the crawler fetched a page it was
// told not to, and every claim the project makes about being polite is false.
func TestRunNeverFetchesDisallowedPaths(t *testing.T) {
	_, sink, _ := runMiniCrawl(t)

	for _, u := range sink.urls() {
		if strings.Contains(u, "/private/") {
			t.Errorf("fetched %q, which robots.txt disallows", u)
		}
	}
	if len(sink.urls()) == 0 {
		t.Fatal("the crawl fetched nothing at all")
	}
}

// An empty frontier does not mean the crawl is finished — an in-flight fetch
// may be about to push more URLs. A crawler that stops on "queue empty" alone
// ends after the first page.
func TestRunReachesPagesBeyondTheSeed(t *testing.T) {
	_, sink, srv := runMiniCrawl(t)

	want := []string{
		srv.URL + "/index.html",
		srv.URL + "/js/array-map.html",
		srv.URL + "/js/array-filter.html",
		srv.URL + "/js/array-map-printable.html", // depth 2 — only reachable via array-map
	}
	got := sink.urls()

	for _, w := range want {
		if !slicesContains(got, w) {
			t.Errorf("never fetched %q\nfetched: %q", w, got)
		}
	}
}

func TestRunHonorsDepthLimit(t *testing.T) {
	_, sink, _ := runMiniCrawl(t)

	// The calendar trap is unbounded; max_depth is what stops it.
	calendar := 0
	for _, u := range sink.urls() {
		if strings.Contains(u, "/calendar/") {
			calendar++
		}
	}
	if calendar == 0 {
		t.Error("the calendar trap was never entered; the test is not exercising the depth cap")
	}
	if calendar > 8 { // pack max_depth
		t.Errorf("fetched %d calendar pages; max_depth should have stopped it sooner", calendar)
	}
}

// Links on a redirected page must resolve against where the page ended up, not
// where it was requested from.
func TestRunResolvesLinksAgainstFinalURL(t *testing.T) {
	_, sink, srv := runMiniCrawl(t)

	for _, u := range sink.urls() {
		if strings.Contains(u, "/redirect/") {
			t.Errorf("stored %q as an identity; a redirect's target is the document", u)
		}
	}
	if !slicesContains(sink.urls(), srv.URL+"/js/array-map.html") {
		t.Error("the redirect chain did not resolve to its target")
	}
}

func TestRunAccountsForEveryURL(t *testing.T) {
	stats, sink, _ := runMiniCrawl(t)

	if stats.Fetched != len(sink.urls()) {
		t.Errorf("Stats.Fetched = %d but the sink received %d records",
			stats.Fetched, len(sink.urls()))
	}
	if stats.RobotsDenied == 0 {
		t.Error("nothing was counted as robots-denied, but /private/ was linked from the index")
	}
}

// A cancelled crawl stops promptly and keeps what it already gathered.
func TestRunHonorsCancellation(t *testing.T) {
	srv := httptest.NewServer(miniSite())
	t.Cleanup(srv.Close)

	base, err := pack.Load(devdocsDir)
	if err != nil {
		t.Fatalf("loading pack: %v", err)
	}
	p, err := pack.FixtureOverride(base, srv.URL)
	if err != nil {
		t.Fatalf("fixture override: %v", err)
	}

	clock := fetch.SystemClock{}
	limiter, err := fetch.NewHostLimiter(clock, 1000, 1000)
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	cancel() // cancelled before it starts

	done := make(chan struct{})
	go func() {
		defer close(done)
		_, _ = Run(ctx, p, Deps{
			Fetcher: &fetch.Fetcher{
				Client: srv.Client(), UserAgent: "LoomBot", MaxBodyBytes: 1 << 20,
				Limiter: limiter, Clock: clock, MaxRedirects: 5,
			},
			Sink: &collectSink{}, Clock: clock, Limiter: limiter, UserAgent: "LoomBot",
		})
	}()

	select {
	case <-done:
	case <-time.After(5 * time.Second):
		t.Fatal("Run ignored a cancelled context")
	}
}

func slicesContains(haystack []string, needle string) bool {
	for _, h := range haystack {
		if h == needle {
			return true
		}
	}
	return false
}
