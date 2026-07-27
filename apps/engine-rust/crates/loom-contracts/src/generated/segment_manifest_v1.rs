#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "Describes one crawl segment directory: the compressed CrawlRecord file beside it, its integrity hash, and the counters needed to plan an index build. Written by the Go crawler, read by the Rust indexer. Both sides validate."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://loom.dev/schemas/segment_manifest.v1.schema.json\","]
#[doc = "  \"title\": \"SegmentManifestV1\","]
#[doc = "  \"description\": \"Describes one crawl segment directory: the compressed CrawlRecord file beside it, its integrity hash, and the counters needed to plan an index build. Written by the Go crawler, read by the Rust indexer. Both sides validate.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"bytes_compressed\","]
#[doc = "    \"bytes_uncompressed\","]
#[doc = "    \"crawler_version\","]
#[doc = "    \"created_at\","]
#[doc = "    \"pages_file\","]
#[doc = "    \"pages_sha256\","]
#[doc = "    \"record_count\","]
#[doc = "    \"schema\","]
#[doc = "    \"segment_id\","]
#[doc = "    \"vertical\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"bytes_compressed\": {"]
#[doc = "      \"description\": \"Size of the pages file on disk, for reporting and build planning. The manifest is untrusted input to the indexer (house rule 2), so this is never an allocation size; the reader streams and clamps to its own configured ceiling.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 68719476736.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"bytes_uncompressed\": {"]
#[doc = "      \"description\": \"Total size of the JSONL stream before compression, for reporting and build planning. Same rule as bytes_compressed: a hint, never an allocation size — a manifest can claim any size it likes while the pages file beside it is empty.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 68719476736.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"crawler_version\": {"]
#[doc = "      \"description\": \"Version of the crawler that produced the segment, for provenance when a build looks wrong\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"created_at\": {"]
#[doc = "      \"description\": \"UTC timestamp at which the segment was sealed\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"first_fetched_at\": {"]
#[doc = "      \"description\": \"Earliest fetched_at across the segment's records; absent when the segment is empty\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"last_fetched_at\": {"]
#[doc = "      \"description\": \"Latest fetched_at across the segment's records; absent when the segment is empty\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"pages_file\": {"]
#[doc = "      \"description\": \"Name of the compressed CrawlRecord file in this segment directory. A bare filename by construction: the reader joins it to the segment directory, so path separators and traversal are forbidden by the pattern.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"not\": {"]
#[doc = "        \"pattern\": \"[\\\\n\\\\r]\""]
#[doc = "      },"]
#[doc = "      \"maxLength\": 255,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[A-Za-z0-9][A-Za-z0-9._-]*$\""]
#[doc = "    },"]
#[doc = "    \"pages_sha256\": {"]
#[doc = "      \"description\": \"SHA-256 of the pages file as written, hex-encoded lowercase. Detects truncation and corruption; it is NOT an authenticity control, since anyone who can rewrite the pages file can rewrite this hash beside it. Verify single-pass — feed the decompressor and the digest from one reader and commit nothing downstream until the digest matches at EOF. Hashing the file, closing it, then reopening to parse is a TOCTOU.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"not\": {"]
#[doc = "        \"pattern\": \"[\\\\n\\\\r]\""]
#[doc = "      },"]
#[doc = "      \"pattern\": \"^[0-9a-f]{64}$\""]
#[doc = "    },"]
#[doc = "    \"record_count\": {"]
#[doc = "      \"description\": \"Number of CrawlRecord documents in the pages file. A reported count, not a trusted one: the reader counts what it actually decodes and treats a mismatch as a corrupt segment. Never pre-allocate from this value.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 100000000.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"description\": \"Contract discriminator, always segment_manifest.v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"segment_manifest.v1\""]
#[doc = "    },"]
#[doc = "    \"segment_id\": {"]
#[doc = "      \"description\": \"Identifier of this segment, unique within the vertical and sortable by creation order\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[a-z0-9][a-z0-9-]*$\""]
#[doc = "    },"]
#[doc = "    \"vertical\": {"]
#[doc = "      \"description\": \"Vertical pack id this segment was crawled under\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false,"]
#[doc = "  \"dependentRequired\": {"]
#[doc = "    \"first_fetched_at\": ["]
#[doc = "      \"last_fetched_at\""]
#[doc = "    ],"]
#[doc = "    \"last_fetched_at\": ["]
#[doc = "      \"first_fetched_at\""]
#[doc = "    ]"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SegmentManifestV1 {
    #[doc = "Size of the pages file on disk, for reporting and build planning. The manifest is untrusted input to the indexer (house rule 2), so this is never an allocation size; the reader streams and clamps to its own configured ceiling."]
    pub bytes_compressed: i64,
    #[doc = "Total size of the JSONL stream before compression, for reporting and build planning. Same rule as bytes_compressed: a hint, never an allocation size — a manifest can claim any size it likes while the pages file beside it is empty."]
    pub bytes_uncompressed: i64,
    #[doc = "Version of the crawler that produced the segment, for provenance when a build looks wrong"]
    pub crawler_version: SegmentManifestV1CrawlerVersion,
    #[doc = "UTC timestamp at which the segment was sealed"]
    pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Earliest fetched_at across the segment's records; absent when the segment is empty"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub first_fetched_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Latest fetched_at across the segment's records; absent when the segment is empty"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub last_fetched_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
    pub pages_file: SegmentManifestV1PagesFile,
    pub pages_sha256: SegmentManifestV1PagesSha256,
    #[doc = "Number of CrawlRecord documents in the pages file. A reported count, not a trusted one: the reader counts what it actually decodes and treats a mismatch as a corrupt segment. Never pre-allocate from this value."]
    pub record_count: i64,
    #[doc = "Contract discriminator, always segment_manifest.v1"]
    pub schema: ::std::string::String,
    #[doc = "Identifier of this segment, unique within the vertical and sortable by creation order"]
    pub segment_id: SegmentManifestV1SegmentId,
    #[doc = "Vertical pack id this segment was crawled under"]
    pub vertical: SegmentManifestV1Vertical,
}
impl SegmentManifestV1 {
    pub fn builder() -> builder::SegmentManifestV1 {
        Default::default()
    }
}
#[doc = "Version of the crawler that produced the segment, for provenance when a build looks wrong"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Version of the crawler that produced the segment, for provenance when a build looks wrong\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SegmentManifestV1CrawlerVersion(::std::string::String);
impl ::std::ops::Deref for SegmentManifestV1CrawlerVersion {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SegmentManifestV1CrawlerVersion> for ::std::string::String {
    fn from(value: SegmentManifestV1CrawlerVersion) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SegmentManifestV1CrawlerVersion {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SegmentManifestV1CrawlerVersion {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SegmentManifestV1CrawlerVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SegmentManifestV1CrawlerVersion {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SegmentManifestV1CrawlerVersion {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Name of the compressed CrawlRecord file in this segment directory. A bare filename by construction: the reader joins it to the segment directory, so path separators and traversal are forbidden by the pattern."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Name of the compressed CrawlRecord file in this segment directory. A bare filename by construction: the reader joins it to the segment directory, so path separators and traversal are forbidden by the pattern.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"not\": {"]
#[doc = "    \"pattern\": \"[\\\\n\\\\r]\""]
#[doc = "  },"]
#[doc = "  \"maxLength\": 255,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[A-Za-z0-9][A-Za-z0-9._-]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SegmentManifestV1PagesFile(::std::string::String);
impl ::std::ops::Deref for SegmentManifestV1PagesFile {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SegmentManifestV1PagesFile> for ::std::string::String {
    fn from(value: SegmentManifestV1PagesFile) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SegmentManifestV1PagesFile {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[A-Za-z0-9][A-Za-z0-9._-]*$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z0-9][A-Za-z0-9._-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SegmentManifestV1PagesFile {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SegmentManifestV1PagesFile {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SegmentManifestV1PagesFile {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SegmentManifestV1PagesFile {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "SHA-256 of the pages file as written, hex-encoded lowercase. Detects truncation and corruption; it is NOT an authenticity control, since anyone who can rewrite the pages file can rewrite this hash beside it. Verify single-pass — feed the decompressor and the digest from one reader and commit nothing downstream until the digest matches at EOF. Hashing the file, closing it, then reopening to parse is a TOCTOU."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"SHA-256 of the pages file as written, hex-encoded lowercase. Detects truncation and corruption; it is NOT an authenticity control, since anyone who can rewrite the pages file can rewrite this hash beside it. Verify single-pass — feed the decompressor and the digest from one reader and commit nothing downstream until the digest matches at EOF. Hashing the file, closing it, then reopening to parse is a TOCTOU.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"not\": {"]
#[doc = "    \"pattern\": \"[\\\\n\\\\r]\""]
#[doc = "  },"]
#[doc = "  \"pattern\": \"^[0-9a-f]{64}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SegmentManifestV1PagesSha256(::std::string::String);
impl ::std::ops::Deref for SegmentManifestV1PagesSha256 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SegmentManifestV1PagesSha256> for ::std::string::String {
    fn from(value: SegmentManifestV1PagesSha256) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SegmentManifestV1PagesSha256 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9a-f]{64}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{64}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SegmentManifestV1PagesSha256 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SegmentManifestV1PagesSha256 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SegmentManifestV1PagesSha256 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SegmentManifestV1PagesSha256 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Identifier of this segment, unique within the vertical and sortable by creation order"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Identifier of this segment, unique within the vertical and sortable by creation order\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-z0-9][a-z0-9-]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SegmentManifestV1SegmentId(::std::string::String);
impl ::std::ops::Deref for SegmentManifestV1SegmentId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SegmentManifestV1SegmentId> for ::std::string::String {
    fn from(value: SegmentManifestV1SegmentId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SegmentManifestV1SegmentId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-z0-9][a-z0-9-]*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9][a-z0-9-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SegmentManifestV1SegmentId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SegmentManifestV1SegmentId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SegmentManifestV1SegmentId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SegmentManifestV1SegmentId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Vertical pack id this segment was crawled under"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Vertical pack id this segment was crawled under\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SegmentManifestV1Vertical(::std::string::String);
impl ::std::ops::Deref for SegmentManifestV1Vertical {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SegmentManifestV1Vertical> for ::std::string::String {
    fn from(value: SegmentManifestV1Vertical) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SegmentManifestV1Vertical {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[a-z0-9][a-z0-9_-]*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[a-z0-9][a-z0-9_-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SegmentManifestV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SegmentManifestV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SegmentManifestV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SegmentManifestV1Vertical {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct SegmentManifestV1 {
        bytes_compressed: ::std::result::Result<i64, ::std::string::String>,
        bytes_uncompressed: ::std::result::Result<i64, ::std::string::String>,
        crawler_version:
            ::std::result::Result<super::SegmentManifestV1CrawlerVersion, ::std::string::String>,
        created_at:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        first_fetched_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        last_fetched_at: ::std::result::Result<
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
            ::std::string::String,
        >,
        pages_file: ::std::result::Result<super::SegmentManifestV1PagesFile, ::std::string::String>,
        pages_sha256:
            ::std::result::Result<super::SegmentManifestV1PagesSha256, ::std::string::String>,
        record_count: ::std::result::Result<i64, ::std::string::String>,
        schema: ::std::result::Result<::std::string::String, ::std::string::String>,
        segment_id: ::std::result::Result<super::SegmentManifestV1SegmentId, ::std::string::String>,
        vertical: ::std::result::Result<super::SegmentManifestV1Vertical, ::std::string::String>,
    }
    impl ::std::default::Default for SegmentManifestV1 {
        fn default() -> Self {
            Self {
                bytes_compressed: Err("no value supplied for bytes_compressed".to_string()),
                bytes_uncompressed: Err("no value supplied for bytes_uncompressed".to_string()),
                crawler_version: Err("no value supplied for crawler_version".to_string()),
                created_at: Err("no value supplied for created_at".to_string()),
                first_fetched_at: Ok(Default::default()),
                last_fetched_at: Ok(Default::default()),
                pages_file: Err("no value supplied for pages_file".to_string()),
                pages_sha256: Err("no value supplied for pages_sha256".to_string()),
                record_count: Err("no value supplied for record_count".to_string()),
                schema: Err("no value supplied for schema".to_string()),
                segment_id: Err("no value supplied for segment_id".to_string()),
                vertical: Err("no value supplied for vertical".to_string()),
            }
        }
    }
    impl SegmentManifestV1 {
        pub fn bytes_compressed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes_compressed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes_compressed: {e}"));
            self
        }
        pub fn bytes_uncompressed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.bytes_uncompressed = value.try_into().map_err(|e| {
                format!("error converting supplied value for bytes_uncompressed: {e}")
            });
            self
        }
        pub fn crawler_version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SegmentManifestV1CrawlerVersion>,
            T::Error: ::std::fmt::Display,
        {
            self.crawler_version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for crawler_version: {e}"));
            self
        }
        pub fn created_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_at: {e}"));
            self
        }
        pub fn first_fetched_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.first_fetched_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for first_fetched_at: {e}"));
            self
        }
        pub fn last_fetched_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.last_fetched_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_fetched_at: {e}"));
            self
        }
        pub fn pages_file<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SegmentManifestV1PagesFile>,
            T::Error: ::std::fmt::Display,
        {
            self.pages_file = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pages_file: {e}"));
            self
        }
        pub fn pages_sha256<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SegmentManifestV1PagesSha256>,
            T::Error: ::std::fmt::Display,
        {
            self.pages_sha256 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pages_sha256: {e}"));
            self
        }
        pub fn record_count<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.record_count = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for record_count: {e}"));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn segment_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SegmentManifestV1SegmentId>,
            T::Error: ::std::fmt::Display,
        {
            self.segment_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segment_id: {e}"));
            self
        }
        pub fn vertical<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SegmentManifestV1Vertical>,
            T::Error: ::std::fmt::Display,
        {
            self.vertical = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vertical: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SegmentManifestV1> for super::SegmentManifestV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SegmentManifestV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                bytes_compressed: value.bytes_compressed?,
                bytes_uncompressed: value.bytes_uncompressed?,
                crawler_version: value.crawler_version?,
                created_at: value.created_at?,
                first_fetched_at: value.first_fetched_at?,
                last_fetched_at: value.last_fetched_at?,
                pages_file: value.pages_file?,
                pages_sha256: value.pages_sha256?,
                record_count: value.record_count?,
                schema: value.schema?,
                segment_id: value.segment_id?,
                vertical: value.vertical?,
            })
        }
    }
    impl ::std::convert::From<super::SegmentManifestV1> for SegmentManifestV1 {
        fn from(value: super::SegmentManifestV1) -> Self {
            Self {
                bytes_compressed: Ok(value.bytes_compressed),
                bytes_uncompressed: Ok(value.bytes_uncompressed),
                crawler_version: Ok(value.crawler_version),
                created_at: Ok(value.created_at),
                first_fetched_at: Ok(value.first_fetched_at),
                last_fetched_at: Ok(value.last_fetched_at),
                pages_file: Ok(value.pages_file),
                pages_sha256: Ok(value.pages_sha256),
                record_count: Ok(value.record_count),
                schema: Ok(value.schema),
                segment_id: Ok(value.segment_id),
                vertical: Ok(value.vertical),
            }
        }
    }
}
