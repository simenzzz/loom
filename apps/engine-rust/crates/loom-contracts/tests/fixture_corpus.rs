//! Runs the shared cross-language fixture corpus. The same files are executed
//! by the Go, Python, and TS suites — a schema change without fixture updates
//! fails everywhere at once.

use std::fs;
use std::path::PathBuf;

use loom_contracts::{Contract, validate};

fn fixtures_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../packages/contracts/fixtures")
}

fn contract_for(dir_name: &str) -> Contract {
    match dir_name {
        "crawl_record.v1" => Contract::CrawlRecordV1,
        "search_request.v1" => Contract::SearchRequestV1,
        "search_response.v1" => Contract::SearchResponseV1,
        other => panic!("fixture dir {other} has no Contract mapping — add it to contract_for"),
    }
}

#[test]
fn shared_fixture_corpus() {
    let root = fixtures_root();
    let schema_dirs: Vec<_> = fs::read_dir(&root)
        .expect("fixture corpus root exists")
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .collect();
    assert!(!schema_dirs.is_empty(), "fixture corpus is empty");

    for schema_dir in schema_dirs {
        let contract = contract_for(&schema_dir.file_name().to_string_lossy());
        for kind in ["valid", "invalid"] {
            let dir = schema_dir.path().join(kind);
            let files: Vec<_> = fs::read_dir(&dir)
                .unwrap_or_else(|_| panic!("{} has no {kind}/ fixtures", contract.name()))
                .filter_map(Result::ok)
                .collect();
            assert!(!files.is_empty(), "{}: {kind}/ is empty", contract.name());

            for file in files {
                let raw = fs::read_to_string(file.path()).expect("fixture readable");
                let doc: serde_json::Value = serde_json::from_str(&raw)
                    .unwrap_or_else(|e| panic!("{:?} is not JSON: {e}", file.path()));
                let result = validate(contract, &doc);
                match kind {
                    "valid" => assert!(
                        result.is_ok(),
                        "{:?} expected valid, got: {}",
                        file.path(),
                        result.expect_err("checked is_ok above")
                    ),
                    _ => assert!(
                        result.is_err(),
                        "{:?} expected validation failure, got Ok",
                        file.path()
                    ),
                }
            }
        }
    }
}

#[test]
fn generated_types_roundtrip_valid_fixtures() {
    // The typed layer must accept what the schema layer accepts.
    let root = fixtures_root();
    let raw =
        fs::read_to_string(root.join("crawl_record.v1/valid/full.json")).expect("fixture readable");
    let record: loom_contracts::generated::crawl_record_v1::CrawlRecordV1 =
        serde_json::from_str(&raw).expect("generated type parses valid fixture");
    let back = serde_json::to_value(&record).expect("serializes");
    validate(Contract::CrawlRecordV1, &back).expect("roundtrip stays contract-valid");
}
