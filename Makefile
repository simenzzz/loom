# ============================================================
# Loom — unified entrypoint. `make help` lists everything.
# ============================================================

SHELL := /bin/bash
.DEFAULT_GOAL := help

COMPOSE      := docker compose -f infra/docker-compose.yml
COMPOSE_TEST := docker compose -f infra/docker-compose.test.yml

# ------------------------------------------------------------
# Meta
# ------------------------------------------------------------
.PHONY: help
help: ## Show this help
	@grep -hE '^[a-zA-Z][a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-24s\033[0m %s\n", $$1, $$2}'

.PHONY: setup
setup: setup-codegen ## Install all per-language dependencies + codegen toolchain
	cd apps/crawler-go && go mod download
	cd apps/engine-rust && cargo fetch
	cd apps/ml-python && python3 -m venv .venv && .venv/bin/pip install -e ".[dev]"
	cd apps/web && npm ci

.PHONY: setup-codegen
setup-codegen: ## Install the pinned contracts codegen toolchain (versions.env)
	cd packages/contracts/codegen && npm ci
	set -a && . packages/contracts/codegen/versions.env && set +a && \
		go install $$GO_JSONSCHEMA_MODULE@$$GO_JSONSCHEMA_VERSION && \
		(cargo typify --version 2>/dev/null | grep -q "$$CARGO_TYPIFY_VERSION" || \
			cargo install cargo-typify --locked --version $$CARGO_TYPIFY_VERSION) && \
		python3 -m venv packages/contracts/codegen/.venv && \
		packages/contracts/codegen/.venv/bin/pip install -q "datamodel-code-generator==$$DATAMODEL_CODEGEN_VERSION"

# ------------------------------------------------------------
# Contracts
# ------------------------------------------------------------
.PHONY: generate-schemas
generate-schemas: ## Regenerate contract types for all four languages
	packages/contracts/codegen/generate.sh

.PHONY: check-schemas
check-schemas: generate-schemas ## Fail if generated contract code drifted from schemas
	git diff --exit-code -- \
		apps/crawler-go/internal/contracts/gen \
		apps/engine-rust/crates/loom-contracts/src/generated \
		apps/ml-python/src/models/gen \
		apps/web/src/contracts/gen

# ------------------------------------------------------------
# Dev
# ------------------------------------------------------------
.PHONY: dev
dev: ## Run all services + fixture site via docker compose
	$(COMPOSE) up --build

.PHONY: dev-down
dev-down: ## Stop the dev stack
	$(COMPOSE) down

.PHONY: dev-web
dev-web: ## Run the web app against a locally running engine
	cd apps/web && npm run dev

# ------------------------------------------------------------
# Test / lint
# ------------------------------------------------------------
.PHONY: test
test: test-go test-rust test-python test-web ## Run every language's test suite

.PHONY: test-go
test-go: ## Go tests (race detector always on)
	cd apps/crawler-go && go test -race ./...

.PHONY: test-rust
test-rust: ## Rust tests
	cd apps/engine-rust && cargo test --workspace

.PHONY: test-python
test-python: ## Python tests
	cd apps/ml-python && .venv/bin/python -m pytest

.PHONY: test-web
test-web: ## Web tests
	cd apps/web && npm test -- --run

.PHONY: stubs
stubs: ## List open TODO(you) sites (Sami implements these — see .claude/CLAUDE.md)
	@grep -rn "TODO(you)" apps eval 2>/dev/null \
		--include=*.go --include=*.rs --include=*.py --include=*.ts --include=*.tsx \
		|| echo "no open stubs"

.PHONY: lint
lint: ## Lint all languages
	cd apps/crawler-go && go vet ./...
	cd apps/engine-rust && cargo clippy --workspace --all-targets -- -D warnings && cargo fmt --check
	cd apps/ml-python && .venv/bin/ruff check src tests && .venv/bin/mypy src
	cd apps/web && npm run lint

.PHONY: e2e
e2e: ## Full docker-compose E2E (fixture site -> crawl -> index -> search)
	$(COMPOSE_TEST) up --build --abort-on-container-exit --exit-code-from e2e
	$(COMPOSE_TEST) down -v

# ------------------------------------------------------------
# Pipeline (P1+)
# ------------------------------------------------------------
.PHONY: crawl-fixture
crawl-fixture: ## Crawl the local fixture site into data/crawl/devdocs
	cd apps/crawler-go && go run ./cmd/loomcrawl crawl --vertical ../../verticals/devdocs --fixture

.PHONY: index
index: ## Build index from crawl segments
	cd apps/engine-rust && cargo run --bin loom-indexer -- --vertical devdocs

# ------------------------------------------------------------
# Eval / bench (harness lands in P1/P2)
# ------------------------------------------------------------
.PHONY: eval
eval: ## Full eval run against golden queries
	python3 eval/run.py --queries eval/data/devdocs/queries.v1.jsonl --qrels eval/data/devdocs/qrels.v1.jsonl

.PHONY: eval-smoke
eval-smoke: ## 10-query smoke eval (CI gate)
	python3 eval/run.py --smoke --queries eval/data/devdocs/queries.v1.jsonl --qrels eval/data/devdocs/qrels.v1.jsonl

.PHONY: eval-report
eval-report: ## Diff latest eval run vs committed baseline
	python3 eval/run.py --report --baseline eval/baselines/devdocs.json

.PHONY: eval-baseline
eval-baseline: ## Promote the latest eval run to the committed baseline
	python3 eval/run.py --promote --baseline eval/baselines/devdocs.json

# ------------------------------------------------------------
# Fixture site
# ------------------------------------------------------------
.PHONY: fixture-site
fixture-site: ## Regenerate the deterministic synthetic fixture site
	python3 infra/fixture-site/generate.py --out infra/fixture-site/site
