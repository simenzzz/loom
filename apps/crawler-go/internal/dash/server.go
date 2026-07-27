// Package dash serves the crawler's status API. In P2 it grows the WebSocket
// event stream that feeds the web dashboard; in P0 it only proves liveness.
package dash

import (
	"encoding/json"
	"log/slog"
	"net/http"
	"time"
)

// Server exposes crawler observability endpoints.
type Server struct {
	log     *slog.Logger
	started time.Time
}

// New returns a Server logging through log.
func New(log *slog.Logger) *Server {
	return &Server{log: log, started: time.Now().UTC()}
}

// Handler returns the HTTP routes for the status API.
func (s *Server) Handler() http.Handler {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /healthz", s.healthz)
	return mux
}

func (s *Server) healthz(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Content-Type", "application/json")
	resp := map[string]any{
		"service":  "crawler-go",
		"status":   "ok",
		"uptime_s": int(time.Since(s.started).Seconds()),
	}
	if err := json.NewEncoder(w).Encode(resp); err != nil {
		s.log.Error("writing healthz response", "err", err)
	}
}
