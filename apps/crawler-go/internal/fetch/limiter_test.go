package fetch

import (
	"context"
	"errors"
	"sync"
	"testing"
	"time"
)

func TestNewHostLimiterRejectsBadRate(t *testing.T) {
	for _, rate := range []float64{0, -1} {
		if _, err := NewHostLimiter(newFakeClock(), rate, 1); err == nil {
			t.Errorf("NewHostLimiter(rate=%v) should fail — an unpaced crawler is the bug this prevents", rate)
		}
	}
}

// The first request to a host is not delayed: a full bucket means a fresh host
// is served immediately.
func TestWaitFirstRequestIsImmediate(t *testing.T) {
	clock := newFakeClock()
	l, err := NewHostLimiter(clock, 1, 1)
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}

	done := make(chan error, 1)
	go func() { done <- l.Wait(context.Background(), "fixture.test") }()

	select {
	case err := <-done:
		if err != nil {
			t.Fatalf("first Wait: %v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("first Wait blocked; a new host should start with a full bucket")
	}
}

// The flagship politeness property: over any window, a host is never asked for
// more than its budget allows. This is the test the crawler exists to satisfy.
func TestWaitNeverExceedsBudget(t *testing.T) {
	const (
		rate     = 2.0 // requests per second
		burst    = 1.0
		requests = 30
	)

	clock := newFakeClock()
	l, err := NewHostLimiter(clock, rate, burst)
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}

	var mu sync.Mutex
	var stamps []time.Time

	var wg sync.WaitGroup
	for range requests {
		wg.Add(1)
		go func() {
			defer wg.Done()
			if err := l.Wait(context.Background(), "fixture.test"); err != nil {
				return
			}
			mu.Lock()
			stamps = append(stamps, clock.Now())
			mu.Unlock()
		}()
	}

	// Drive time forward until every goroutine has been served.
	deadline := time.Now().Add(10 * time.Second)
	for {
		mu.Lock()
		served := len(stamps)
		mu.Unlock()
		if served == requests {
			break
		}
		if time.Now().After(deadline) {
			t.Fatalf("only %d/%d requests served; the limiter is stuck", served, requests)
		}
		if !clock.AdvanceToNextSleeper() {
			time.Sleep(time.Millisecond) // let blocked goroutines register
		}
	}
	wg.Wait()

	// The property: in any window, count <= rate*window + burst.
	mu.Lock()
	defer mu.Unlock()
	for i := range stamps {
		for j := i + 1; j < len(stamps); j++ {
			window := stamps[j].Sub(stamps[i]).Seconds()
			count := float64(j - i + 1)
			if allowed := rate*window + burst; count > allowed+1e-9 {
				t.Fatalf("budget exceeded: %v requests in %.3fs, allowance %.3f", count, window, allowed)
			}
		}
	}
}

// One slow host must not stall another. A limiter that shares state across
// hosts turns the crawl into a queue behind its slowest server.
func TestWaitIsPerHost(t *testing.T) {
	clock := newFakeClock()
	l, err := NewHostLimiter(clock, 1, 1)
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}

	ctx := context.Background()
	if err := l.Wait(ctx, "slow.test"); err != nil { // drains slow.test's bucket
		t.Fatalf("priming: %v", err)
	}

	done := make(chan error, 1)
	go func() { done <- l.Wait(ctx, "fast.test") }()

	select {
	case err := <-done:
		if err != nil {
			t.Fatalf("Wait on a second host: %v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("a request to fast.test blocked behind slow.test's budget")
	}
}

// A Crawl-delay longer than our budget must be honored; one shorter must not
// let us speed up.
func TestSetHostRateOnlySlowsDown(t *testing.T) {
	clock := newFakeClock()
	l, err := NewHostLimiter(clock, 10, 1)
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}
	ctx := context.Background()

	l.SetHostRate("polite.test", 1) // robots asks for 1/s; we wanted 10/s
	if err := l.Wait(ctx, "polite.test"); err != nil {
		t.Fatalf("first Wait: %v", err)
	}

	done := make(chan error, 1)
	go func() { done <- l.Wait(ctx, "polite.test") }()

	clock.Advance(200 * time.Millisecond) // enough for 10/s, not for 1/s
	select {
	case <-done:
		t.Fatal("second request served after 200ms; the 1/s Crawl-delay was ignored")
	case <-time.After(50 * time.Millisecond):
	}

	clock.Advance(900 * time.Millisecond)
	select {
	case err := <-done:
		if err != nil {
			t.Fatalf("second Wait: %v", err)
		}
	case <-time.After(time.Second):
		t.Fatal("second request never served after a full second")
	}

	l.SetHostRate("polite.test", 1000) // must not speed us back up
	if err := l.Wait(ctx, "polite.test"); err == nil {
		// Serving immediately would mean the raise took effect. Give it a
		// moment to prove it is still paced.
		go func() { _ = l.Wait(ctx, "polite.test") }()
		clock.Advance(10 * time.Millisecond)
		if clock.Sleepers() == 0 {
			t.Error("SetHostRate raised a host's rate; it must only ever lower it")
		}
	}
}

// A cancelled context returns promptly and does not consume budget it will not
// use.
func TestWaitHonorsContextCancellation(t *testing.T) {
	clock := newFakeClock()
	l, err := NewHostLimiter(clock, 1, 1)
	if err != nil {
		t.Fatalf("NewHostLimiter: %v", err)
	}

	if err := l.Wait(context.Background(), "fixture.test"); err != nil {
		t.Fatalf("priming: %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan error, 1)
	go func() { done <- l.Wait(ctx, "fixture.test") }()

	time.Sleep(10 * time.Millisecond) // let it block
	cancel()

	select {
	case err := <-done:
		if !errors.Is(err, context.Canceled) {
			t.Errorf("Wait returned %v, want context.Canceled", err)
		}
	case <-time.After(time.Second):
		t.Fatal("Wait ignored a cancelled context")
	}
}
