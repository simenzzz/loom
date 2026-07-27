package frontier

import (
	"fmt"
	"testing"

	"loom/crawler/internal/contracts/gen"
	"loom/crawler/internal/pack"
)

const devdocsDir = "../../../../verticals/devdocs"

// fixturePack loads the real devdocs pack and points it at the fixture site,
// which is what `make crawl-fixture` does.
func fixturePack(t *testing.T) *gen.VerticalPackV1 {
	t.Helper()
	base, err := pack.Load(devdocsDir)
	if err != nil {
		t.Fatalf("loading pack: %v", err)
	}
	p, err := pack.FixtureOverride(base, "http://localhost:7799")
	if err != nil {
		t.Fatalf("fixture override: %v", err)
	}
	return p
}

func TestNewSeedsTheQueue(t *testing.T) {
	p := fixturePack(t)

	f, err := New(p)
	if err != nil {
		t.Fatalf("New: %v", err)
	}
	if f.Len() != len(p.Seeds) {
		t.Errorf("Len() = %d, want %d (one per seed)", f.Len(), len(p.Seeds))
	}

	item, ok := f.Pop()
	if !ok {
		t.Fatal("Pop() on a freshly seeded frontier returned nothing")
	}
	if item.Depth != 0 {
		t.Errorf("seed depth = %d, want 0", item.Depth)
	}
}

func TestNewRejectsNilPack(t *testing.T) {
	if _, err := New(nil); err == nil {
		t.Error("New(nil) should fail — every limit comes from the pack")
	}
}

// Breadth-first, not depth-first. A LIFO frontier would spend the whole page
// budget descending the calendar trap and never reach half the corpus.
func TestPopIsFIFO(t *testing.T) {
	f, err := New(fixturePack(t))
	if err != nil {
		t.Fatalf("New: %v", err)
	}
	for f.Len() > 0 {
		f.Pop()
	}

	want := []string{
		"http://localhost:7799/a",
		"http://localhost:7799/b",
		"http://localhost:7799/c",
	}
	for _, u := range want {
		if !f.Push(u, 1) {
			t.Fatalf("Push(%q) rejected", u)
		}
	}
	for _, expected := range want {
		item, ok := f.Pop()
		if !ok {
			t.Fatalf("Pop() returned nothing, wanted %q", expected)
		}
		if item.URL != expected {
			t.Errorf("Pop() = %q, want %q — the queue is not FIFO", item.URL, expected)
		}
	}
}

func TestPushRejections(t *testing.T) {
	tests := []struct {
		name    string
		url     string
		depth   int
		counter func(Stats) int
	}{
		{
			name: "off-host", url: "http://evil.example/x", depth: 1,
			counter: func(s Stats) int { return s.OffHost },
		},
		{
			name: "too deep", url: "http://localhost:7799/deep", depth: 9, // max_depth is 8
			counter: func(s Stats) int { return s.TooDeep },
		},
		{
			name: "deny_contains matches", url: "http://localhost:7799/login", depth: 1,
			counter: func(s Stats) int { return s.Filtered },
		},
		{
			name: "malformed", url: "http://[::1", depth: 1,
			counter: func(s Stats) int { return s.Malformed },
		},
		{
			name: "non-http scheme", url: "javascript:alert(1)", depth: 1,
			counter: func(s Stats) int { return s.Malformed },
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			f, err := New(fixturePack(t))
			if err != nil {
				t.Fatalf("New: %v", err)
			}
			before := tc.counter(f.Stats())

			if f.Push(tc.url, tc.depth) {
				t.Fatalf("Push(%q, %d) was accepted and should not have been", tc.url, tc.depth)
			}
			if got := tc.counter(f.Stats()); got != before+1 {
				t.Errorf("the counter for %s did not increment (%d -> %d)", tc.name, before, got)
			}
		})
	}
}

// Every rejection is counted. A URL that vanishes without incrementing anything
// is a crawl that cannot explain why it ended.
func TestStatsAccountForEveryPush(t *testing.T) {
	f, err := New(fixturePack(t))
	if err != nil {
		t.Fatalf("New: %v", err)
	}
	before := f.Stats()

	pushes := []struct {
		url   string
		depth int
	}{
		{"http://localhost:7799/a", 1},
		{"http://localhost:7799/a", 1}, // duplicate
		{"http://evil.example/x", 1},   // off-host
		{"http://localhost:7799/deep", 99},
		{"http://localhost:7799/login", 1},
		{"not a url at all", 1},
		{"http://localhost:7799/b", 2},
	}
	for _, p := range pushes {
		f.Push(p.url, p.depth)
	}

	s := f.Stats()
	total := (s.Accepted - before.Accepted) + (s.Duplicate - before.Duplicate) +
		(s.OffHost - before.OffHost) + (s.TooDeep - before.TooDeep) +
		(s.Filtered - before.Filtered) + (s.Malformed - before.Malformed) +
		(s.OverCapacity - before.OverCapacity)

	if total != len(pushes) {
		t.Errorf("counters account for %d pushes, but %d were made — a URL was dropped silently",
			total, len(pushes))
	}
}

// Differently-spelled URLs for one page must collapse, or the crawler fetches
// the same document repeatedly and the index stores it twice.
func TestPushDeduplicatesAcrossSpellings(t *testing.T) {
	f, err := New(fixturePack(t))
	if err != nil {
		t.Fatalf("New: %v", err)
	}

	spellings := []string{
		"http://localhost:7799/js/array-map.html",
		"http://LOCALHOST:7799/js/array-map.html",
		"http://localhost:7799/js/array-map.html#examples",
		"http://localhost:7799/js/./array-map.html",
		"http://localhost:7799/js/other/../array-map.html",
	}

	accepted := 0
	for _, u := range spellings {
		if f.Push(u, 1) {
			accepted++
		}
	}
	if accepted != 1 {
		t.Errorf("%d of %d spellings accepted, want exactly 1", accepted, len(spellings))
	}
}

// The fixture site's calendar trap: 60 pages each linking one level deeper.
// max_depth must stop it, and it must not consume the whole page budget.
func TestDepthCapStopsTheCalendarTrap(t *testing.T) {
	p := fixturePack(t)
	f, err := New(p)
	if err != nil {
		t.Fatalf("New: %v", err)
	}

	maxDepth := p.Limits.MaxDepth
	for day := range 60 {
		f.Push(fmt.Sprintf("http://localhost:7799/calendar/day-%d.html", day), day+1)
	}

	if f.Stats().TooDeep == 0 {
		t.Error("no calendar page was rejected for depth; the trap would run to 60")
	}
	for f.Len() > 0 {
		if item, _ := f.Pop(); item.Depth > maxDepth {
			t.Fatalf("popped %q at depth %d, past max_depth %d", item.URL, item.Depth, maxDepth)
		}
	}
}

func TestMaxPagesIsEnforced(t *testing.T) {
	p := fixturePack(t)
	p.Limits.MaxPages = 5

	f, err := New(p)
	if err != nil {
		t.Fatalf("New: %v", err)
	}
	for i := range 50 {
		f.Push(fmt.Sprintf("http://localhost:7799/page-%d.html", i), 1)
	}

	if got := f.Stats().Accepted; got > p.Limits.MaxPages {
		t.Errorf("accepted %d URLs, max_pages is %d", got, p.Limits.MaxPages)
	}
	if f.Stats().OverCapacity == 0 {
		t.Error("nothing was counted as over capacity, but max_pages was reached")
	}
}
