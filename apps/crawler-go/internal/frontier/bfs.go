// Package frontier decides what to crawl next and what never to crawl at all.
//
// P1 is breadth-first: a FIFO queue, so pages near a seed are reached before
// pages far from it. P2 replaces the queue with a heap ordered by depth,
// authority and staleness — the interface here is shaped so that swap does not
// reach into the orchestrator.
package frontier

import "loom/crawler/internal/contracts/gen"

// Item is one URL waiting to be fetched.
type Item struct {
	// URL is canonical, because the seen-set keys on it.
	URL string

	// Depth is link distance from the nearest seed. Seeds are 0.
	Depth int
}

// Stats counts every URL the frontier saw and where it went. Nothing is dropped
// silently: a URL that will not be crawled increments exactly one of these, so
// a crawl that ends early can say why.
type Stats struct {
	Accepted     int // pushed onto the queue
	Duplicate    int // already seen
	OffHost      int // host not in the pack's allowlist
	TooDeep      int // beyond max_depth
	Filtered     int // matched a deny_contains rule
	Malformed    int // would not canonicalize
	OverCapacity int // rejected because max_pages was already reached
}

// BFS is a breadth-first frontier bounded by a vertical pack's limits.
//
// Not safe for concurrent use. The orchestrator owns it from a single
// goroutine and hands URLs to workers; that keeps the ordering deterministic
// and the seen-set free of locks.
type BFS struct {
	pack  *gen.VerticalPackV1
	queue []Item
	seen  map[string]bool
	stats Stats
}

// New returns a frontier bounded by the pack's limits and host allowlist.
//
// TODO(you): implement.
//
// Steps:
//  1. Reject a nil pack — every limit this type enforces comes from it.
//  2. Initialize the queue and the seen-set.
//  3. Push every seed at depth 0. Seeds are already known to be in the
//     allowlist (pack.Load enforces that), but push them through the same Push
//     path anyway so the seen-set and the counters see them.
func New(pack *gen.VerticalPackV1) (*BFS, error) {
	panic("TODO(you): implement New — see the recipe above")
}

// Push offers a URL to the frontier, returning whether it was accepted.
//
// TODO(you): implement.
//
// Check in this order, incrementing exactly one Stats counter per rejection.
// The order matters: cheap checks first, and "seen" before the policy checks so
// a URL already rejected once is not re-counted as a policy rejection.
//
//  1. Canonicalize with urlx.Canonicalize. On error: Malformed, reject.
//  2. Already in the seen-set: Duplicate, reject.
//  3. Mark seen NOW, before the remaining checks. A URL rejected for depth must
//     not be reconsidered on every subsequent page that links to it — that is
//     how a crawler spends its whole budget re-rejecting the same trap.
//  4. Host not allowed by the pack: OffHost, reject. Use pack.HostAllowed
//     rather than comparing strings here — the loader already decided what
//     "the same host" means, and a second opinion in this package is how a
//     seed accepted at load gets dropped at depth 0.
//  5. Depth > limits.max_depth: TooDeep, reject. This is what stops the
//     fixture site's 60-page calendar trap at 8.
//  6. URL contains any url_filters.deny_contains substring: Filtered, reject.
//  7. Accepted count already at limits.max_pages: OverCapacity, reject.
//  8. Otherwise append to the queue, increment Accepted, return true.
//
// Invariants the tests assert:
//   - Accepted + every rejection counter == the number of Push calls.
//   - A URL is never accepted twice, however it is spelled.
//   - Depth never exceeds max_depth; Accepted never exceeds max_pages.
func (f *BFS) Push(rawURL string, depth int) bool {
	panic("TODO(you): implement Push — see the recipe above")
}

// Pop removes and returns the next URL, or false when the frontier is empty.
//
// TODO(you): implement. FIFO — that is what makes it breadth-first, and what
// makes the crawl reach all 19 fixture doc pages before descending the calendar
// trap. A LIFO here would spend max_pages going 8 deep into the calendar and
// never reach half the corpus.
func (f *BFS) Pop() (Item, bool) {
	panic("TODO(you): implement Pop — see the recipe above")
}

// Len reports how many URLs are waiting.
//
// TODO(you): implement.
func (f *BFS) Len() int {
	panic("TODO(you): implement Len — see the recipe above")
}

// Stats returns the counters. Copy, not pointer: callers report, they do not
// adjust.
//
// TODO(you): implement.
func (f *BFS) Stats() Stats {
	panic("TODO(you): implement Stats — see the recipe above")
}
