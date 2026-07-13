"""Runs the shared cross-language contract fixture corpus.

The same fixture files are executed by the Go, Rust, and TS suites; a schema
change without fixture updates fails all four suites at once.
"""

import json
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator

REPO_ROOT = Path(__file__).resolve().parents[3]
SCHEMAS = REPO_ROOT / "packages" / "contracts" / "schemas"
FIXTURES = REPO_ROOT / "packages" / "contracts" / "fixtures"


def _validator(schema_name: str) -> Draft202012Validator:
    schema_path = SCHEMAS / f"{schema_name}.schema.json"
    schema = json.loads(schema_path.read_text())
    return Draft202012Validator(schema)


def _cases() -> list[tuple[str, str, Path]]:
    cases = []
    for schema_dir in sorted(FIXTURES.iterdir()):
        if not schema_dir.is_dir():
            continue
        for kind in ("valid", "invalid"):
            files = sorted((schema_dir / kind).glob("*.json"))
            assert files, f"{schema_dir.name}: {kind}/ is empty"
            cases.extend((schema_dir.name, kind, f) for f in files)
    assert cases, "fixture corpus is empty"
    return cases


@pytest.mark.parametrize(
    ("schema_name", "kind", "path"),
    _cases(),
    ids=lambda v: v.name if isinstance(v, Path) else str(v),
)
def test_shared_fixture_corpus(schema_name: str, kind: str, path: Path) -> None:
    validator = _validator(schema_name)
    doc = json.loads(path.read_text())
    errors = list(validator.iter_errors(doc))
    if kind == "valid":
        assert not errors, f"{path.name} expected valid, got: {errors[0].message}"
    else:
        assert errors, f"{path.name} expected validation failure, got none"


def test_generated_pydantic_model_parses_valid_fixture() -> None:
    """The generated typed layer must accept what the schema layer accepts."""
    from src.models.gen.crawl_record_v1_schema import CrawlRecordV1

    raw = json.loads((FIXTURES / "crawl_record.v1" / "valid" / "full.json").read_text())
    record = CrawlRecordV1.model_validate(raw)
    assert record.schema_ == "crawl_record.v1"
    assert record.simhash64 is not None


def test_generated_pydantic_model_rejects_unknown_field() -> None:
    from pydantic import ValidationError

    from src.models.gen.crawl_record_v1_schema import CrawlRecordV1

    raw = json.loads(
        (FIXTURES / "crawl_record.v1" / "invalid" / "unknown-field.json").read_text()
    )
    with pytest.raises(ValidationError):
        CrawlRecordV1.model_validate(raw)
