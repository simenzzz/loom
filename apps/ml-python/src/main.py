"""Loom ML sidecar entrypoint.

P0 exposes liveness only. P6 adds /embed, P7 adds /rewrite, /summarize and
LTR training endpoints — every request/response validated against
packages/contracts schemas on both sides of the boundary.
"""

from fastapi import FastAPI

app = FastAPI(title="loom-ml", version="0.1.0")


@app.get("/healthz")
def healthz() -> dict[str, str]:
    return {"service": "ml-python", "status": "ok"}
