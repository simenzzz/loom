package fetch

import (
	"context"
	"time"
)

// Clock is the crawler's only source of time.
//
// Everything that waits takes one of these so tests can drive time directly. A
// politeness test that slept would take a real minute to prove a one-per-second
// budget over sixty requests, which means in practice it would never be written.
type Clock interface {
	// Now returns the current time.
	Now() time.Time

	// Sleep blocks for d, or until ctx is cancelled, whichever comes first. It
	// returns ctx.Err() on cancellation and nil when d elapsed.
	Sleep(ctx context.Context, d time.Duration) error
}

// SystemClock is the real clock, used everywhere outside tests.
type SystemClock struct{}

// Now returns the wall-clock time.
func (SystemClock) Now() time.Time { return time.Now() }

// Sleep waits for d or ctx, whichever fires first.
func (SystemClock) Sleep(ctx context.Context, d time.Duration) error {
	if d <= 0 {
		return ctx.Err()
	}
	timer := time.NewTimer(d)
	defer timer.Stop()

	select {
	case <-timer.C:
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}
