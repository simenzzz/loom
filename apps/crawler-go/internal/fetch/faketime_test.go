package fetch

import (
	"context"
	"sort"
	"sync"
	"time"
)

// fakeClock is a manually-advanced clock.
//
// Sleepers do not resume when wall-clock time passes; they resume when a test
// calls Advance. That is what lets a one-request-per-second budget be proven
// over sixty requests in microseconds, and it makes the result deterministic
// instead of load-dependent.
type fakeClock struct {
	mu       sync.Mutex
	now      time.Time
	sleepers []*sleeper
}

type sleeper struct {
	until time.Time
	ch    chan struct{}
}

func newFakeClock() *fakeClock {
	return &fakeClock{now: time.Date(2026, 7, 28, 12, 0, 0, 0, time.UTC)}
}

func (c *fakeClock) Now() time.Time {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.now
}

func (c *fakeClock) Sleep(ctx context.Context, d time.Duration) error {
	if d <= 0 {
		return ctx.Err()
	}
	c.mu.Lock()
	s := &sleeper{until: c.now.Add(d), ch: make(chan struct{})}
	c.sleepers = append(c.sleepers, s)
	c.mu.Unlock()

	select {
	case <-s.ch:
		return nil
	case <-ctx.Done():
		return ctx.Err()
	}
}

// Advance moves time forward and wakes every sleeper whose deadline has passed.
func (c *fakeClock) Advance(d time.Duration) {
	c.mu.Lock()
	c.now = c.now.Add(d)
	var still []*sleeper
	for _, s := range c.sleepers {
		if !s.until.After(c.now) {
			close(s.ch)
			continue
		}
		still = append(still, s)
	}
	c.sleepers = still
	c.mu.Unlock()
}

// AdvanceToNextSleeper jumps to the earliest pending deadline and wakes it.
//
// This is how a test drives a limiter without guessing intervals: run until
// everything is blocked, then release the next one. Returns false when nobody
// is sleeping.
func (c *fakeClock) AdvanceToNextSleeper() bool {
	c.mu.Lock()
	if len(c.sleepers) == 0 {
		c.mu.Unlock()
		return false
	}
	sort.Slice(c.sleepers, func(i, j int) bool {
		return c.sleepers[i].until.Before(c.sleepers[j].until)
	})
	target := c.sleepers[0].until
	c.mu.Unlock()

	c.Advance(target.Sub(c.Now()))
	return true
}

// Sleepers reports how many goroutines are currently blocked in Sleep.
func (c *fakeClock) Sleepers() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return len(c.sleepers)
}
