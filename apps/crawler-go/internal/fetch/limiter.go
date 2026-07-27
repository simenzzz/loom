package fetch

import (
	"context"
	"sync"
	"time"
)

// HostLimiter paces requests so no host is asked for more than its budget.
//
// Politeness is enforced here, not hoped for. The flagship test records every
// request timestamp and asserts no host's budget was ever exceeded, so this
// type is what stands between the crawler and someone else's server.
//
// One bucket per host: a slow host must not delay requests to a fast one, and a
// fast host must not spend a slow host's budget.
type HostLimiter struct {
	clock Clock

	mu      sync.Mutex
	buckets map[string]*bucket
	rate    float64 // default requests per second per host
	burst   float64 // maximum tokens a host may accumulate
}

// bucket is one host's token state.
type bucket struct {
	tokens float64
	last   time.Time
	rate   float64 // this host's rate, which robots Crawl-delay may lower
}

// NewHostLimiter returns a limiter allowing rate requests per second per host,
// with the given burst ceiling.
//
// TODO(you): implement.
//
// Steps:
//  1. Reject a non-positive rate — an unpaced crawler is the bug this type
//     exists to prevent, and a zero here would silently produce one.
//  2. Default burst to 1 when it is below 1: a burst under one token means no
//     request can ever proceed.
//  3. Initialize the bucket map and store the clock.
func NewHostLimiter(clock Clock, rate, burst float64) (*HostLimiter, error) {
	panic("TODO(you): implement NewHostLimiter — see the recipe above")
}

// SetHostRate lowers a single host's rate, for when its robots.txt asks for a
// longer Crawl-delay than our own budget.
//
// TODO(you): implement.
//
// It may only ever make a host SLOWER. A robots.txt asking for one request
// every 10 seconds must get 10 seconds even though policy.toml would permit one
// per second; a robots.txt asking for a delay shorter than our budget does not
// entitle us to speed up. Take the minimum of the current and proposed rate.
func (l *HostLimiter) SetHostRate(host string, rate float64) {
	panic("TODO(you): implement SetHostRate — see the recipe above")
}

// Wait blocks until host's budget permits one request, or ctx is cancelled.
//
// TODO(you): implement.
//
// The algorithm is a token bucket (the standard formulation; see Tanenbaum,
// Computer Networks, §5.4.2 — a leaky bucket that admits bursts):
//
//  1. Lock. Look up or create the host's bucket, starting it full so the first
//     request to a new host is not delayed.
//  2. Refill by elapsed time: tokens += (now - last) * rate, capped at burst.
//     Set last = now. This is what makes it a bucket rather than a fixed
//     window — an idle host accumulates credit up to the cap and no further.
//  3. If tokens >= 1, spend one and return nil without sleeping.
//  4. Otherwise compute the wait for the next whole token: (1 - tokens) / rate.
//     Spend the token NOW (drive tokens to zero and advance last past the
//     wait), then unlock BEFORE sleeping.
//  5. Sleep via the clock, returning ctx.Err() if it cancels.
//
// The ordering in steps 4 and 5 is the whole difficulty:
//
//   - Reserve before sleeping, or two goroutines both see one token, both
//     sleep, and both fire — a budget of one per second serving two requests.
//   - Never hold the mutex across the sleep, or one host's delay blocks every
//     other host and the crawler serializes.
//
// Invariants the tests assert:
//   - Over any window, requests to one host never exceed rate*window + burst.
//   - Two hosts never block each other.
//   - Concurrent callers for the same host are paced correctly (run under
//     -race; the recorded timestamps must still satisfy the budget).
//   - A cancelled context returns promptly with ctx.Err() and does not consume
//     a token it will not use.
func (l *HostLimiter) Wait(ctx context.Context, host string) error {
	panic("TODO(you): implement Wait — see the recipe above")
}
