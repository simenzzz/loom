package fetch

import (
	"context"
	"errors"
	"net/http"
)

// Sentinel errors classifying every fetch failure. Nothing is dropped silently:
// a URL that failed is counted under one of these, and the caller decides
// whether to retry it or record it as done.
var (
	// ErrRetryable is a failure that may succeed later: a timeout, a connection
	// reset, a 429, a 5xx.
	ErrRetryable = errors.New("fetch: retryable")

	// ErrFatal is a failure that will not: a 4xx other than 429, a body that is
	// not HTML, a redirect loop, a URL that will not canonicalize.
	ErrFatal = errors.New("fetch: fatal")

	// ErrRobotsDisallowed is a URL robots.txt forbids. Separate from ErrFatal
	// because it is not a failure at all — it is the system working, and the
	// counter it increments is one an operator wants to see.
	ErrRobotsDisallowed = errors.New("fetch: disallowed by robots.txt")
)

// Response is one fetched page.
type Response struct {
	// FinalURL is the canonical URL after redirects. It may differ from the
	// requested URL, and it is the one that becomes document identity.
	FinalURL string

	// StatusCode of the final response.
	StatusCode int

	// ContentType with parameters stripped, e.g. "text/html" from
	// "text/html; charset=utf-8".
	ContentType string

	// Body, capped at MaxBodyBytes. See Truncated.
	Body []byte

	// Truncated reports that the body hit the cap and was cut. A truncated page
	// is still indexable, but it must never be treated as a complete document
	// for near-duplicate detection in P2.
	Truncated bool

	// ETag and LastModified as sent, for conditional revalidation in P2. Empty
	// when absent.
	ETag         string
	LastModified string
}

// Fetcher performs one polite HTTP GET.
//
// It owns politeness, size limits and redirect handling. It does not own the
// frontier, the seen-set or robots.txt retrieval — those are the caller's, and
// keeping them out is what makes this testable against an httptest server.
type Fetcher struct {
	// Client is the HTTP client. Tests supply one pointed at httptest; nothing
	// in this package ever reaches the live network.
	Client *http.Client

	// UserAgent is sent on every request. It identifies the crawler and carries
	// a contact URL, per RFC 9309 §2.1.
	UserAgent string

	// MaxBodyBytes caps a response body. A crawler that trusts Content-Length
	// is a crawler one malicious server can OOM.
	MaxBodyBytes int64

	// Limiter paces requests per host.
	Limiter *HostLimiter

	// Clock is the time source, so tests never sleep.
	Clock Clock

	// MaxRedirects bounds a redirect chain before it is called a loop.
	MaxRedirects int
}

// Fetch retrieves rawURL, honoring the per-host budget and the size cap.
//
// TODO(you): implement.
//
// Algorithm:
//
//  1. Wait on the limiter for the URL's host. Do this BEFORE the request, and
//     before any retry — a retry is another request to the same server and
//     spends budget like any other.
//  2. Build the request with the context so cancellation propagates, and set
//     User-Agent and Accept-Encoding.
//  3. Follow redirects manually rather than letting http.Client do it: the
//     client would not consult the limiter between hops, so a chain of five
//     redirects becomes five unpaced requests. Cap at MaxRedirects and return
//     ErrFatal beyond it. Track visited URLs to name a loop as a loop — the
//     fixture site has /loop/x <-> /loop/y for exactly this.
//  4. Classify the status:
//     - 2xx: proceed.
//     - 3xx with a Location: another hop (step 3).
//     - 429 and 5xx: ErrRetryable. Honor Retry-After when present, via the
//     clock, and cap it — a hostile server can send Retry-After: 86400.
//     - other 4xx: ErrFatal.
//  5. Check Content-Type before reading the body. Anything that is not
//     text/html or application/xhtml+xml is ErrFatal: this is a page crawler,
//     and a 700 MB video is not a parse failure but a resource attack.
//  6. Read the body through an io.LimitReader at MaxBodyBytes+1 so the cap can
//     be detected rather than inferred. If it read MaxBodyBytes+1, set
//     Truncated and keep the first MaxBodyBytes.
//  7. Canonicalize the final URL with urlx.Canonicalize and populate Response.
//
// Invariants the tests assert:
//   - Every network attempt is preceded by a Limiter.Wait for that host.
//   - The body never exceeds MaxBodyBytes regardless of what the server sends
//     or what Content-Length claims.
//   - A redirect loop terminates and reports a loop, not a timeout.
//   - Every error wraps exactly one of the sentinels above, so errors.Is
//     classifies it.
//   - The response body is always closed, on every path including errors.
func (f *Fetcher) Fetch(ctx context.Context, rawURL string) (*Response, error) {
	panic("TODO(you): implement Fetch — see the recipe above")
}

// FetchRobots retrieves a host's robots.txt.
//
// TODO(you): implement.
//
// It is separate from Fetch because the rules differ (RFC 9309 §2.3.1.3):
//
//   - 2xx: parse the body.
//   - 404 or any other 4xx: no robots.txt exists, everything is allowed.
//     Return nil rules and a nil error.
//   - 429 or 5xx: the site is unwell. Treat as "everything disallowed" and
//     return ErrRetryable — do NOT fall back to allowing everything, which is
//     the tempting bug that turns a bad afternoon for a site into a crawl of a
//     site that told us to go away.
//   - Unreachable / timeout: same as 5xx.
//
// Cap the body hard: robots.txt is text, and RFC 9309 §2.5 requires honoring at
// least 500 KiB. Beyond that, parse what was read rather than failing.
//
// This still goes through the limiter — robots.txt is a request like any other.
func (f *Fetcher) FetchRobots(ctx context.Context, host string) ([]byte, error) {
	panic("TODO(you): implement FetchRobots — see the recipe above")
}
