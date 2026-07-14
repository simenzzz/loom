# Loom — Implementation Roadmap (source of truth)

Update this file as tasks complete (`[ ]` → `[x]`). Every phase ends
runnable, committed, CI green, with an eval/bench line recorded once the
harness exists. Full architecture rationale lives in [SPEC.md](SPEC.md).

## P0 — Scaffold, contracts pipeline, CI

Exit criteria: `make dev` boots all services + fixture site; the shared
contract fixture corpus passes in all four languages; CI green; drift gate
proven.

- [x] Repo tree, Makefile, `.env.example`, `.gitignore`
- [x] Schemas: `crawl_record.v1`, `search_request.v1`, `search_response.v1`
- [x] Fixture corpus (`packages/contracts/fixtures/*/valid|invalid`)
- [x] 4-language codegen (`packages/contracts/codegen/generate.sh`):
      typify (Rust), go-jsonschema (Go), datamodel-code-generator (Pydantic),
      json-schema-to-typescript (TS) + verbatim schema copies for runtime
      validation
- [x] crawler-go skeleton: `loomcrawl serve`, `/healthz`, embedded-schema
      validation (`internal/contracts`), fixture-corpus test (`-race`)
- [x] engine-rust workspace: `loom-contracts` (validators + generated types),
      `loom-server` (axum `/healthz`, contract-validated `/search` stub),
      `loom-indexer` stub bin; clippy `-D warnings` + fmt clean
- [x] ml-python: FastAPI `/healthz`, fixture-corpus test via jsonschema +
      generated Pydantic models
- [x] web: Vite + React 19, AJV boundary validation (`src/lib/validate.ts`),
      fixture-corpus vitest suite, search page skeleton
- [x] Fixture site: deterministic synthetic devdocs generator (robots.txt,
      sitemap, redirect chain + loop, depth trap, near-duplicate pages,
      disallowed section) + nginx image
- [x] `verticals/devdocs` pack (pack/policy/extract/seeds)
- [x] docker-compose.yml + docker-compose.test.yml + P0 smoke E2E
- [x] CI: per-language jobs + contracts drift gate + compose E2E
- [ ] First commit pushed; CI observed green on GitHub

## P1 — Thin slice: crawl 100 pages → index → search box

Exit criteria: `make crawl-fixture && make index` then "array map" returns
the fixture `js/array-map.html` page top-3; smoke eval NDCG recorded; full
loop runs in compose E2E with zero live network.

- [ ] Vertical pack loader (`internal/pack`) reading pack/policy/extract TOML
- [ ] Minimal polite fetcher: robots.txt honor, 1 rps/host, BFS, depth limit
- [ ] Segment writer emitting validated CrawlRecords (`pages.jsonl.zst` + manifest)
- [ ] Naive extraction (tag-strip + title) & whitespace tokenizer (Rust)
- [ ] In-memory inverted index + correct BM25 (k1=1.2, b=0.75)
- [ ] `loom-indexer`: segments → v0 flat index file; `loom-server` loads it
- [ ] `/search` over the real index + fixed-window snippets
- [ ] Search page renders live results with latency badge
- [ ] `eval/run.py` smoke harness + 10 golden queries/qrels
- [ ] Replace P0 smoke E2E with full crawl→index→search E2E

## P2 — Real crawler + eval harness v1 + dashboard

- [ ] Frontier heap (depth/authority/staleness), host-sharded queues
- [ ] Per-host token buckets + robots crawl-delay; errgroup concurrency cap
- [ ] Hand-rolled Bloom filter (10M urls @1% FPR) + SQLite ledger
- [ ] Full URL canonicalization (golden corpus test)
- [ ] Sitemap discovery/parsing
- [ ] SimHash (64-bit, 4-shingles) + 4-band LSH dedup
- [ ] `loomcrawl resume` from ledger; staleness-driven revisit scheduling
- [ ] WS `crawl_event.v1` stream + dashboard page
- [ ] Eval harness v1: hand-rolled NDCG@10/MRR/Recall@50, baselines, results jsonl
- [ ] 60 graded golden queries
- [ ] Politeness property test (timestamps never exceed budget); trap test;
      kill -9 resume test

## P3 — Real index (the heart)

- [ ] Text-density boilerplate removal + pack extract hints; code-block field
- [ ] Porter stemmer vs official vectors; trigram language detection
- [ ] Varbyte+delta codecs, 128-doc blocks, skip pointers, block-max metadata
- [ ] Positional postings (title/body/code fields)
- [ ] Front-coded term dictionary
- [ ] SPIMI bounded-memory build + external k-way merge
- [ ] zstd docstore with offset table
- [ ] Build manifest + atomic CURRENT swap + `/admin/reload`
- [ ] Query parser: boolean, "phrase" (positional), field: prefix
- [ ] Max-window snippet generation with highlighting
- [ ] STORAGE_FORMATS.md byte-level docs
- [ ] Proptests (SPIMI == naive oracle; codec roundtrips); fuzz targets 1 hr clean

## P4 — Query performance

- [ ] WAND → Block-Max WAND (proptest: == exhaustive top-k)
- [ ] LRU result cache
- [ ] SymSpell + Levenshtein automaton (A/B in eval)
- [ ] Weighted trie autocomplete + /autocomplete
- [ ] Suggestions UI + did-you-mean
- [ ] Bench harness (p50/p99, index size); p99 < 50ms @ 10k docs

## P5 — Link analysis

- [ ] Link graph from CrawlRecords (canonical-resolved)
- [ ] PageRank power iteration (closed-form unit tests, sums-to-1 property)
- [ ] Anchor-text field
- [ ] BM25F field weighting from pack.toml + recency feature
- [ ] Linear score combination; eval delta recorded

## P6 — Vectors + hybrid

- [ ] Sidecar /embed (MiniLM 384-dim) + service-token auth
- [ ] Batch embedding at index build; mmap-friendly vectors file
- [ ] HNSW from scratch (recall@10 ≥ 0.95 vs brute force)
- [ ] SIMD cosine (portable, scalar fallback, equivalence proptest)
- [ ] RRF hybrid fusion (k=60)
- [ ] Degradation path E2E (`degraded: true`)

## P7 — LTR + LLM ops

- [ ] Feature logging → training jsonl (`make ltr-dump`)
- [ ] LambdaMART from scratch (Python train, Rust inference, parity fixtures)
- [ ] 5-fold CV guardrail vs linear baseline
- [ ] LLM query rewrite (≤3 rewrites, RRF-fused) + summarize (cited URLs only)
- [ ] Prompt registry lifecycle + A/B experiment logs
- [ ] `LLM_OPS_ENABLED=false` path tested

## P8 — Deploy + polish (v1 finish line)

- [ ] Fly.io app (server + weekly crawl/index job, 3GB volume)
- [ ] Render: ml-python; Vercel: web
- [ ] Rate limiting, CORS allowlist, service tokens (fail closed at startup
      if `LOOM_ML_SERVICE_TOKEN` is empty or still the template placeholder)
- [ ] Real 50k-page devdocs crawl + politeness audit
- [ ] README demo GIF + NDCG progression table; ADRs finalized

## Post-v1

- P9 arXiv vertical pack (proof: no core code changes)
- P10 click-based LTR (position-bias correction, interleaving)
- P11 recipes + news packs, freshness recrawl, faceting
