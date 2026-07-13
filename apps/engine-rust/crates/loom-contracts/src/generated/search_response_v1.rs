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
#[doc = "Response body of loom-server GET /search. The web app validates this with AJV before rendering."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://loom.dev/schemas/search_response.v1.schema.json\","]
#[doc = "  \"title\": \"SearchResponseV1\","]
#[doc = "  \"description\": \"Response body of loom-server GET /search. The web app validates this with AJV before rendering.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"degraded\","]
#[doc = "    \"query\","]
#[doc = "    \"results\","]
#[doc = "    \"schema\","]
#[doc = "    \"took_ms\","]
#[doc = "    \"total\","]
#[doc = "    \"vertical\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"degraded\": {"]
#[doc = "      \"description\": \"True when the ML sidecar was unavailable and the vector/LLM legs were skipped (BM25-only)\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"query\": {"]
#[doc = "      \"description\": \"The query as executed (post-normalization)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 512"]
#[doc = "    },"]
#[doc = "    \"results\": {"]
#[doc = "      \"description\": \"Ranked results, best first\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"object\","]
#[doc = "        \"required\": ["]
#[doc = "          \"rank\","]
#[doc = "          \"score\","]
#[doc = "          \"snippet\","]
#[doc = "          \"title\","]
#[doc = "          \"url\""]
#[doc = "        ],"]
#[doc = "        \"properties\": {"]
#[doc = "          \"rank\": {"]
#[doc = "            \"description\": \"1-based rank within this response page\","]
#[doc = "            \"type\": \"integer\","]
#[doc = "            \"minimum\": 1.0"]
#[doc = "          },"]
#[doc = "          \"score\": {"]
#[doc = "            \"description\": \"Final ranking score (monotonically decreasing with rank)\","]
#[doc = "            \"type\": \"number\""]
#[doc = "          },"]
#[doc = "          \"snippet\": {"]
#[doc = "            \"description\": \"Best passage with query terms wrapped in ▌▐ marker pair for highlighting\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 1024"]
#[doc = "          },"]
#[doc = "          \"title\": {"]
#[doc = "            \"description\": \"Document title (extracted)\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 512"]
#[doc = "          },"]
#[doc = "          \"url\": {"]
#[doc = "            \"description\": \"Canonical document URL\","]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\","]
#[doc = "            \"maxLength\": 4096"]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"additionalProperties\": false"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 50"]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"description\": \"Contract discriminator, always search_response.v1\","]
#[doc = "      \"const\": \"search_response.v1\""]
#[doc = "    },"]
#[doc = "    \"suggestion\": {"]
#[doc = "      \"description\": \"Optional did-you-mean rewrite when the query looks misspelled (P4)\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"text\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"text\": {"]
#[doc = "          \"description\": \"Suggested corrected query\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"maxLength\": 512"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"took_ms\": {"]
#[doc = "      \"description\": \"Server-side query latency in milliseconds\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"total\": {"]
#[doc = "      \"description\": \"Total matching documents (may be an upper-bound estimate once WAND lands)\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"vertical\": {"]
#[doc = "      \"description\": \"Vertical pack id that was searched\","]
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
pub struct SearchResponseV1 {
    #[doc = "True when the ML sidecar was unavailable and the vector/LLM legs were skipped (BM25-only)"]
    pub degraded: bool,
    #[doc = "The query as executed (post-normalization)"]
    pub query: SearchResponseV1Query,
    #[doc = "Ranked results, best first"]
    pub results: ::std::vec::Vec<SearchResponseV1ResultsItem>,
    #[doc = "Contract discriminator, always search_response.v1"]
    pub schema: ::serde_json::Value,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub suggestion: ::std::option::Option<SearchResponseV1Suggestion>,
    #[doc = "Server-side query latency in milliseconds"]
    pub took_ms: f64,
    #[doc = "Total matching documents (may be an upper-bound estimate once WAND lands)"]
    pub total: u64,
    #[doc = "Vertical pack id that was searched"]
    pub vertical: SearchResponseV1Vertical,
}
impl SearchResponseV1 {
    pub fn builder() -> builder::SearchResponseV1 {
        Default::default()
    }
}
#[doc = "The query as executed (post-normalization)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The query as executed (post-normalization)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 512"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchResponseV1Query(::std::string::String);
impl ::std::ops::Deref for SearchResponseV1Query {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchResponseV1Query> for ::std::string::String {
    fn from(value: SearchResponseV1Query) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchResponseV1Query {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchResponseV1Query {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchResponseV1Query {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchResponseV1Query {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchResponseV1Query {
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
#[doc = "`SearchResponseV1ResultsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"rank\","]
#[doc = "    \"score\","]
#[doc = "    \"snippet\","]
#[doc = "    \"title\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"rank\": {"]
#[doc = "      \"description\": \"1-based rank within this response page\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"score\": {"]
#[doc = "      \"description\": \"Final ranking score (monotonically decreasing with rank)\","]
#[doc = "      \"type\": \"number\""]
#[doc = "    },"]
#[doc = "    \"snippet\": {"]
#[doc = "      \"description\": \"Best passage with query terms wrapped in ▌▐ marker pair for highlighting\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 1024"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"description\": \"Document title (extracted)\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 512"]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"description\": \"Canonical document URL\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"maxLength\": 4096"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SearchResponseV1ResultsItem {
    #[doc = "1-based rank within this response page"]
    pub rank: ::std::num::NonZeroU64,
    #[doc = "Final ranking score (monotonically decreasing with rank)"]
    pub score: f64,
    #[doc = "Best passage with query terms wrapped in ▌▐ marker pair for highlighting"]
    pub snippet: SearchResponseV1ResultsItemSnippet,
    #[doc = "Document title (extracted)"]
    pub title: SearchResponseV1ResultsItemTitle,
    #[doc = "Canonical document URL"]
    pub url: SearchResponseV1ResultsItemUrl,
}
impl SearchResponseV1ResultsItem {
    pub fn builder() -> builder::SearchResponseV1ResultsItem {
        Default::default()
    }
}
#[doc = "Best passage with query terms wrapped in ▌▐ marker pair for highlighting"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Best passage with query terms wrapped in ▌▐ marker pair for highlighting\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 1024"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchResponseV1ResultsItemSnippet(::std::string::String);
impl ::std::ops::Deref for SearchResponseV1ResultsItemSnippet {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchResponseV1ResultsItemSnippet> for ::std::string::String {
    fn from(value: SearchResponseV1ResultsItemSnippet) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchResponseV1ResultsItemSnippet {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 1024usize {
            return Err("longer than 1024 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchResponseV1ResultsItemSnippet {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchResponseV1ResultsItemSnippet {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchResponseV1ResultsItemSnippet {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchResponseV1ResultsItemSnippet {
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
#[doc = "Document title (extracted)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Document title (extracted)\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 512"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchResponseV1ResultsItemTitle(::std::string::String);
impl ::std::ops::Deref for SearchResponseV1ResultsItemTitle {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchResponseV1ResultsItemTitle> for ::std::string::String {
    fn from(value: SearchResponseV1ResultsItemTitle) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchResponseV1ResultsItemTitle {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchResponseV1ResultsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchResponseV1ResultsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchResponseV1ResultsItemTitle {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchResponseV1ResultsItemTitle {
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
#[doc = "Canonical document URL"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Canonical document URL\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"maxLength\": 4096"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchResponseV1ResultsItemUrl(::std::string::String);
impl ::std::ops::Deref for SearchResponseV1ResultsItemUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchResponseV1ResultsItemUrl> for ::std::string::String {
    fn from(value: SearchResponseV1ResultsItemUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchResponseV1ResultsItemUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchResponseV1ResultsItemUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchResponseV1ResultsItemUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchResponseV1ResultsItemUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchResponseV1ResultsItemUrl {
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
#[doc = "Optional did-you-mean rewrite when the query looks misspelled (P4)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Optional did-you-mean rewrite when the query looks misspelled (P4)\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"text\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"Suggested corrected query\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 512"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct SearchResponseV1Suggestion {
    #[doc = "Suggested corrected query"]
    pub text: SearchResponseV1SuggestionText,
}
impl SearchResponseV1Suggestion {
    pub fn builder() -> builder::SearchResponseV1Suggestion {
        Default::default()
    }
}
#[doc = "Suggested corrected query"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Suggested corrected query\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 512"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchResponseV1SuggestionText(::std::string::String);
impl ::std::ops::Deref for SearchResponseV1SuggestionText {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchResponseV1SuggestionText> for ::std::string::String {
    fn from(value: SearchResponseV1SuggestionText) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchResponseV1SuggestionText {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchResponseV1SuggestionText {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchResponseV1SuggestionText {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchResponseV1SuggestionText {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchResponseV1SuggestionText {
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
#[doc = "Vertical pack id that was searched"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Vertical pack id that was searched\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchResponseV1Vertical(::std::string::String);
impl ::std::ops::Deref for SearchResponseV1Vertical {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchResponseV1Vertical> for ::std::string::String {
    fn from(value: SearchResponseV1Vertical) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchResponseV1Vertical {
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
impl ::std::convert::TryFrom<&str> for SearchResponseV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchResponseV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchResponseV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchResponseV1Vertical {
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
    pub struct SearchResponseV1 {
        degraded: ::std::result::Result<bool, ::std::string::String>,
        query: ::std::result::Result<super::SearchResponseV1Query, ::std::string::String>,
        results: ::std::result::Result<
            ::std::vec::Vec<super::SearchResponseV1ResultsItem>,
            ::std::string::String,
        >,
        schema: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        suggestion: ::std::result::Result<
            ::std::option::Option<super::SearchResponseV1Suggestion>,
            ::std::string::String,
        >,
        took_ms: ::std::result::Result<f64, ::std::string::String>,
        total: ::std::result::Result<u64, ::std::string::String>,
        vertical: ::std::result::Result<super::SearchResponseV1Vertical, ::std::string::String>,
    }
    impl ::std::default::Default for SearchResponseV1 {
        fn default() -> Self {
            Self {
                degraded: Err("no value supplied for degraded".to_string()),
                query: Err("no value supplied for query".to_string()),
                results: Err("no value supplied for results".to_string()),
                schema: Err("no value supplied for schema".to_string()),
                suggestion: Ok(Default::default()),
                took_ms: Err("no value supplied for took_ms".to_string()),
                total: Err("no value supplied for total".to_string()),
                vertical: Err("no value supplied for vertical".to_string()),
            }
        }
    }
    impl SearchResponseV1 {
        pub fn degraded<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.degraded = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for degraded: {e}"));
            self
        }
        pub fn query<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchResponseV1Query>,
            T::Error: ::std::fmt::Display,
        {
            self.query = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for query: {e}"));
            self
        }
        pub fn results<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::SearchResponseV1ResultsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.results = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for results: {e}"));
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
        pub fn suggestion<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::SearchResponseV1Suggestion>>,
            T::Error: ::std::fmt::Display,
        {
            self.suggestion = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for suggestion: {e}"));
            self
        }
        pub fn took_ms<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.took_ms = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for took_ms: {e}"));
            self
        }
        pub fn total<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.total = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for total: {e}"));
            self
        }
        pub fn vertical<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchResponseV1Vertical>,
            T::Error: ::std::fmt::Display,
        {
            self.vertical = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vertical: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SearchResponseV1> for super::SearchResponseV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SearchResponseV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                degraded: value.degraded?,
                query: value.query?,
                results: value.results?,
                schema: value.schema?,
                suggestion: value.suggestion?,
                took_ms: value.took_ms?,
                total: value.total?,
                vertical: value.vertical?,
            })
        }
    }
    impl ::std::convert::From<super::SearchResponseV1> for SearchResponseV1 {
        fn from(value: super::SearchResponseV1) -> Self {
            Self {
                degraded: Ok(value.degraded),
                query: Ok(value.query),
                results: Ok(value.results),
                schema: Ok(value.schema),
                suggestion: Ok(value.suggestion),
                took_ms: Ok(value.took_ms),
                total: Ok(value.total),
                vertical: Ok(value.vertical),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SearchResponseV1ResultsItem {
        rank: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        score: ::std::result::Result<f64, ::std::string::String>,
        snippet:
            ::std::result::Result<super::SearchResponseV1ResultsItemSnippet, ::std::string::String>,
        title:
            ::std::result::Result<super::SearchResponseV1ResultsItemTitle, ::std::string::String>,
        url: ::std::result::Result<super::SearchResponseV1ResultsItemUrl, ::std::string::String>,
    }
    impl ::std::default::Default for SearchResponseV1ResultsItem {
        fn default() -> Self {
            Self {
                rank: Err("no value supplied for rank".to_string()),
                score: Err("no value supplied for score".to_string()),
                snippet: Err("no value supplied for snippet".to_string()),
                title: Err("no value supplied for title".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl SearchResponseV1ResultsItem {
        pub fn rank<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.rank = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rank: {e}"));
            self
        }
        pub fn score<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.score = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for score: {e}"));
            self
        }
        pub fn snippet<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchResponseV1ResultsItemSnippet>,
            T::Error: ::std::fmt::Display,
        {
            self.snippet = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for snippet: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchResponseV1ResultsItemTitle>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchResponseV1ResultsItemUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SearchResponseV1ResultsItem> for super::SearchResponseV1ResultsItem {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SearchResponseV1ResultsItem,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                rank: value.rank?,
                score: value.score?,
                snippet: value.snippet?,
                title: value.title?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::SearchResponseV1ResultsItem> for SearchResponseV1ResultsItem {
        fn from(value: super::SearchResponseV1ResultsItem) -> Self {
            Self {
                rank: Ok(value.rank),
                score: Ok(value.score),
                snippet: Ok(value.snippet),
                title: Ok(value.title),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct SearchResponseV1Suggestion {
        text: ::std::result::Result<super::SearchResponseV1SuggestionText, ::std::string::String>,
    }
    impl ::std::default::Default for SearchResponseV1Suggestion {
        fn default() -> Self {
            Self {
                text: Err("no value supplied for text".to_string()),
            }
        }
    }
    impl SearchResponseV1Suggestion {
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchResponseV1SuggestionText>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SearchResponseV1Suggestion> for super::SearchResponseV1Suggestion {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SearchResponseV1Suggestion,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { text: value.text? })
        }
    }
    impl ::std::convert::From<super::SearchResponseV1Suggestion> for SearchResponseV1Suggestion {
        fn from(value: super::SearchResponseV1Suggestion) -> Self {
            Self {
                text: Ok(value.text),
            }
        }
    }
}
