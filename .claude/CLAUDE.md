# Loom — agent context

Vertical search engine, every core layer hand-written (no Lucene/Tantivy/
FAISS/Elastic, no crawler frameworks). Read `docs/SPEC.md` for architecture,
`docs/implementation_roadmap.md` for current phase — check tasks off there
as they complete; it is the source of truth.

## Non-negotiable rules

1. **Schema-first.** `packages/contracts/schemas/` is the source of truth.
   Never edit generated code (`*/gen/`, `engine-rust/.../generated/`). After
   a schema change: `make generate-schemas`, update the fixture corpus, and
   commit both. CI diffs generated output.
2. **Validate on both sides** of every boundary — writers validate what they
   emit, readers validate what they consume, even for the same document.
3. **No live network in tests.** The fixture site (`infra/fixture-site/`)
   and httptest fakes play the web. A test that touches the internet is a bug.
4. **No hardcoded constants** — everything routes through `.env.example`.
5. **Eval discipline.** Any change to ranking must be justified by
   `make eval-report` against the committed baseline.
6. **No off-the-shelf core algorithms.** BM25, WAND, HNSW, PageRank, SimHash,
   Bloom filters, codecs, tries, LambdaMART are implemented here, from papers.
   Library use is fine for transport/serialization (axum, serde, FastAPI).

## Commands

`make help` lists everything. The important ones: `make test` (all four
languages), `make lint`, `make e2e`, `make generate-schemas`, `make eval`.

## Per-service context

Each app has its own CLAUDE.md: `apps/crawler-go/`, `apps/engine-rust/`,
`apps/ml-python/`, `apps/web/`.
