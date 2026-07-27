# Loom — Flagship Plan: a vertical search engine built from scratch

## Context

Sami wants a new flagship project that exercises his full skill breadth (Go concurrency,
Rust systems work, Python ML, React/TS, contract-first polyglot monorepos) and is
algorithm-heavy rather than CRUD. After a brainstorm of 7 candidates, he chose **Loom**:
a vertical search engine where every core layer is hand-written — no Lucene/Tantivy/
FAISS/Elastic, no crawler frameworks. Style follows `learningproj` (Lucubrum): LLMs as
schema-validated components (never the product), contracts package validated on both
sides of every boundary, golden-data eval harness, phased roadmap-as-source-of-truth,
managed-service deploys, rich Makefile.

**Corpus decision**: corpus-agnostic core + **vertical packs** as data/config plugins.
V1 = developer docs (MDN, Rust/Go/Python official docs). Post-v1 packs: arXiv CS,
recipes (schema.org), news. UI grows vertical tabs.

New repo at `/home/sami/loom`.

## Architecture

Four services + contracts:

- **`apps/crawler-go`** (Go): frontier scheduler (heap by depth/authority/staleness,
  host-sharded), per-host token-bucket politeness + robots.txt, URL canonicalization,
  hand-rolled Bloom filter seen-set + SQLite crawl ledger, sitemap parsing, 64-bit
  SimHash near-dup detection with banded LSH, crash-resume, WS event stream for the
  dashboard. Output: filesystem segments (`pages.jsonl.zst` + manifest) of validated
  CrawlRecords.
- **`apps/engine-rust`** (cargo workspace, tideway conventions): crates
  `loom-textproc` (text-density boilerplate removal, tokenizer, Porter stemmer, trigram
  lang detect), `loom-postings` (delta+varbyte codecs, positional postings, skip
  pointers, block-max metadata, front-coded term dictionary), `loom-index` (SPIMI build
  + external k-way merge, zstd docstore, manifest + atomic `CURRENT` swap),
  `loom-query` (boolean/phrase parser, BM25, Block-Max WAND, max-window snippets, LRU
  cache), `loom-graph` (PageRank power iteration, anchor-text field), `loom-vector`
  (HNSW from scratch, SIMD dot products), `loom-spell` (SymSpell + Levenshtein
  automaton, weighted trie autocomplete), `loom-rank` (features, RRF fusion, GBDT
  inference), `loom-server` (axum: /search /suggest /autocomplete /admin/reload),
  `bins/loom-indexer` (offline segments → index build). `fuzz/` cargo-fuzz targets.
- **`apps/ml-python`** (FastAPI/Pydantic sidecar): `/embed` (MiniLM-class 384-dim),
  LambdaMART training from scratch (regression trees + lambda gradients), LLM ops =
  query rewrite + result summarization only, versioned prompt registry
  (`src/prompts/{op}/v{n}.txt`) with A/B experiment logs. **Optional at runtime**:
  engine degrades to BM25-only with `degraded: true` (enforced by E2E test).
- **`apps/web`** (React 19 + TS + Vite): search page (suggestions, did-you-mean,
  snippets, AI-summary card), live crawl/index dashboard over WS.
