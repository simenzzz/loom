// loomcrawl is the Loom crawler CLI.
//
//	loomcrawl serve   – run the status API (and, from P2, the dashboard WS feed)
//	loomcrawl crawl   – crawl a vertical pack into segment files (lands in P1)
package main

import (
	"context"
	"errors"
	"fmt"
	"log/slog"
	"net/http"
	"os"
	"os/signal"
	"syscall"
	"time"

	"loom/crawler/internal/dash"
)

const shutdownTimeout = 5 * time.Second

func main() {
	log := slog.New(slog.NewJSONHandler(os.Stderr, nil))
	if err := run(log, os.Args[1:]); err != nil {
		log.Error("fatal", "err", err)
		os.Exit(1)
	}
}

func run(log *slog.Logger, args []string) error {
	cmd := "serve"
	if len(args) > 0 {
		cmd = args[0]
	}
	switch cmd {
	case "serve":
		return serve(log)
	case "crawl":
		return errors.New("crawl lands in P1 — run `loomcrawl serve` for now")
	default:
		return fmt.Errorf("unknown command %q (expected serve|crawl)", cmd)
	}
}

func serve(log *slog.Logger) error {
	port := os.Getenv("LOOM_CRAWLER_PORT")
	if port == "" {
		port = "7710"
	}
	srv := &http.Server{
		Addr:              ":" + port,
		Handler:           dash.New(log).Handler(),
		ReadHeaderTimeout: 10 * time.Second,
	}

	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGINT, syscall.SIGTERM)
	defer stop()

	errCh := make(chan error, 1)
	go func() { errCh <- srv.ListenAndServe() }()
	log.Info("crawler status API listening", "port", port)

	select {
	case err := <-errCh:
		return fmt.Errorf("status API: %w", err)
	case <-ctx.Done():
	}

	shutdownCtx, cancel := context.WithTimeout(context.Background(), shutdownTimeout)
	defer cancel()
	if err := srv.Shutdown(shutdownCtx); err != nil {
		return fmt.Errorf("shutting down status API: %w", err)
	}
	log.Info("crawler stopped cleanly")
	return nil
}
