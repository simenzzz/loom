// Package crawl runs a crawl: frontier in, validated CrawlRecords out.
//
// This is where the pieces meet, and where the concurrency lives. Everything it
// depends on arrives through Deps as an interface, so the whole orchestration
// is testable against an httptest fake web with no live network and no sleeps.
package crawl

import (
	"context"
	"net/url"
	"time"

	"loom/crawler/internal/contracts/gen"
	"loom/crawler/internal/fetch"
	"loom/crawler/internal/robots"
)

// Fetcher is the subset of *fetch.Fetcher the orchestrator uses.
type Fetcher interface {
	Fetch(ctx context.Context, rawURL string) (*fetch.Response, error)
	FetchRobots(ctx context.Context, host string) ([]byte, error)
}

// RecordSink receives each fetched page. The segment writer implements it;
// tests collect into a slice.
//
// The sink validates every record against crawl_record.v1 before it is durable
// — writers validate what they emit, readers validate what they consume.
type RecordSink interface {
	Write(record *gen.CrawlRecordV1) error
}

// Deps are the orchestrator's collaborators.
type Deps struct {
	Fetcher Fetcher
	Sink    RecordSink
	Clock   fetch.Clock

	// Limiter is shared with the Fetcher so robots Crawl-delay can lower a
	// host's rate for every later request to it.
	Limiter *fetch.HostLimiter

	// UserAgent is the token matched against robots.txt groups. It is the
	// product token only ("LoomBot"), not the full User-Agent header — RFC 9309
	// §2.2.1 matches on the product token.
	UserAgent string
}

// Stats is what a finished crawl reports.
type Stats struct {
	Fetched      int // pages successfully fetched and written
	RobotsDenied int // URLs robots.txt forbade
	Retryable    int // failed with ErrRetryable
	Fatal        int // failed with ErrFatal
	Elapsed      time.Duration
}

// Run crawls the pack's seeds until the frontier empties or ctx is cancelled.
//
// TODO(you): implement.
//
// Algorithm:
//
//  1. Build a frontier from the pack (frontier.New). It seeds itself.
//  2. Set up a robots cache keyed by host. Fetch a host's robots.txt at most
//     once per crawl: it is a request like any other, and refetching it per URL
//     would spend most of the budget on it.
//     - On success, parse with robots.Parse using deps.UserAgent, and if the
//     group states a Crawl-delay, call Limiter.SetHostRate so every later
//     request to that host is paced by it.
//     - On ErrRetryable, treat the whole host as disallowed for this crawl.
//     Do NOT fall back to allowing everything.
//     - Skip all of this when the pack sets respect_robots false.
//  3. Run workers bounded by LOOM_CRAWLER_GLOBAL_CONCURRENCY. Use
//     golang.org/x/sync/errgroup with SetLimit, or a worker pool over a
//     channel — either is fine, but the bound must be real: an unbounded
//     goroutine per URL is a fork bomb aimed at someone else's server.
//  4. Each worker loop:
//     a. Pop a URL. The frontier is NOT safe for concurrent use, so pops and
//     pushes must be serialized — either keep the frontier on one goroutine
//     and hand work out over a channel, or guard it with a mutex. Pick one
//     and say which in a comment; mixing them is how the race gets in.
//     b. Check robots for the URL's host. Denied: count RobotsDenied, continue.
//     c. Fetch. Classify the error with errors.Is:
//     - ErrRetryable: count and continue (P2 adds real retry scheduling).
//     - ErrFatal: count and continue.
//     - nil: proceed.
//     d. Extract links from the body with extract.Links, using the response's
//     FinalURL as the base — not the requested URL, or every link on a
//     redirected page resolves against the wrong origin.
//     e. Push each link at depth+1.
//     f. Build a gen.CrawlRecordV1 and hand it to the sink.
//  5. Stop when the frontier is empty AND every worker is idle. This is the
//     subtle part: an empty frontier does not mean the crawl is done, because
//     an in-flight fetch may be about to push more URLs. Track outstanding work
//     explicitly — a WaitGroup, or a counter of in-flight fetches — and only
//     terminate when the queue is empty and that count is zero. Terminating on
//     "queue empty" alone ends the crawl after the first page.
//  6. Honor ctx throughout. Cancellation must stop the crawl promptly and
//     return what was gathered so far, not discard it.
//
// Building the record (every field is required by crawl_record.v1 unless noted):
//   - schema:        "crawl_record.v1"
//   - vertical:      pack.Pack.Id
//   - url:           the response's FinalURL (post-redirect)
//   - canonical_url: the canonical form of FinalURL
//   - fetched_at:    deps.Clock.Now() in UTC, RFC 3339
//   - status_code, content_type, depth
//   - html:          the response body as a string
//   - links:         what extract.Links returned
//   - etag, last_modified: when the response carried them (optional)
//   - simhash64:     absent in P1; P2 fills it
//
// Invariants the tests assert:
//   - No URL disallowed by robots.txt is ever fetched. The fixture site's
//     /private/secret.html appearing in any record means politeness is broken.
//   - Every record validates against crawl_record.v1.
//   - Depth never exceeds the pack's max_depth.
//   - Fetched + RobotsDenied + Retryable + Fatal accounts for every URL popped.
//   - Runs clean under -race.
//   - No goroutine outlives Run.
func Run(ctx context.Context, pack *gen.VerticalPackV1, deps Deps) (Stats, error) {
	panic("TODO(you): implement Run — see the recipe above")
}

// robotsFor fetches and parses a host's robots.txt, caching the result.
//
// TODO(you): implement. Split out from Run because Run is long enough without
// it, and because the caching rule ("at most once per host per crawl") is
// easier to see when it is the only thing a function does.
//
// Returns nil rules when the pack sets respect_robots false or the host has no
// robots.txt — nil means "no restrictions", and Allowed must not be called on
// it.
func robotsFor(ctx context.Context, host string, deps Deps, cache map[string]*robots.Rules) (*robots.Rules, error) {
	panic("TODO(you): implement robotsFor — see the recipe above")
}

// baseURL parses a response's final URL for use as the link-resolution base.
//
// TODO(you): implement. Trivial, but it is the seam where using the requested
// URL instead of the final one would silently resolve every link on a
// redirected page against the wrong origin.
func baseURL(finalURL string) (*url.URL, error) {
	panic("TODO(you): implement baseURL — see the recipe above")
}
