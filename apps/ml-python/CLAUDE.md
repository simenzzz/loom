# ml-python

FastAPI sidecar: embeddings (P6), LambdaMART training (P7), LLM ops (P7).
The engine must keep answering when this service is down — degraded mode is
a tested feature, so never make the engine hard-depend on the sidecar.

Conventions:
- Pydantic v2 models generated from contracts (`src/models/gen`, do not edit)
- LLM discipline: prompts live in `src/prompts/{op}/v{n}.txt` with a status
  header; every LLM response is schema-validated with bounded retries; every
  artifact carries request_id, prompt_version, provider, model, input_hash
- LLM providers are faked in tests (recorded fixtures) — never live calls
- ruff + mypy strict (generated code excluded)
