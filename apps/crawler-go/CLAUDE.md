# crawler-go

Polite crawler: vertical pack in, validated CrawlRecord segment files out
(`data/crawl/<vertical>/segments/`). The filesystem is the boundary with the
indexer — no RPC.

Conventions:
- Go 1.25, stdlib-first; `-race` always; no sleeps in tests (inject clocks)
- All web interaction in tests goes through `httptest` fakes
- Politeness is enforced, not hoped: per-host token buckets; the flagship
  test asserts recorded fetch timestamps never exceed any host budget
- Errors: classify retryable vs fatal; never silently drop a URL — count it
- Validate every CrawlRecord with `internal/contracts` before writing
