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
#[doc = "One fetched page, written by the Go crawler into segment files (pages.jsonl.zst) and read by the Rust indexer. Both sides validate."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://loom.dev/schemas/crawl_record.v1.schema.json\","]
#[doc = "  \"title\": \"CrawlRecordV1\","]
#[doc = "  \"description\": \"One fetched page, written by the Go crawler into segment files (pages.jsonl.zst) and read by the Rust indexer. Both sides validate.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"canonical_url\","]
#[doc = "    \"content_type\","]
#[doc = "    \"depth\","]
#[doc = "    \"fetched_at\","]
#[doc = "    \"html\","]
#[doc = "    \"links\","]
#[doc = "    \"schema\","]
#[doc = "    \"status_code\","]
#[doc = "    \"url\","]
#[doc = "    \"vertical\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"canonical_url\": {"]
#[doc = "      \"description\": \"Canonicalized URL used as the document identity everywhere downstream\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"maxLength\": 4096"]
#[doc = "    },"]
#[doc = "    \"content_type\": {"]
#[doc = "      \"description\": \"Response Content-Type without parameters, e.g. text/html\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 255"]
#[doc = "    },"]
#[doc = "    \"depth\": {"]
#[doc = "      \"description\": \"Link depth from the nearest seed (seeds are depth 0)\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 1000.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"etag\": {"]
#[doc = "      \"description\": \"ETag response header if present\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 512"]
#[doc = "    },"]
#[doc = "    \"fetched_at\": {"]
#[doc = "      \"description\": \"UTC fetch timestamp\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"date-time\""]
#[doc = "    },"]
#[doc = "    \"html\": {"]
#[doc = "      \"description\": \"Raw response body (UTF-8 HTML). Size-capped by the crawler before writing.\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"last_modified\": {"]
#[doc = "      \"description\": \"Last-Modified response header if present, RFC 1123 as sent\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64"]
#[doc = "    },"]
#[doc = "    \"links\": {"]
#[doc = "      \"description\": \"Outbound links discovered on the page, canonicalized\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"format\": \"uri\","]
#[doc = "        \"maxLength\": 4096"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 10000"]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"description\": \"Contract discriminator, always crawl_record.v1\","]
#[doc = "      \"const\": \"crawl_record.v1\""]
#[doc = "    },"]
#[doc = "    \"simhash64\": {"]
#[doc = "      \"description\": \"64-bit SimHash of the extracted text, hex-encoded (16 chars). Absent until P2.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9a-f]{16}$\""]
#[doc = "    },"]
#[doc = "    \"status_code\": {"]
#[doc = "      \"description\": \"HTTP status of the final response\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 599.0,"]
#[doc = "      \"minimum\": 100.0"]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"URL as fetched (post-redirect final URL)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"maxLength\": 4096"]
#[doc = "    },"]
#[doc = "    \"vertical\": {"]
#[doc = "      \"description\": \"Vertical pack id this page was crawled under\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CrawlRecordV1 {
    #[doc = "Canonicalized URL used as the document identity everywhere downstream"]
    pub canonical_url: CrawlRecordV1CanonicalUrl,
    #[doc = "Response Content-Type without parameters, e.g. text/html"]
    pub content_type: CrawlRecordV1ContentType,
    #[doc = "Link depth from the nearest seed (seeds are depth 0)"]
    pub depth: i64,
    #[doc = "ETag response header if present"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub etag: ::std::option::Option<CrawlRecordV1Etag>,
    #[doc = "UTC fetch timestamp"]
    pub fetched_at: ::chrono::DateTime<::chrono::offset::Utc>,
    #[doc = "Raw response body (UTF-8 HTML). Size-capped by the crawler before writing."]
    pub html: ::std::string::String,
    #[doc = "Last-Modified response header if present, RFC 1123 as sent"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub last_modified: ::std::option::Option<CrawlRecordV1LastModified>,
    #[doc = "Outbound links discovered on the page, canonicalized"]
    pub links: ::std::vec::Vec<CrawlRecordV1LinksItem>,
    #[doc = "Contract discriminator, always crawl_record.v1"]
    pub schema: ::serde_json::Value,
    #[doc = "64-bit SimHash of the extracted text, hex-encoded (16 chars). Absent until P2."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub simhash64: ::std::option::Option<CrawlRecordV1Simhash64>,
    #[doc = "HTTP status of the final response"]
    pub status_code: i64,
    #[doc = "URL as fetched (post-redirect final URL)"]
    pub url: CrawlRecordV1Url,
    #[doc = "Vertical pack id this page was crawled under"]
    pub vertical: CrawlRecordV1Vertical,
}
impl CrawlRecordV1 {
    pub fn builder() -> builder::CrawlRecordV1 {
        Default::default()
    }
}
#[doc = "Canonicalized URL used as the document identity everywhere downstream"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Canonicalized URL used as the document identity everywhere downstream\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"maxLength\": 4096"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1CanonicalUrl(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1CanonicalUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1CanonicalUrl> for ::std::string::String {
    fn from(value: CrawlRecordV1CanonicalUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1CanonicalUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRecordV1CanonicalUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1CanonicalUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1CanonicalUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1CanonicalUrl {
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
#[doc = "Response Content-Type without parameters, e.g. text/html"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Response Content-Type without parameters, e.g. text/html\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 255"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1ContentType(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1ContentType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1ContentType> for ::std::string::String {
    fn from(value: CrawlRecordV1ContentType) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1ContentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 255usize {
            return Err("longer than 255 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRecordV1ContentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1ContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1ContentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1ContentType {
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
#[doc = "ETag response header if present"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"ETag response header if present\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 512"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1Etag(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1Etag {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1Etag> for ::std::string::String {
    fn from(value: CrawlRecordV1Etag) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1Etag {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRecordV1Etag {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1Etag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1Etag {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1Etag {
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
#[doc = "Last-Modified response header if present, RFC 1123 as sent"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Last-Modified response header if present, RFC 1123 as sent\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1LastModified(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1LastModified {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1LastModified> for ::std::string::String {
    fn from(value: CrawlRecordV1LastModified) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1LastModified {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 64usize {
            return Err("longer than 64 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRecordV1LastModified {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1LastModified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1LastModified {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1LastModified {
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
#[doc = "`CrawlRecordV1LinksItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"maxLength\": 4096"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1LinksItem(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1LinksItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1LinksItem> for ::std::string::String {
    fn from(value: CrawlRecordV1LinksItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1LinksItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRecordV1LinksItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1LinksItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1LinksItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1LinksItem {
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
#[doc = "64-bit SimHash of the extracted text, hex-encoded (16 chars). Absent until P2."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"64-bit SimHash of the extracted text, hex-encoded (16 chars). Absent until P2.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9a-f]{16}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1Simhash64(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1Simhash64 {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1Simhash64> for ::std::string::String {
    fn from(value: CrawlRecordV1Simhash64) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1Simhash64 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9a-f]{16}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-f]{16}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRecordV1Simhash64 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1Simhash64 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1Simhash64 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1Simhash64 {
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
#[doc = "URL as fetched (post-redirect final URL)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL as fetched (post-redirect final URL)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"maxLength\": 4096"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1Url(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1Url {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1Url> for ::std::string::String {
    fn from(value: CrawlRecordV1Url) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1Url {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CrawlRecordV1Url {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1Url {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1Url {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1Url {
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
#[doc = "Vertical pack id this page was crawled under"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Vertical pack id this page was crawled under\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CrawlRecordV1Vertical(::std::string::String);
impl ::std::ops::Deref for CrawlRecordV1Vertical {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CrawlRecordV1Vertical> for ::std::string::String {
    fn from(value: CrawlRecordV1Vertical) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for CrawlRecordV1Vertical {
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
impl ::std::convert::TryFrom<&str> for CrawlRecordV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CrawlRecordV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CrawlRecordV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CrawlRecordV1Vertical {
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
    pub struct CrawlRecordV1 {
        canonical_url:
            ::std::result::Result<super::CrawlRecordV1CanonicalUrl, ::std::string::String>,
        content_type: ::std::result::Result<super::CrawlRecordV1ContentType, ::std::string::String>,
        depth: ::std::result::Result<i64, ::std::string::String>,
        etag: ::std::result::Result<
            ::std::option::Option<super::CrawlRecordV1Etag>,
            ::std::string::String,
        >,
        fetched_at:
            ::std::result::Result<::chrono::DateTime<::chrono::offset::Utc>, ::std::string::String>,
        html: ::std::result::Result<::std::string::String, ::std::string::String>,
        last_modified: ::std::result::Result<
            ::std::option::Option<super::CrawlRecordV1LastModified>,
            ::std::string::String,
        >,
        links: ::std::result::Result<
            ::std::vec::Vec<super::CrawlRecordV1LinksItem>,
            ::std::string::String,
        >,
        schema: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        simhash64: ::std::result::Result<
            ::std::option::Option<super::CrawlRecordV1Simhash64>,
            ::std::string::String,
        >,
        status_code: ::std::result::Result<i64, ::std::string::String>,
        url: ::std::result::Result<super::CrawlRecordV1Url, ::std::string::String>,
        vertical: ::std::result::Result<super::CrawlRecordV1Vertical, ::std::string::String>,
    }
    impl ::std::default::Default for CrawlRecordV1 {
        fn default() -> Self {
            Self {
                canonical_url: Err("no value supplied for canonical_url".to_string()),
                content_type: Err("no value supplied for content_type".to_string()),
                depth: Err("no value supplied for depth".to_string()),
                etag: Ok(Default::default()),
                fetched_at: Err("no value supplied for fetched_at".to_string()),
                html: Err("no value supplied for html".to_string()),
                last_modified: Ok(Default::default()),
                links: Err("no value supplied for links".to_string()),
                schema: Err("no value supplied for schema".to_string()),
                simhash64: Ok(Default::default()),
                status_code: Err("no value supplied for status_code".to_string()),
                url: Err("no value supplied for url".to_string()),
                vertical: Err("no value supplied for vertical".to_string()),
            }
        }
    }
    impl CrawlRecordV1 {
        pub fn canonical_url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CrawlRecordV1CanonicalUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.canonical_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for canonical_url: {e}"));
            self
        }
        pub fn content_type<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CrawlRecordV1ContentType>,
            T::Error: ::std::fmt::Display,
        {
            self.content_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content_type: {e}"));
            self
        }
        pub fn depth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.depth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for depth: {e}"));
            self
        }
        pub fn etag<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CrawlRecordV1Etag>>,
            T::Error: ::std::fmt::Display,
        {
            self.etag = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for etag: {e}"));
            self
        }
        pub fn fetched_at<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
            T::Error: ::std::fmt::Display,
        {
            self.fetched_at = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fetched_at: {e}"));
            self
        }
        pub fn html<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.html = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for html: {e}"));
            self
        }
        pub fn last_modified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CrawlRecordV1LastModified>>,
            T::Error: ::std::fmt::Display,
        {
            self.last_modified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for last_modified: {e}"));
            self
        }
        pub fn links<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CrawlRecordV1LinksItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.links = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for links: {e}"));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::serde_json::Value>,
            T::Error: ::std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {e}"));
            self
        }
        pub fn simhash64<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::CrawlRecordV1Simhash64>>,
            T::Error: ::std::fmt::Display,
        {
            self.simhash64 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for simhash64: {e}"));
            self
        }
        pub fn status_code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.status_code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for status_code: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CrawlRecordV1Url>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
        pub fn vertical<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CrawlRecordV1Vertical>,
            T::Error: ::std::fmt::Display,
        {
            self.vertical = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vertical: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CrawlRecordV1> for super::CrawlRecordV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CrawlRecordV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                canonical_url: value.canonical_url?,
                content_type: value.content_type?,
                depth: value.depth?,
                etag: value.etag?,
                fetched_at: value.fetched_at?,
                html: value.html?,
                last_modified: value.last_modified?,
                links: value.links?,
                schema: value.schema?,
                simhash64: value.simhash64?,
                status_code: value.status_code?,
                url: value.url?,
                vertical: value.vertical?,
            })
        }
    }
    impl ::std::convert::From<super::CrawlRecordV1> for CrawlRecordV1 {
        fn from(value: super::CrawlRecordV1) -> Self {
            Self {
                canonical_url: Ok(value.canonical_url),
                content_type: Ok(value.content_type),
                depth: Ok(value.depth),
                etag: Ok(value.etag),
                fetched_at: Ok(value.fetched_at),
                html: Ok(value.html),
                last_modified: Ok(value.last_modified),
                links: Ok(value.links),
                schema: Ok(value.schema),
                simhash64: Ok(value.simhash64),
                status_code: Ok(value.status_code),
                url: Ok(value.url),
                vertical: Ok(value.vertical),
            }
        }
    }
}
