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
#[doc = "Query parameters accepted by loom-server GET /search, expressed as an object so the same contract validates client and server."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://loom.dev/schemas/search_request.v1.schema.json\","]
#[doc = "  \"title\": \"SearchRequestV1\","]
#[doc = "  \"description\": \"Query parameters accepted by loom-server GET /search, expressed as an object so the same contract validates client and server.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"query\","]
#[doc = "    \"schema\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"limit\": {"]
#[doc = "      \"description\": \"Number of results to return\","]
#[doc = "      \"default\": 10,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 50.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"offset\": {"]
#[doc = "      \"description\": \"Result offset for pagination\","]
#[doc = "      \"default\": 0,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 1000.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"query\": {"]
#[doc = "      \"description\": \"User query string\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 512,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"description\": \"Contract discriminator, always search_request.v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"search_request.v1\""]
#[doc = "    },"]
#[doc = "    \"vertical\": {"]
#[doc = "      \"description\": \"Vertical pack id to search (defaults to devdocs)\","]
#[doc = "      \"default\": \"devdocs\","]
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
pub struct SearchRequestV1 {
    #[doc = "Number of results to return"]
    #[serde(default = "defaults::default_nzu64::<::std::num::NonZeroU64, 10>")]
    pub limit: ::std::num::NonZeroU64,
    #[doc = "Result offset for pagination"]
    #[serde(default)]
    pub offset: i64,
    #[doc = "User query string"]
    pub query: SearchRequestV1Query,
    #[doc = "Contract discriminator, always search_request.v1"]
    pub schema: ::std::string::String,
    #[doc = "Vertical pack id to search (defaults to devdocs)"]
    #[serde(default = "defaults::search_request_v1_vertical")]
    pub vertical: SearchRequestV1Vertical,
}
impl SearchRequestV1 {
    pub fn builder() -> builder::SearchRequestV1 {
        Default::default()
    }
}
#[doc = "User query string"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"User query string\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 512,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchRequestV1Query(::std::string::String);
impl ::std::ops::Deref for SearchRequestV1Query {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchRequestV1Query> for ::std::string::String {
    fn from(value: SearchRequestV1Query) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for SearchRequestV1Query {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 512usize {
            return Err("longer than 512 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for SearchRequestV1Query {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchRequestV1Query {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchRequestV1Query {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchRequestV1Query {
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
#[doc = "Vertical pack id to search (defaults to devdocs)"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Vertical pack id to search (defaults to devdocs)\","]
#[doc = "  \"default\": \"devdocs\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct SearchRequestV1Vertical(::std::string::String);
impl ::std::ops::Deref for SearchRequestV1Vertical {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<SearchRequestV1Vertical> for ::std::string::String {
    fn from(value: SearchRequestV1Vertical) -> Self {
        value.0
    }
}
impl ::std::default::Default for SearchRequestV1Vertical {
    fn default() -> Self {
        SearchRequestV1Vertical("devdocs".to_string())
    }
}
impl ::std::str::FromStr for SearchRequestV1Vertical {
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
impl ::std::convert::TryFrom<&str> for SearchRequestV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for SearchRequestV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for SearchRequestV1Vertical {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for SearchRequestV1Vertical {
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
    pub struct SearchRequestV1 {
        limit: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        offset: ::std::result::Result<i64, ::std::string::String>,
        query: ::std::result::Result<super::SearchRequestV1Query, ::std::string::String>,
        schema: ::std::result::Result<::std::string::String, ::std::string::String>,
        vertical: ::std::result::Result<super::SearchRequestV1Vertical, ::std::string::String>,
    }
    impl ::std::default::Default for SearchRequestV1 {
        fn default() -> Self {
            Self {
                limit: Ok(super::defaults::default_nzu64::<::std::num::NonZeroU64, 10>()),
                offset: Ok(Default::default()),
                query: Err("no value supplied for query".to_string()),
                schema: Err("no value supplied for schema".to_string()),
                vertical: Ok(super::defaults::search_request_v1_vertical()),
            }
        }
    }
    impl SearchRequestV1 {
        pub fn limit<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.limit = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limit: {e}"));
            self
        }
        pub fn offset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.offset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for offset: {e}"));
            self
        }
        pub fn query<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchRequestV1Query>,
            T::Error: ::std::fmt::Display,
        {
            self.query = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for query: {e}"));
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
        pub fn vertical<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::SearchRequestV1Vertical>,
            T::Error: ::std::fmt::Display,
        {
            self.vertical = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for vertical: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<SearchRequestV1> for super::SearchRequestV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: SearchRequestV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                limit: value.limit?,
                offset: value.offset?,
                query: value.query?,
                schema: value.schema?,
                vertical: value.vertical?,
            })
        }
    }
    impl ::std::convert::From<super::SearchRequestV1> for SearchRequestV1 {
        fn from(value: super::SearchRequestV1) -> Self {
            Self {
                limit: Ok(value.limit),
                offset: Ok(value.offset),
                query: Ok(value.query),
                schema: Ok(value.schema),
                vertical: Ok(value.vertical),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_nzu64<T, const V: u64>() -> T
    where
        T: ::std::convert::TryFrom<::std::num::NonZeroU64>,
        <T as ::std::convert::TryFrom<::std::num::NonZeroU64>>::Error: ::std::fmt::Debug,
    {
        T::try_from(::std::num::NonZeroU64::try_from(V).unwrap()).unwrap()
    }
    pub(super) fn search_request_v1_vertical() -> super::SearchRequestV1Vertical {
        super::SearchRequestV1Vertical("devdocs".to_string())
    }
}
