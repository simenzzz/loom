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

## Division of labor (from P1 onward)

Sami implements the core algorithms by hand. Loom is a learning project as
much as a product; an unasked implementation destroys the exercise.

**Sami writes** — anything that *is* the search engine: fetcher, robots
parser, URL canonicalization, frontier, link extraction, crawl orchestration
and its concurrency; text extraction, tokenizer, inverted index, BM25,
on-disk index codecs, query execution, snippets; every later phase's
algorithms (Bloom, SimHash, SPIMI, WAND, PageRank, HNSW, SymSpell,
LambdaMART) and the hand-rolled eval metrics.

**Claude writes** — everything that carries them: schemas, codegen and
fixtures; type/struct/signature scaffolds and doc-comment recipes; the
failing tests; config and pack loading; CLI and flag wiring; serialization
and file plumbing; HTTP handler shape and state; the React UI; the eval
harness around the metrics; compose, CI and E2E; and all docs.

### The TODO(you) contract

Claude commits **compiling** files whose core bodies are
`panic("TODO(you): …")` (Go), `todo!("TODO(you): …")` (Rust), or
`raise NotImplementedError("TODO(you): …")` (Python). Each carries a
doc-comment recipe: what the function must do, the algorithm in numbered
steps, the invariants, and the paper or spec reference. Alongside every stub
Claude commits table-driven tests that **fail** until the body is written.

Rules, non-negotiable:

- Claude must **never** fill, sketch, paraphrase, or "just show what it would
  look like" for a `TODO(you)` body unless Sami explicitly asks — **even when
  tests fail on the panics.** A red suite on a `TODO(you)` panic is the
  intended state of the loop, not a bug to fix.
- If a stub blocks Claude's own work, Claude names the blocking stub and
  stops. It may improve the recipe or tighten the test; never write the body.
- Before editing anything under `apps/crawler-go/internal/` or
  `apps/engine-rust/crates/`, run `make stubs` and check the open sites.
- If a diff would replace a `TODO(you)` body unasked, revert it.

The review gate still applies in full: `everything-claude-code:code-reviewer`
and `everything-claude-code:security-reviewer` run in parallel before any
phase is called done (root `/home/sami/CLAUDE.md`), and a phase is done only
when committed **and** CI has run green on it.

## Commands

`make help` lists everything. The important ones: `make test` (all four
languages), `make lint`, `make e2e`, `make generate-schemas`, `make eval`,
`make stubs` (list open `TODO(you)` sites).

## Per-service context

Each app has its own CLAUDE.md: `apps/crawler-go/`, `apps/engine-rust/`,
`apps/ml-python/`, `apps/web/`.
