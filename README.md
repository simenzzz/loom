# Loom

A vertical search engine where every core layer is hand-written — no Lucene,
no Tantivy, no FAISS, no Elastic, no crawler frameworks.

- **Go crawler**: frontier scheduler, per-host token-bucket politeness,
  Bloom-filter seen-set, SimHash near-dup detection, crash-resume
- **Rust engine**: SPIMI inverted index with varbyte compression, BM25 +
  Block-Max WAND, positional phrase queries, PageRank, from-scratch HNSW,
  SymSpell spellcheck, hand-rolled LambdaMART inference
- **Python sidecar**: embeddings, LTR training, LLM ops (query rewrite,
  summaries) behind strict schema validation — LLMs are components, never
  the product
- **React 19 web app**: search UI + live crawl dashboard

Corpus-agnostic core with **vertical packs** as data/config plugins
(`verticals/`). V1 vertical: developer documentation.

## Quickstart

```bash
cp .env.example .env
make setup     # install per-language dependencies
make dev       # boot all services + fixture site (docker compose)
make test      # all four languages' test suites
make e2e       # isolated compose E2E, zero live network
```

## Layout

| Path | What |
|------|------|
| `apps/crawler-go` | Polite crawler → segment files |
| `apps/engine-rust` | Index builder + query server |
| `apps/ml-python` | Embeddings / LTR / LLM ops sidecar |
| `apps/web` | Search UI + dashboard |
| `packages/contracts` | JSON Schemas (source of truth) + 4-language codegen |
| `verticals/` | Vertical packs (seeds, crawl policy, extractor hints, ranking weights) |
| `eval/` | Golden queries, NDCG/MRR harness, benchmarks |
| `infra/` | Compose files, fixture site, ops scripts |
| `docs/` | SPEC, roadmap (source of truth), storage formats, ADRs |

## House rules

1. Schemas in `packages/contracts/schemas/` are the source of truth; run
   `make generate-schemas` after any change and commit the generated code
   (CI enforces drift).
2. Every service boundary validates on **both** sides.
3. Tests never touch the live network — the fixture site plays the web.
4. Every ranking change must move a number in `eval/results/`.
