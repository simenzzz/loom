//! Contract types generated from `packages/contracts/schemas` plus runtime
//! validation. Every service boundary validates on both sides (house rule),
//! so readers of segment files and HTTP handlers all funnel through
//! [`validate`].

pub mod generated;

use std::sync::OnceLock;

use jsonschema::Validator;
use serde_json::Value;

/// A contract known to this build. The schema JSON is embedded verbatim by
/// codegen, so the binary needs no filesystem access to validate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Contract {
    CrawlRecordV1,
    SearchRequestV1,
    SearchResponseV1,
    SegmentManifestV1,
    VerticalPackV1,
}

impl Contract {
    /// Every contract known to this build, in discriminant order.
    ///
    /// This is the single list a new contract is added to; [`Contract::validator`]
    /// indexes into a parallel array using the discriminant, and the
    /// `all_matches_discriminant_order` test pins the two together.
    pub const ALL: &'static [Contract] = &[
        Contract::CrawlRecordV1,
        Contract::SearchRequestV1,
        Contract::SearchResponseV1,
        Contract::SegmentManifestV1,
        Contract::VerticalPackV1,
    ];

    /// Resolves a `schema` discriminator string to its contract.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|c| c.name() == name)
    }

    /// Contract name as it appears in `schema` discriminator fields.
    pub fn name(self) -> &'static str {
        match self {
            Contract::CrawlRecordV1 => "crawl_record.v1",
            Contract::SearchRequestV1 => "search_request.v1",
            Contract::SearchResponseV1 => "search_response.v1",
            Contract::SegmentManifestV1 => "segment_manifest.v1",
            Contract::VerticalPackV1 => "vertical_pack.v1",
        }
    }

    fn raw_schema(self) -> &'static str {
        match self {
            Contract::CrawlRecordV1 => {
                include_str!("generated/schemas/crawl_record.v1.schema.json")
            }
            Contract::SearchRequestV1 => {
                include_str!("generated/schemas/search_request.v1.schema.json")
            }
            Contract::SearchResponseV1 => {
                include_str!("generated/schemas/search_response.v1.schema.json")
            }
            Contract::SegmentManifestV1 => {
                include_str!("generated/schemas/segment_manifest.v1.schema.json")
            }
            Contract::VerticalPackV1 => {
                include_str!("generated/schemas/vertical_pack.v1.schema.json")
            }
        }
    }

    fn validator(self) -> &'static Validator {
        static VALIDATORS: [OnceLock<Validator>; Contract::ALL.len()] =
            [const { OnceLock::new() }; Contract::ALL.len()];
        // Sound because Contract is fieldless and ALL is discriminant-ordered
        // (pinned by all_matches_discriminant_order).
        let slot = &VALIDATORS[self as usize];
        slot.get_or_init(|| {
            let schema: Value = serde_json::from_str(self.raw_schema())
                .expect("embedded schema is valid JSON (checked by codegen)");
            // format is assertion, not annotation — all four languages must
            // agree on what the corpus accepts.
            jsonschema::options()
                .should_validate_formats(true)
                .build(&schema)
                .expect("embedded schema compiles (checked by contract tests)")
        })
    }
}

/// Validation failure for a document crossing a contract boundary.
#[derive(Debug, thiserror::Error)]
#[error("contract {contract}: {detail}")]
pub struct ContractViolation {
    contract: &'static str,
    detail: String,
}

/// Validates a JSON document against the named contract.
pub fn validate(contract: Contract, doc: &Value) -> Result<(), ContractViolation> {
    contract
        .validator()
        .validate(doc)
        .map_err(|e| ContractViolation {
            contract: contract.name(),
            detail: e.to_string(),
        })
}