- **`packages/contracts`**: hand-authored **JSON Schema 2020-12 as source of truth**
  (inverts Lucubrum's Pydantic-first since types cross Go/Rust/Python/TS). Codegen:
  typify (Rust), go-jsonschema (Go), datamodel-code-generator (Pydantic),
  json-schema-to-typescript (TS). Shared `fixtures/valid|invalid` corpus executed by
  all four languages' test suites; CI codegen-drift gate (`git diff --exit-code`).
  Schemas: crawl_record, segment_manifest, search_request/response, suggest,
  autocomplete, embed, rewrite, summary, ltr_model, crawl_event, vertical_pack.
- **`verticals/{devdocs,arxiv,recipes,news}/`**: `pack.toml` (id, ranking weights),
  `seeds.txt`, `policy.toml` (hosts, depth, max_pages, rates), `extract.toml`.
- **Data flow**: crawler → filesystem segments → offline indexer → versioned build dir
  + atomic `CURRENT` pointer swap (zero-downtime reload/rollback). Metadata in SQLite
  (modernc in Go, rusqlite in Rust) — no managed DB in v1; Supabase is the documented
  click-log upgrade path.
- **`infra/`**: docker-compose.yml + docker-compose.test.yml, **fixture-site** (nginx
  serving a frozen ~500-page devdocs snapshot with robots.txt, sitemap, redirect chains,
  a crawler trap) so tests never touch the live network.
- **`eval/`**: `data/devdocs/{queries.v1.jsonl,qrels.v1.jsonl}` (graded 0–3, url-keyed),
  hand-rolled `metrics.py` (NDCG@10, MRR, Recall@50) unit-tested vs worked examples,
  `run.py`, `baselines/` regression gate (CI fails on >2% NDCG drop), `bench/`
  (latency p50/p99, index size, politeness audit), append-only `results/` jsonl with
  git sha. Make targets: eval, eval-smoke, eval-report, eval-baseline, bench.
- **`docs/`**: SPEC.md, API.md, STORAGE_FORMATS.md (byte-level format docs),
  implementation_roadmap.md (living source of truth with checkboxes), ADRs
  (~6: contracts-first-JSON-Schema, filesystem boundary, SQLite, Fly topology, etc).
- `.claude/` root CLAUDE.md + per-app CLAUDE.md + skills (crawler-skill, engine-skill,
  contracts-skill).

## Phases (v1 = P0–P8; each ends runnable, CI green, eval line recorded)

- **P0 — Scaffold, contracts pipeline, CI (~1wk)**: repo tree, Makefile, first schemas
  + 4-language codegen + fixture suite, compose files, fixture-site snapshot, CI
  (go -race / cargo clippy -D warnings / pytest / vitest / drift gate).
  *Exit*: `make dev` boots everything; contract fixtures pass in all 4 languages.
- **P1 — Thin slice (~1.5wk)**: minimal polite BFS crawl of fixture site → naive
  extraction → in-memory inverted index with correct BM25 → v0 flat index file →
  axum /search → React search box. 10-query smoke eval.
  *Exit*: "array map" returns the fixture site's synthetic Array.prototype.map() page
  top-3 (the corpus is generated, deliberately not MDN content); full loop runs in
  compose E2E with zero live network. Demoable search engine over the ~32-page fixture
  corpus — see the roadmap for the reachable-set breakdown and the Step 10 decision on
  growing it.
- **P2 — Real crawler + eval v1 + dashboard (~2wk)**: frontier heap, token buckets,
  Bloom filter + ledger, full canonicalization, sitemaps, SimHash+LSH dedup, resume,
  WS dashboard; eval harness v1 + 60 graded queries.
  *Exit*: politeness property test (recorded timestamps never exceed per-host budget),
  trap page doesn't wedge, kill -9 → resume without refetch, committed eval baseline.
- **P3 — Real index (~3wk, the heart)**: text-density extraction, Porter (validated
  against official vectors), SPIMI + external merge, delta+varbyte + skip pointers +
  block-max metadata, positional phrase queries, zstd docstore, atomic CURRENT swap,
  boolean/phrase/field parser, max-window snippets, STORAGE_FORMATS.md.
  *Exit*: proptest SPIMI == naive oracle; codec roundtrip proptests; decoder/parser
  fuzz 1hr clean; 10k real MDN pages within peak-RSS budget; NDCG ≥ P1 baseline.
- **P4 — Query performance (~2wk)**: WAND → Block-Max WAND, LRU cache, SymSpell +
  Levenshtein automaton (A/B in eval), weighted-trie autocomplete, bench harness.
  *Exit*: proptest BMW top-k == exhaustive top-k; p99 < 50ms on 10k docs; typo'd
  golden queries recover via suggestion.
- **P5 — Link analysis (~1.5wk)**: link graph, PageRank (d=0.85, dangling handling,
  closed-form unit tests + sums-to-1 property), anchor-text field, BM25F field weights
  from pack.toml, linear score combination (LTR placeholder).
  *Exit*: NDCG improvement on navigational queries vs P4, recorded; weight changes
  need no code change.
- **P6 — Vectors + hybrid (~2wk)**: sidecar /embed, batch embedding at build, HNSW
  from scratch, SIMD cosine (portable simd, scalar fallback), RRF fusion (k=60),
  tested degradation path.
  *Exit*: HNSW recall@10 ≥ 0.95 vs brute-force oracle; SIMD == scalar (tolerance);
  hybrid ≥ BM25 on golden set, strictly better on tagged semantic queries; sidecar
  killed mid-E2E → still answers, `degraded: true`.
- **P7 — LTR + LLM ops (~2.5wk)**: feature logging, LambdaMART from scratch (Python
  training, Rust tree-walk inference over top-100), 5-fold CV guardrail (ship only if
  beats linear baseline), LLM query rewrite (≤3 rewrites fused via RRF) + summarization
  (schema forbids uncited URLs), prompt registry + A/B harness.
  *Exit*: Rust inference == Python on 1k vectors; deterministic training under seed;
  works with LLM_OPS_ENABLED=false; malformed-LLM-output fixtures → bounded retry.
- **P8 — Deploy + polish (~1.5wk)**: Fly.io single machine + 3GB volume (loom-server
  always-on; weekly crawl/index scheduled job; atomic swap = zero-downtime refresh),
  Render free tier for ml-python (cold start = degraded mode, already tested), Vercel
  web, rate limiting/CORS/service tokens, real 50k-page devdocs crawl with politeness
  audit, README with demo GIF + NDCG progression table (BM25 → +PageRank → +hybrid →
  +LTR), ADRs finalized.
  *Exit*: public URL; `git clone && make setup && make demo` works clean via fixture
  site.

**Post-v1**: P9 arXiv pack (proof of corpus-agnostic claim — exit criterion: no core
code changes, gaps get ADRs); P10 click-based LTR (position-bias correction,
team-draft interleaving); P11 recipes/news packs + freshness-driven recrawl + faceting.

## Testing strategy

- **Rust**: proptests as core guarantees (codec roundtrips, SPIMI==oracle,
  BMW==exhaustive, HNSW recall, PageRank invariants); cargo-fuzz on all decoders +
  query parser + extractor (total decoders, never panic); Porter golden vectors;
  criterion benches.
- **Go**: httptest fake web (robots variants, redirects, 429/503, traps, size bombs);
  politeness timestamp property test; injected clock (no sleeps); -race always; Bloom
  FPR statistical test; canonicalization golden corpus; go fuzz on canonicalizer +
  robots parser; crash/resume test.
- **Python**: faked LLM providers (recorded fixtures), retry-on-invalid tests, LTR
  determinism + cross-language parity fixtures.
- **Contracts**: shared valid/invalid fixture corpus run by all four languages.
- **E2E** (compose test env): fixture-site → crawl → index → search → Playwright UI
  journey + degradation E2E. Eval-as-test: CI fails on >2% NDCG regression.

## Deployment

Vercel (web) · Fly.io shared-cpu-1x/1GB + 3GB volume (Rust server + scheduled
crawl/index job co-located; single-attach volume) · Render free (ml-python) · SQLite
on volume. Index budget: 50k pages ≈ 150–400MB postings+docstore + ~120MB vectors —
fits with headroom; `policy.toml max_pages` enforces the cap. Compose mirrors prod
images exactly.

## Risks & scope cuts (cut from the top)

1. LLM summarization (garnish) 2. Levenshtein automaton (keep SymSpell) 3. LambdaMART
→ coordinate-ascent linear LTR 4. HNSW/hybrid → ship BM25-only (degradation path is
the cut) 5. BMW → plain WAND/MaxScore (block-max metadata already on disk) 6. lang
detect → `en` stub 7. 50k → 10k pages 8. WS dashboard → polling.
**Never cut**: contracts pipeline, eval harness, politeness tests, fixture-site
determinism, atomic index swap.

Cross-cutting: MDN snapshot is CC-BY-SA (attribute, keep small); conservative crawl
rates + politeness audit before any long real crawl; `data/` gitignored with a
disk-report script (WSL vhdx); if Fly free tier drifts, the compose file already is a
$5-VPS deploy (ADR).

## Verification (end-to-end)

1. `make setup && make dev` → all services + fixture site up.
2. `make crawl-fixture && make index` → query "array map" in the UI returns the right
   page; dashboard streamed the crawl live.
3. `make test` (all four languages, -race/clippy/fuzz-smoke) and `make e2e` green.
4. `make eval-report` → NDCG@10/MRR vs committed baseline; each ranking phase must
   move the number.
5. P8: hit the public URL; kill ml-python → search still answers with degraded flag.

## First implementation step

P0 scaffold: repo init at /home/sami/loom, Makefile, contracts + codegen for
crawl_record.v1 and search_response.v1, compose skeleton, fixture-site snapshot script,
CI workflow.
