package fetch

import (
	"context"
	"errors"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

// newTestFetcher wires a Fetcher against an httptest server. Nothing in this
// package ever touches the live network — the fixture site and these fakes play
// the web.
func newTestFetcher(t *testing.T, handler http.Handler) (*Fetcher, *httptest.Server, *fakeClock) {
	t.Helper()
	srv := httptest.NewServer(handler)
	t.Cleanup(srv.Close)

	clock := newFakeClock()
	limiter, err := NewHostLimiter(clock, 1000, 1000) // effectively unpaced; pacing has its own tests
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}

	return &Fetcher{
		Client:       srv.Client(),
		UserAgent:    "LoomBot/0.1 (+https://github.com/samibk/loom)",
		MaxBodyBytes: 1 << 20,
		Limiter:      limiter,
		Clock:        clock,
		MaxRedirects: 5,
	}, srv, clock
}

func htmlHandler(body string) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		fmt.Fprint(w, body)
	})
}

func TestFetchSuccess(t *testing.T) {
	f, srv, _ := newTestFetcher(t, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html; charset=utf-8")
		w.Header().Set("ETag", `"abc123"`)
		w.Header().Set("Last-Modified", "Mon, 27 Jul 2026 12:00:00 GMT")
		fmt.Fprint(w, "<html><title>Array.prototype.map()</title></html>")
	}))

	resp, err := f.Fetch(context.Background(), srv.URL+"/js/array-map.html")
	if err != nil {
		t.Fatalf("Fetch: %v", err)
	}
	if resp.StatusCode != http.StatusOK {
		t.Errorf("StatusCode = %d, want 200", resp.StatusCode)
	}
	// Parameters must be stripped: the indexer switches on the bare type.
	if resp.ContentType != "text/html" {
		t.Errorf("ContentType = %q, want %q", resp.ContentType, "text/html")
	}
	if !strings.Contains(string(resp.Body), "Array.prototype.map()") {
		t.Errorf("body does not contain the page title: %q", resp.Body)
	}
	if resp.ETag != `"abc123"` {
		t.Errorf("ETag = %q, want %q", resp.ETag, `"abc123"`)
	}
	if resp.Truncated {
		t.Error("Truncated set for a body well under the cap")
	}
}

// The crawler identifies itself. RFC 9309 §2.1, and it is what lets a site
// owner block us if they want to.
func TestFetchSendsUserAgent(t *testing.T) {
	got := make(chan string, 1)
	f, srv, _ := newTestFetcher(t, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		got <- r.Header.Get("User-Agent")
		w.Header().Set("Content-Type", "text/html")
		fmt.Fprint(w, "<html></html>")
	}))

	if _, err := f.Fetch(context.Background(), srv.URL+"/x"); err != nil {
		t.Fatalf("Fetch: %v", err)
	}
	if ua := <-got; !strings.Contains(ua, "LoomBot") {
		t.Errorf("User-Agent = %q, want it to identify LoomBot", ua)
	}
}

func TestFetchClassifiesStatus(t *testing.T) {
	tests := []struct {
		name   string
		status int
		want   error
	}{
		{"404 is fatal", http.StatusNotFound, ErrFatal},
		{"403 is fatal", http.StatusForbidden, ErrFatal},
		{"410 is fatal", http.StatusGone, ErrFatal},
		{"429 is retryable", http.StatusTooManyRequests, ErrRetryable},
		{"500 is retryable", http.StatusInternalServerError, ErrRetryable},
		{"503 is retryable", http.StatusServiceUnavailable, ErrRetryable},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			f, srv, _ := newTestFetcher(t, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				w.WriteHeader(tc.status)
			}))

			_, err := f.Fetch(context.Background(), srv.URL+"/x")
			if err == nil {
				t.Fatalf("status %d should have produced an error", tc.status)
			}
			if !errors.Is(err, tc.want) {
				t.Errorf("status %d: error %v does not match %v — every failure must be classifiable",
					tc.status, err, tc.want)
			}
		})
	}
}

// The fixture site's /redirect/a -> /redirect/b -> /js/array-map.html chain.
// The final URL is the document's identity, not the requested one.
func TestFetchFollowsRedirectChain(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/redirect/a", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "/redirect/b", http.StatusMovedPermanently)
	})
	mux.HandleFunc("/redirect/b", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "/js/array-map.html", http.StatusMovedPermanently)
	})
	mux.Handle("/js/array-map.html", htmlHandler("<html><title>Array.prototype.map()</title></html>"))

	f, srv, _ := newTestFetcher(t, mux)

	resp, err := f.Fetch(context.Background(), srv.URL+"/redirect/a")
	if err != nil {
		t.Fatalf("Fetch: %v", err)
	}
	if !strings.HasSuffix(resp.FinalURL, "/js/array-map.html") {
		t.Errorf("FinalURL = %q, want it to end at /js/array-map.html", resp.FinalURL)
	}
}

