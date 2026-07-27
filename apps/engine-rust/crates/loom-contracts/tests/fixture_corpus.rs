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
    Contract::from_name(dir_name).unwrap_or_else(|| {
        panic!("fixture dir {dir_name} has no Contract — add it to Contract::ALL")
    })
}

/// The corpus test walks fixture dirs and maps them to contracts. Nothing
/// walked the other way, so a contract added without a fixture directory had
/// zero coverage while every suite stayed green.
#[test]
fn every_contract_has_fixtures() {
    for contract in Contract::ALL {
        for kind in ["valid", "invalid"] {
            let dir = fixtures_root().join(contract.name()).join(kind);
            assert!(
                dir.is_dir(),
                "{} has no {kind}/ fixture corpus at {dir:?}",
                contract.name()
            );
        }
    }
}

/// `validator()` indexes VALIDATORS by discriminant, so a reordered ALL would
/// silently hand back the wrong schema rather than failing to compile.
#[test]
fn all_matches_discriminant_order() {
    for (i, contract) in Contract::ALL.iter().enumerate() {
        assert_eq!(
            *contract as usize, i,
            "Contract::ALL is not discriminant-ordered"
        );
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

/// The schema layer accepting a document does not prove the generated struct
/// does. These two are the first contracts with deeply nested objects and
/// optional fields — where a typify mismatch would actually show up.
#[test]
fn nested_generated_types_roundtrip_valid_fixtures() {
    use loom_contracts::generated::{segment_manifest_v1, vertical_pack_v1};

    let root = fixtures_root();
    for name in ["minimal", "full"] {
        let raw = fs::read_to_string(root.join(format!("segment_manifest.v1/valid/{name}.json")))
            .expect("fixture readable");
        let doc: segment_manifest_v1::SegmentManifestV1 =
            serde_json::from_str(&raw).expect("generated type parses valid fixture");
        let back = serde_json::to_value(&doc).expect("serializes");
        validate(Contract::SegmentManifestV1, &back).expect("roundtrip stays contract-valid");
    }

    // minimal.json omits url_filters — the only optional object in the pack,
    // and therefore the only Option<..> branch in the generated type.
    for name in ["devdocs", "minimal", "fixture-override"] {
        let raw = fs::read_to_string(root.join(format!("vertical_pack.v1/valid/{name}.json")))
            .expect("fixture readable");
        let doc: vertical_pack_v1::VerticalPackV1 =
            serde_json::from_str(&raw).expect("generated type parses valid fixture");
        let back = serde_json::to_value(&doc).expect("serializes");
        validate(Contract::VerticalPackV1, &back).expect("roundtrip stays contract-valid");
    }
}