// The fixture site's /loop/x <-> /loop/y. This must terminate and say what it
// was, not hang until the context times out.
func TestFetchDetectsRedirectLoop(t *testing.T) {
	mux := http.NewServeMux()
	mux.HandleFunc("/loop/x", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "/loop/y", http.StatusMovedPermanently)
	})
	mux.HandleFunc("/loop/y", func(w http.ResponseWriter, r *http.Request) {
		http.Redirect(w, r, "/loop/x", http.StatusMovedPermanently)
	})

	f, srv, _ := newTestFetcher(t, mux)

	_, err := f.Fetch(context.Background(), srv.URL+"/loop/x")
	if err == nil {
		t.Fatal("a redirect loop should be an error, not an infinite fetch")
	}
	if !errors.Is(err, ErrFatal) {
		t.Errorf("redirect loop gave %v, want ErrFatal", err)
	}
}

// A crawler that trusts the server about size is one hostile server away from
// being OOM-killed.
func TestFetchCapsBodySize(t *testing.T) {
	const cap = 1024
	f, srv, _ := newTestFetcher(t, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/html")
		// Lies about the length, then sends far more than the cap.
		w.Header().Set("Content-Length", "10")
		for range 100 {
			fmt.Fprint(w, strings.Repeat("a", 1024))
		}
	}))
	f.MaxBodyBytes = cap

	resp, err := f.Fetch(context.Background(), srv.URL+"/big")
	if err != nil {
		if errors.Is(err, ErrFatal) {
			return // refusing an oversized body is also acceptable
		}
		t.Fatalf("Fetch: %v", err)
	}
	if int64(len(resp.Body)) > cap {
		t.Errorf("body is %d bytes, cap is %d", len(resp.Body), cap)
	}
	if !resp.Truncated {
		t.Error("Truncated should be set when the cap was hit")
	}
}

// This is a page crawler. A video is not a parse failure, it is a resource
// attack, and it must be refused before the body is read.
func TestFetchRejectsNonHTML(t *testing.T) {
	for _, ct := range []string{"application/pdf", "image/png", "video/mp4", "application/octet-stream"} {
		t.Run(ct, func(t *testing.T) {
			f, srv, _ := newTestFetcher(t, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				w.Header().Set("Content-Type", ct)
				fmt.Fprint(w, "not html")
			}))

			if _, err := f.Fetch(context.Background(), srv.URL+"/x"); !errors.Is(err, ErrFatal) {
				t.Errorf("Content-Type %q gave %v, want ErrFatal", ct, err)
			}
		})
	}
}

// Every network attempt must be paced. A fetcher that skips the limiter on any
// path is a fetcher that will eventually hammer someone.
func TestFetchWaitsOnLimiter(t *testing.T) {
	f, srv, clock := newTestFetcher(t, htmlHandler("<html></html>"))

	limiter, err := NewHostLimiter(clock, 1, 1)
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}
	f.Limiter = limiter

	ctx := context.Background()
	if _, err := f.Fetch(ctx, srv.URL+"/a"); err != nil {
		t.Fatalf("first Fetch: %v", err)
	}

	done := make(chan struct{})
	go func() {
		defer close(done)
		_, _ = f.Fetch(ctx, srv.URL+"/b")
	}()

	// The second fetch must be waiting on the budget, not already in flight.
	for range 100 {
		if clock.Sleepers() > 0 {
			clock.Advance(2 * 1e9) // 2s, enough for a 1/s budget
			<-done
			return
		}
	}
	t.Fatal("the second Fetch did not consult the limiter")
}

func TestFetchRobotsMissingMeansAllowed(t *testing.T) {
	f, srv, _ := newTestFetcher(t, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		http.NotFound(w, r)
	}))

	body, err := f.FetchRobots(context.Background(), srv.URL)
	if err != nil {
		t.Fatalf("a 404 robots.txt means no rules, not an error: %v", err)
	}
	if len(body) != 0 {
		t.Errorf("body = %q, want empty for a missing robots.txt", body)
	}
}

// The tempting bug: treating a 5xx robots.txt as "no rules". That turns a bad
// afternoon for a site into a crawl of a site that told us to go away.
func TestFetchRobotsServerErrorIsRetryableNotPermissive(t *testing.T) {
	f, srv, _ := newTestFetcher(t, http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusServiceUnavailable)
	}))

	_, err := f.FetchRobots(context.Background(), srv.URL)
	if err == nil {
		t.Fatal("a 503 robots.txt must not be read as permission to crawl")
	}
	if !errors.Is(err, ErrRetryable) {
		t.Errorf("error %v, want ErrRetryable", err)
	}
}
