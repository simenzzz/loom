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
#[doc = "A vertical pack as the loader sees it: pack.toml, policy.toml, extract.toml and seeds.txt merged into one document. The Go crawler validates what it parsed; the Rust ranker reads the same shape for field weights. Adding a vertical must never require a code change."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"$id\": \"https://loom.dev/schemas/vertical_pack.v1.schema.json\","]
#[doc = "  \"title\": \"VerticalPackV1\","]
#[doc = "  \"description\": \"A vertical pack as the loader sees it: pack.toml, policy.toml, extract.toml and seeds.txt merged into one document. The Go crawler validates what it parsed; the Rust ranker reads the same shape for field weights. Adding a vertical must never require a code change.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"code\","]
#[doc = "    \"content\","]
#[doc = "    \"hosts\","]
#[doc = "    \"limits\","]
#[doc = "    \"pack\","]
#[doc = "    \"politeness\","]
#[doc = "    \"ranking\","]
#[doc = "    \"schema\","]
#[doc = "    \"seeds\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"code\": {"]
#[doc = "      \"description\": \"Code-block extraction hints, from extract.toml [code]\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"preserve_selectors\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"preserve_selectors\": {"]
#[doc = "          \"description\": \"Selectors whose contents are indexed as the separate code field rather than body text\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 128,"]
#[doc = "            \"minLength\": 1"]
#[doc = "          },"]
#[doc = "          \"maxItems\": 64"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"content\": {"]
#[doc = "      \"description\": \"Main-content extraction hints, from extract.toml [content]. Applied on top of the generic text-density algorithm from P3.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"main_selectors\","]
#[doc = "        \"title_selectors\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"main_selectors\": {"]
#[doc = "          \"description\": \"Selectors that narrow where the main content lives, most specific first\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 128,"]
#[doc = "            \"minLength\": 1"]
#[doc = "          },"]
#[doc = "          \"maxItems\": 64"]
#[doc = "        },"]
#[doc = "        \"title_selectors\": {"]
#[doc = "          \"description\": \"Selectors consulted in order to find the document title\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 128,"]
#[doc = "            \"minLength\": 1"]
#[doc = "          },"]
#[doc = "          \"maxItems\": 64,"]
#[doc = "          \"minItems\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"hosts\": {"]
#[doc = "      \"description\": \"Host allowlist, from policy.toml [hosts]\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"allowed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"allowed\": {"]
#[doc = "          \"description\": \"Hosts the frontier may fetch; everything else is dropped. Bare names are permitted so the fixture site (localhost, or the compose service name) is expressible. This list is a NAME allowlist and is not by itself an SSRF control: DNS can point an allowed name at a private address, and redirects leave the allowed host after the check. The fetcher must additionally reject post-resolution addresses in loopback, link-local, private, CGNAT and IPv4-mapped-v6 ranges, connect to the validated IP rather than re-resolving, and re-run both checks on every redirect hop. Entries carry no port, so allowlisting a host authorizes every port on it — the runtime matcher must compare host and port.\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 253,"]
#[doc = "            \"minLength\": 1,"]
#[doc = "            \"pattern\": \"^[a-z0-9]([a-z0-9-]*[a-z0-9])?(\\\\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)*$\""]
#[doc = "          },"]
#[doc = "          \"maxItems\": 1000,"]
#[doc = "          \"minItems\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"limits\": {"]
#[doc = "      \"description\": \"Hard crawl limits, from policy.toml [limits]. The crawler enforces every one of these; nothing is best-effort.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"max_body_bytes\","]
#[doc = "        \"max_depth\","]
#[doc = "        \"max_pages\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"max_body_bytes\": {"]
#[doc = "          \"description\": \"Response body size cap; larger bodies are truncated, never buffered whole. Bounded above at 128 MiB because a cap with no ceiling is not a cap — and because the fetcher must never size a buffer from it.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"maximum\": 134217728.0,"]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"max_depth\": {"]
#[doc = "          \"description\": \"Maximum link depth from a seed; bounded by crawl_record.v1's depth ceiling\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"maximum\": 1000.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"max_pages\": {"]
#[doc = "          \"description\": \"Maximum pages to fetch in one crawl. Bounded above because max_depth is only a meaningful ceiling while this one is: an unbounded page budget makes a depth cap decorative.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"maximum\": 10000000.0,"]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"pack\": {"]
#[doc = "      \"description\": \"Pack identity, from pack.toml [pack]\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\","]
#[doc = "        \"name\","]
#[doc = "        \"tab_label\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"description\": \"Vertical id; matches the directory name under verticals/ and the vertical field on every downstream document\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"maxLength\": 64,"]
#[doc = "          \"minLength\": 1,"]
#[doc = "          \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "        },"]
#[doc = "        \"name\": {"]
#[doc = "          \"description\": \"Human-readable pack name\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"maxLength\": 128,"]
#[doc = "          \"minLength\": 1"]
#[doc = "        },"]
#[doc = "        \"tab_label\": {"]
#[doc = "          \"description\": \"Short label for the vertical tab in the web UI\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"maxLength\": 32,"]
#[doc = "          \"minLength\": 1"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"politeness\": {"]
#[doc = "      \"description\": \"Politeness policy, from policy.toml [politeness]\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"default_rps_per_host\","]
#[doc = "        \"respect_crawl_delay\","]
#[doc = "        \"respect_robots\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"default_rps_per_host\": {"]
#[doc = "          \"description\": \"Default request rate per host when robots.txt states no crawl-delay\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 100.0,"]
#[doc = "          \"exclusiveMinimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"respect_crawl_delay\": {"]
#[doc = "          \"description\": \"Whether a robots.txt Crawl-delay overrides default_rps_per_host when it is more conservative\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"respect_robots\": {"]
#[doc = "          \"description\": \"Whether robots.txt disallow rules are honored. The schema cannot enforce when false is legitimate, so the loader must: refuse false unless every hosts.allowed entry is loopback or the LOOM_FIXTURE_SITE_BASE origin, and log at WARN when it is honored. Setting this false against a third party's host is not a configuration choice, it is a defect.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"ranking\": {"]
#[doc = "      \"description\": \"Ranking parameters, from pack.toml [ranking]. Field weights apply from P5; the BM25 constants are read from P1 onward so no scoring constant is hardcoded in Rust.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"anchor_weight\","]
#[doc = "        \"bm25_b\","]
#[doc = "        \"bm25_k1\","]
#[doc = "        \"body_weight\","]
#[doc = "        \"code_weight\","]
#[doc = "        \"recency_half_life_days\","]
#[doc = "        \"title_weight\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"anchor_weight\": {"]
#[doc = "          \"description\": \"BM25F weight of the inbound anchor-text field\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"bm25_b\": {"]
#[doc = "          \"description\": \"BM25 length-normalization parameter in [0,1], conventionally 0.75\","]
#[doc = "          \"default\": 0.75,"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 1.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"bm25_k1\": {"]
#[doc = "          \"description\": \"BM25 term-frequency saturation parameter, conventionally 1.2\","]
#[doc = "          \"default\": 1.2,"]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"maximum\": 10.0,"]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"body_weight\": {"]
#[doc = "          \"description\": \"BM25F weight of the body field\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"code_weight\": {"]
#[doc = "          \"description\": \"BM25F weight of the code-block field\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"recency_half_life_days\": {"]
#[doc = "          \"description\": \"Half-life of the recency decay in days; 0 disables decay, as documentation corpora require\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        },"]
#[doc = "        \"title_weight\": {"]
#[doc = "          \"description\": \"BM25F weight of the title field\","]
#[doc = "          \"type\": \"number\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"schema\": {"]
#[doc = "      \"description\": \"Contract discriminator, always vertical_pack.v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"vertical_pack.v1\""]
#[doc = "    },"]
#[doc = "    \"seeds\": {"]
#[doc = "      \"description\": \"Crawl seeds, from seeds.txt with comments and blank lines removed. Seeds are depth 0, which means they enter the frontier as its initial contents. THE SCHEMA CANNOT BIND THEM TO hosts.allowed — that is a cross-field rule, so the loader MUST re-check every seed's host against hosts.allowed and reject the pack otherwise. Treat a seed as untrusted input, not as pre-authorized because it came from config.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"description\": \"An http(s) seed URL. The authority may not contain userinfo: 'https://docs.python.org@169.254.169.254/' parses as a request to 169.254.169.254 while reading as docs.python.org to a human, which is the classic allowlist bypass.\","]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"format\": \"uri\","]
#[doc = "        \"maxLength\": 4096,"]
#[doc = "        \"pattern\": \"^https?://[a-zA-Z0-9._~%!$&'()*+,;=:\\\\[\\\\]-]+([/?#]|$)\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 10000,"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"url_filters\": {"]
#[doc = "      \"description\": \"URL rejection rules, from policy.toml [url_filters]. Absent means no substring filtering.\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"deny_contains\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"deny_contains\": {"]
#[doc = "          \"description\": \"Drop any URL containing one of these substrings\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"maxLength\": 256,"]
#[doc = "            \"minLength\": 1"]
#[doc = "          },"]
#[doc = "          \"maxItems\": 1000"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1 {
    pub code: VerticalPackV1Code,
    pub content: VerticalPackV1Content,
    pub hosts: VerticalPackV1Hosts,
    pub limits: VerticalPackV1Limits,
    pub pack: VerticalPackV1Pack,
    pub politeness: VerticalPackV1Politeness,
    pub ranking: VerticalPackV1Ranking,
    #[doc = "Contract discriminator, always vertical_pack.v1"]
    pub schema: ::std::string::String,
    #[doc = "Crawl seeds, from seeds.txt with comments and blank lines removed. Seeds are depth 0, which means they enter the frontier as its initial contents. THE SCHEMA CANNOT BIND THEM TO hosts.allowed — that is a cross-field rule, so the loader MUST re-check every seed's host against hosts.allowed and reject the pack otherwise. Treat a seed as untrusted input, not as pre-authorized because it came from config."]
    pub seeds: ::std::vec::Vec<VerticalPackV1SeedsItem>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub url_filters: ::std::option::Option<VerticalPackV1UrlFilters>,
}
impl VerticalPackV1 {
    pub fn builder() -> builder::VerticalPackV1 {
        Default::default()
    }
}
#[doc = "Code-block extraction hints, from extract.toml [code]"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Code-block extraction hints, from extract.toml [code]\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"preserve_selectors\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"preserve_selectors\": {"]
#[doc = "      \"description\": \"Selectors whose contents are indexed as the separate code field rather than body text\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 128,"]
#[doc = "        \"minLength\": 1"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 64"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1Code {
    #[doc = "Selectors whose contents are indexed as the separate code field rather than body text"]
    pub preserve_selectors: ::std::vec::Vec<VerticalPackV1CodePreserveSelectorsItem>,
}
impl VerticalPackV1Code {
    pub fn builder() -> builder::VerticalPackV1Code {
        Default::default()
    }
}
#[doc = "`VerticalPackV1CodePreserveSelectorsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 128,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1CodePreserveSelectorsItem(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1CodePreserveSelectorsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1CodePreserveSelectorsItem> for ::std::string::String {
    fn from(value: VerticalPackV1CodePreserveSelectorsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1CodePreserveSelectorsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1CodePreserveSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1CodePreserveSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1CodePreserveSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1CodePreserveSelectorsItem {
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
#[doc = "Main-content extraction hints, from extract.toml [content]. Applied on top of the generic text-density algorithm from P3."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Main-content extraction hints, from extract.toml [content]. Applied on top of the generic text-density algorithm from P3.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"main_selectors\","]
#[doc = "    \"title_selectors\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"main_selectors\": {"]
#[doc = "      \"description\": \"Selectors that narrow where the main content lives, most specific first\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 128,"]
#[doc = "        \"minLength\": 1"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 64"]
#[doc = "    },"]
#[doc = "    \"title_selectors\": {"]
#[doc = "      \"description\": \"Selectors consulted in order to find the document title\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 128,"]
#[doc = "        \"minLength\": 1"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 64,"]
#[doc = "      \"minItems\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1Content {
    #[doc = "Selectors that narrow where the main content lives, most specific first"]
    pub main_selectors: ::std::vec::Vec<VerticalPackV1ContentMainSelectorsItem>,
    #[doc = "Selectors consulted in order to find the document title"]
    pub title_selectors: ::std::vec::Vec<VerticalPackV1ContentTitleSelectorsItem>,
}
impl VerticalPackV1Content {
    pub fn builder() -> builder::VerticalPackV1Content {
        Default::default()
    }
}
#[doc = "`VerticalPackV1ContentMainSelectorsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 128,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1ContentMainSelectorsItem(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1ContentMainSelectorsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1ContentMainSelectorsItem> for ::std::string::String {
    fn from(value: VerticalPackV1ContentMainSelectorsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1ContentMainSelectorsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1ContentMainSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1ContentMainSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1ContentMainSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1ContentMainSelectorsItem {
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
#[doc = "`VerticalPackV1ContentTitleSelectorsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 128,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1ContentTitleSelectorsItem(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1ContentTitleSelectorsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1ContentTitleSelectorsItem> for ::std::string::String {
    fn from(value: VerticalPackV1ContentTitleSelectorsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1ContentTitleSelectorsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1ContentTitleSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1ContentTitleSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1ContentTitleSelectorsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1ContentTitleSelectorsItem {
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
#[doc = "Host allowlist, from policy.toml [hosts]"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Host allowlist, from policy.toml [hosts]\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"allowed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"allowed\": {"]
#[doc = "      \"description\": \"Hosts the frontier may fetch; everything else is dropped. Bare names are permitted so the fixture site (localhost, or the compose service name) is expressible. This list is a NAME allowlist and is not by itself an SSRF control: DNS can point an allowed name at a private address, and redirects leave the allowed host after the check. The fetcher must additionally reject post-resolution addresses in loopback, link-local, private, CGNAT and IPv4-mapped-v6 ranges, connect to the validated IP rather than re-resolving, and re-run both checks on every redirect hop. Entries carry no port, so allowlisting a host authorizes every port on it — the runtime matcher must compare host and port.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 253,"]
#[doc = "        \"minLength\": 1,"]
#[doc = "        \"pattern\": \"^[a-z0-9]([a-z0-9-]*[a-z0-9])?(\\\\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)*$\""]
#[doc = "      },"]
#[doc = "      \"maxItems\": 1000,"]
#[doc = "      \"minItems\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1Hosts {
    #[doc = "Hosts the frontier may fetch; everything else is dropped. Bare names are permitted so the fixture site (localhost, or the compose service name) is expressible. This list is a NAME allowlist and is not by itself an SSRF control: DNS can point an allowed name at a private address, and redirects leave the allowed host after the check. The fetcher must additionally reject post-resolution addresses in loopback, link-local, private, CGNAT and IPv4-mapped-v6 ranges, connect to the validated IP rather than re-resolving, and re-run both checks on every redirect hop. Entries carry no port, so allowlisting a host authorizes every port on it — the runtime matcher must compare host and port."]
    pub allowed: ::std::vec::Vec<VerticalPackV1HostsAllowedItem>,
}
impl VerticalPackV1Hosts {
    pub fn builder() -> builder::VerticalPackV1Hosts {
        Default::default()
    }
}
#[doc = "`VerticalPackV1HostsAllowedItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 253,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-z0-9]([a-z0-9-]*[a-z0-9])?(\\\\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1HostsAllowedItem(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1HostsAllowedItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1HostsAllowedItem> for ::std::string::String {
    fn from(value: VerticalPackV1HostsAllowedItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1HostsAllowedItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 253usize {
            return Err("longer than 253 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-z0-9]([a-z0-9-]*[a-z0-9])?(\\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)*$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-z0-9]([a-z0-9-]*[a-z0-9])?(\\.[a-z0-9]([a-z0-9-]*[a-z0-9])?)*$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1HostsAllowedItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1HostsAllowedItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1HostsAllowedItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1HostsAllowedItem {
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
#[doc = "Hard crawl limits, from policy.toml [limits]. The crawler enforces every one of these; nothing is best-effort."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Hard crawl limits, from policy.toml [limits]. The crawler enforces every one of these; nothing is best-effort.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"max_body_bytes\","]
#[doc = "    \"max_depth\","]
#[doc = "    \"max_pages\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"max_body_bytes\": {"]
#[doc = "      \"description\": \"Response body size cap; larger bodies are truncated, never buffered whole. Bounded above at 128 MiB because a cap with no ceiling is not a cap — and because the fetcher must never size a buffer from it.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 134217728.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"max_depth\": {"]
#[doc = "      \"description\": \"Maximum link depth from a seed; bounded by crawl_record.v1's depth ceiling\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 1000.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"max_pages\": {"]
#[doc = "      \"description\": \"Maximum pages to fetch in one crawl. Bounded above because max_depth is only a meaningful ceiling while this one is: an unbounded page budget makes a depth cap decorative.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"maximum\": 10000000.0,"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1Limits {
    #[doc = "Response body size cap; larger bodies are truncated, never buffered whole. Bounded above at 128 MiB because a cap with no ceiling is not a cap — and because the fetcher must never size a buffer from it."]
    pub max_body_bytes: ::std::num::NonZeroU64,
    #[doc = "Maximum link depth from a seed; bounded by crawl_record.v1's depth ceiling"]
    pub max_depth: i64,
    #[doc = "Maximum pages to fetch in one crawl. Bounded above because max_depth is only a meaningful ceiling while this one is: an unbounded page budget makes a depth cap decorative."]
    pub max_pages: ::std::num::NonZeroU64,
}
impl VerticalPackV1Limits {
    pub fn builder() -> builder::VerticalPackV1Limits {
        Default::default()
    }
}
#[doc = "Pack identity, from pack.toml [pack]"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Pack identity, from pack.toml [pack]\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"id\","]
#[doc = "    \"name\","]
#[doc = "    \"tab_label\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"id\": {"]
#[doc = "      \"description\": \"Vertical id; matches the directory name under verticals/ and the vertical field on every downstream document\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 64,"]
#[doc = "      \"minLength\": 1,"]
#[doc = "      \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"description\": \"Human-readable pack name\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 128,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    },"]
#[doc = "    \"tab_label\": {"]
#[doc = "      \"description\": \"Short label for the vertical tab in the web UI\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"maxLength\": 32,"]
#[doc = "      \"minLength\": 1"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1Pack {
    #[doc = "Vertical id; matches the directory name under verticals/ and the vertical field on every downstream document"]
    pub id: VerticalPackV1PackId,
    #[doc = "Human-readable pack name"]
    pub name: VerticalPackV1PackName,
    #[doc = "Short label for the vertical tab in the web UI"]
    pub tab_label: VerticalPackV1PackTabLabel,
}
impl VerticalPackV1Pack {
    pub fn builder() -> builder::VerticalPackV1Pack {
        Default::default()
    }
}
#[doc = "Vertical id; matches the directory name under verticals/ and the vertical field on every downstream document"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Vertical id; matches the directory name under verticals/ and the vertical field on every downstream document\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 64,"]
#[doc = "  \"minLength\": 1,"]
#[doc = "  \"pattern\": \"^[a-z0-9][a-z0-9_-]*$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1PackId(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1PackId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1PackId> for ::std::string::String {
    fn from(value: VerticalPackV1PackId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1PackId {
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
impl ::std::convert::TryFrom<&str> for VerticalPackV1PackId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1PackId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1PackId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1PackId {
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
#[doc = "Human-readable pack name"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Human-readable pack name\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 128,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1PackName(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1PackName {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1PackName> for ::std::string::String {
    fn from(value: VerticalPackV1PackName) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1PackName {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 128usize {
            return Err("longer than 128 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1PackName {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1PackName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1PackName {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1PackName {
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
#[doc = "Short label for the vertical tab in the web UI"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Short label for the vertical tab in the web UI\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 32,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1PackTabLabel(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1PackTabLabel {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1PackTabLabel> for ::std::string::String {
    fn from(value: VerticalPackV1PackTabLabel) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1PackTabLabel {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 32usize {
            return Err("longer than 32 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1PackTabLabel {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1PackTabLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1PackTabLabel {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1PackTabLabel {
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
#[doc = "Politeness policy, from policy.toml [politeness]"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Politeness policy, from policy.toml [politeness]\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"default_rps_per_host\","]
#[doc = "    \"respect_crawl_delay\","]
#[doc = "    \"respect_robots\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"default_rps_per_host\": {"]
#[doc = "      \"description\": \"Default request rate per host when robots.txt states no crawl-delay\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 100.0,"]
#[doc = "      \"exclusiveMinimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"respect_crawl_delay\": {"]
#[doc = "      \"description\": \"Whether a robots.txt Crawl-delay overrides default_rps_per_host when it is more conservative\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"respect_robots\": {"]
#[doc = "      \"description\": \"Whether robots.txt disallow rules are honored. The schema cannot enforce when false is legitimate, so the loader must: refuse false unless every hosts.allowed entry is loopback or the LOOM_FIXTURE_SITE_BASE origin, and log at WARN when it is honored. Setting this false against a third party's host is not a configuration choice, it is a defect.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1Politeness {
    #[doc = "Default request rate per host when robots.txt states no crawl-delay"]
    pub default_rps_per_host: f64,
    #[doc = "Whether a robots.txt Crawl-delay overrides default_rps_per_host when it is more conservative"]
    pub respect_crawl_delay: bool,
    #[doc = "Whether robots.txt disallow rules are honored. The schema cannot enforce when false is legitimate, so the loader must: refuse false unless every hosts.allowed entry is loopback or the LOOM_FIXTURE_SITE_BASE origin, and log at WARN when it is honored. Setting this false against a third party's host is not a configuration choice, it is a defect."]
    pub respect_robots: bool,
}
impl VerticalPackV1Politeness {
    pub fn builder() -> builder::VerticalPackV1Politeness {
        Default::default()
    }
}
#[doc = "Ranking parameters, from pack.toml [ranking]. Field weights apply from P5; the BM25 constants are read from P1 onward so no scoring constant is hardcoded in Rust."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"Ranking parameters, from pack.toml [ranking]. Field weights apply from P5; the BM25 constants are read from P1 onward so no scoring constant is hardcoded in Rust.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"anchor_weight\","]
#[doc = "    \"bm25_b\","]
#[doc = "    \"bm25_k1\","]
#[doc = "    \"body_weight\","]
#[doc = "    \"code_weight\","]
#[doc = "    \"recency_half_life_days\","]
#[doc = "    \"title_weight\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"anchor_weight\": {"]
#[doc = "      \"description\": \"BM25F weight of the inbound anchor-text field\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"bm25_b\": {"]
#[doc = "      \"description\": \"BM25 length-normalization parameter in [0,1], conventionally 0.75\","]
#[doc = "      \"default\": 0.75,"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 1.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"bm25_k1\": {"]
#[doc = "      \"description\": \"BM25 term-frequency saturation parameter, conventionally 1.2\","]
#[doc = "      \"default\": 1.2,"]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"maximum\": 10.0,"]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"body_weight\": {"]
#[doc = "      \"description\": \"BM25F weight of the body field\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"code_weight\": {"]
#[doc = "      \"description\": \"BM25F weight of the code-block field\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"recency_half_life_days\": {"]
#[doc = "      \"description\": \"Half-life of the recency decay in days; 0 disables decay, as documentation corpora require\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    },"]
#[doc = "    \"title_weight\": {"]
#[doc = "      \"description\": \"BM25F weight of the title field\","]
#[doc = "      \"type\": \"number\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1Ranking {
    #[doc = "BM25F weight of the inbound anchor-text field"]
    pub anchor_weight: f64,
    #[doc = "BM25 length-normalization parameter in [0,1], conventionally 0.75"]
    pub bm25_b: f64,
    #[doc = "BM25 term-frequency saturation parameter, conventionally 1.2"]
    pub bm25_k1: f64,
    #[doc = "BM25F weight of the body field"]
    pub body_weight: f64,
    #[doc = "BM25F weight of the code-block field"]
    pub code_weight: f64,
    #[doc = "Half-life of the recency decay in days; 0 disables decay, as documentation corpora require"]
    pub recency_half_life_days: f64,
    #[doc = "BM25F weight of the title field"]
    pub title_weight: f64,
}
impl VerticalPackV1Ranking {
    pub fn builder() -> builder::VerticalPackV1Ranking {
        Default::default()
    }
}
#[doc = "An http(s) seed URL. The authority may not contain userinfo: 'https://docs.python.org@169.254.169.254/' parses as a request to 169.254.169.254 while reading as docs.python.org to a human, which is the classic allowlist bypass."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"An http(s) seed URL. The authority may not contain userinfo: 'https://docs.python.org@169.254.169.254/' parses as a request to 169.254.169.254 while reading as docs.python.org to a human, which is the classic allowlist bypass.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"maxLength\": 4096,"]
#[doc = "  \"pattern\": \"^https?://[a-zA-Z0-9._~%!$&'()*+,;=:\\\\[\\\\]-]+([/?#]|$)\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1SeedsItem(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1SeedsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1SeedsItem> for ::std::string::String {
    fn from(value: VerticalPackV1SeedsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1SeedsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 4096usize {
            return Err("longer than 4096 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^https?://[a-zA-Z0-9._~%!$&'()*+,;=:\\[\\]-]+([/?#]|$)")
                    .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^https?://[a-zA-Z0-9._~%!$&'()*+,;=:\\[\\]-]+([/?#]|$)\""
                    .into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1SeedsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1SeedsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1SeedsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1SeedsItem {
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
#[doc = "URL rejection rules, from policy.toml [url_filters]. Absent means no substring filtering."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"URL rejection rules, from policy.toml [url_filters]. Absent means no substring filtering.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"deny_contains\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"deny_contains\": {"]
#[doc = "      \"description\": \"Drop any URL containing one of these substrings\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"maxLength\": 256,"]
#[doc = "        \"minLength\": 1"]
#[doc = "      },"]
#[doc = "      \"maxItems\": 1000"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct VerticalPackV1UrlFilters {
    #[doc = "Drop any URL containing one of these substrings"]
    pub deny_contains: ::std::vec::Vec<VerticalPackV1UrlFiltersDenyContainsItem>,
}
impl VerticalPackV1UrlFilters {
    pub fn builder() -> builder::VerticalPackV1UrlFilters {
        Default::default()
    }
}
#[doc = "`VerticalPackV1UrlFiltersDenyContainsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"maxLength\": 256,"]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct VerticalPackV1UrlFiltersDenyContainsItem(::std::string::String);
impl ::std::ops::Deref for VerticalPackV1UrlFiltersDenyContainsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<VerticalPackV1UrlFiltersDenyContainsItem> for ::std::string::String {
    fn from(value: VerticalPackV1UrlFiltersDenyContainsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for VerticalPackV1UrlFiltersDenyContainsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() > 256usize {
            return Err("longer than 256 characters".into());
        }
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for VerticalPackV1UrlFiltersDenyContainsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for VerticalPackV1UrlFiltersDenyContainsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for VerticalPackV1UrlFiltersDenyContainsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for VerticalPackV1UrlFiltersDenyContainsItem {
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
    pub struct VerticalPackV1 {
        code: ::std::result::Result<super::VerticalPackV1Code, ::std::string::String>,
        content: ::std::result::Result<super::VerticalPackV1Content, ::std::string::String>,
        hosts: ::std::result::Result<super::VerticalPackV1Hosts, ::std::string::String>,
        limits: ::std::result::Result<super::VerticalPackV1Limits, ::std::string::String>,
        pack: ::std::result::Result<super::VerticalPackV1Pack, ::std::string::String>,
        politeness: ::std::result::Result<super::VerticalPackV1Politeness, ::std::string::String>,
        ranking: ::std::result::Result<super::VerticalPackV1Ranking, ::std::string::String>,
        schema: ::std::result::Result<::std::string::String, ::std::string::String>,
        seeds: ::std::result::Result<
            ::std::vec::Vec<super::VerticalPackV1SeedsItem>,
            ::std::string::String,
        >,
        url_filters: ::std::result::Result<
            ::std::option::Option<super::VerticalPackV1UrlFilters>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VerticalPackV1 {
        fn default() -> Self {
            Self {
                code: Err("no value supplied for code".to_string()),
                content: Err("no value supplied for content".to_string()),
                hosts: Err("no value supplied for hosts".to_string()),
                limits: Err("no value supplied for limits".to_string()),
                pack: Err("no value supplied for pack".to_string()),
                politeness: Err("no value supplied for politeness".to_string()),
                ranking: Err("no value supplied for ranking".to_string()),
                schema: Err("no value supplied for schema".to_string()),
                seeds: Err("no value supplied for seeds".to_string()),
                url_filters: Ok(Default::default()),
            }
        }
    }
    impl VerticalPackV1 {
        pub fn code<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1Code>,
            T::Error: ::std::fmt::Display,
        {
            self.code = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code: {e}"));
            self
        }
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1Content>,
            T::Error: ::std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {e}"));
            self
        }
        pub fn hosts<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1Hosts>,
            T::Error: ::std::fmt::Display,
        {
            self.hosts = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hosts: {e}"));
            self
        }
        pub fn limits<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1Limits>,
            T::Error: ::std::fmt::Display,
        {
            self.limits = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for limits: {e}"));
            self
        }
        pub fn pack<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1Pack>,
            T::Error: ::std::fmt::Display,
        {
            self.pack = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pack: {e}"));
            self
        }
        pub fn politeness<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1Politeness>,
            T::Error: ::std::fmt::Display,
        {
            self.politeness = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for politeness: {e}"));
            self
        }
        pub fn ranking<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1Ranking>,
            T::Error: ::std::fmt::Display,
        {
            self.ranking = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ranking: {e}"));
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
        pub fn seeds<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VerticalPackV1SeedsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.seeds = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for seeds: {e}"));
            self
        }
        pub fn url_filters<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::VerticalPackV1UrlFilters>>,
            T::Error: ::std::fmt::Display,
        {
            self.url_filters = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url_filters: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1> for super::VerticalPackV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                code: value.code?,
                content: value.content?,
                hosts: value.hosts?,
                limits: value.limits?,
                pack: value.pack?,
                politeness: value.politeness?,
                ranking: value.ranking?,
                schema: value.schema?,
                seeds: value.seeds?,
                url_filters: value.url_filters?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1> for VerticalPackV1 {
        fn from(value: super::VerticalPackV1) -> Self {
            Self {
                code: Ok(value.code),
                content: Ok(value.content),
                hosts: Ok(value.hosts),
                limits: Ok(value.limits),
                pack: Ok(value.pack),
                politeness: Ok(value.politeness),
                ranking: Ok(value.ranking),
                schema: Ok(value.schema),
                seeds: Ok(value.seeds),
                url_filters: Ok(value.url_filters),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1Code {
        preserve_selectors: ::std::result::Result<
            ::std::vec::Vec<super::VerticalPackV1CodePreserveSelectorsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VerticalPackV1Code {
        fn default() -> Self {
            Self {
                preserve_selectors: Err("no value supplied for preserve_selectors".to_string()),
            }
        }
    }
    impl VerticalPackV1Code {
        pub fn preserve_selectors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::VerticalPackV1CodePreserveSelectorsItem>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.preserve_selectors = value.try_into().map_err(|e| {
                format!("error converting supplied value for preserve_selectors: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1Code> for super::VerticalPackV1Code {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1Code,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                preserve_selectors: value.preserve_selectors?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1Code> for VerticalPackV1Code {
        fn from(value: super::VerticalPackV1Code) -> Self {
            Self {
                preserve_selectors: Ok(value.preserve_selectors),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1Content {
        main_selectors: ::std::result::Result<
            ::std::vec::Vec<super::VerticalPackV1ContentMainSelectorsItem>,
            ::std::string::String,
        >,
        title_selectors: ::std::result::Result<
            ::std::vec::Vec<super::VerticalPackV1ContentTitleSelectorsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VerticalPackV1Content {
        fn default() -> Self {
            Self {
                main_selectors: Err("no value supplied for main_selectors".to_string()),
                title_selectors: Err("no value supplied for title_selectors".to_string()),
            }
        }
    }
    impl VerticalPackV1Content {
        pub fn main_selectors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::VerticalPackV1ContentMainSelectorsItem>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.main_selectors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for main_selectors: {e}"));
            self
        }
        pub fn title_selectors<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::VerticalPackV1ContentTitleSelectorsItem>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.title_selectors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title_selectors: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1Content> for super::VerticalPackV1Content {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1Content,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                main_selectors: value.main_selectors?,
                title_selectors: value.title_selectors?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1Content> for VerticalPackV1Content {
        fn from(value: super::VerticalPackV1Content) -> Self {
            Self {
                main_selectors: Ok(value.main_selectors),
                title_selectors: Ok(value.title_selectors),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1Hosts {
        allowed: ::std::result::Result<
            ::std::vec::Vec<super::VerticalPackV1HostsAllowedItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VerticalPackV1Hosts {
        fn default() -> Self {
            Self {
                allowed: Err("no value supplied for allowed".to_string()),
            }
        }
    }
    impl VerticalPackV1Hosts {
        pub fn allowed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::VerticalPackV1HostsAllowedItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.allowed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for allowed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1Hosts> for super::VerticalPackV1Hosts {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1Hosts,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                allowed: value.allowed?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1Hosts> for VerticalPackV1Hosts {
        fn from(value: super::VerticalPackV1Hosts) -> Self {
            Self {
                allowed: Ok(value.allowed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1Limits {
        max_body_bytes: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        max_depth: ::std::result::Result<i64, ::std::string::String>,
        max_pages: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
    }
    impl ::std::default::Default for VerticalPackV1Limits {
        fn default() -> Self {
            Self {
                max_body_bytes: Err("no value supplied for max_body_bytes".to_string()),
                max_depth: Err("no value supplied for max_depth".to_string()),
                max_pages: Err("no value supplied for max_pages".to_string()),
            }
        }
    }
    impl VerticalPackV1Limits {
        pub fn max_body_bytes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_body_bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_body_bytes: {e}"));
            self
        }
        pub fn max_depth<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_depth = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_depth: {e}"));
            self
        }
        pub fn max_pages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.max_pages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_pages: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1Limits> for super::VerticalPackV1Limits {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1Limits,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                max_body_bytes: value.max_body_bytes?,
                max_depth: value.max_depth?,
                max_pages: value.max_pages?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1Limits> for VerticalPackV1Limits {
        fn from(value: super::VerticalPackV1Limits) -> Self {
            Self {
                max_body_bytes: Ok(value.max_body_bytes),
                max_depth: Ok(value.max_depth),
                max_pages: Ok(value.max_pages),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1Pack {
        id: ::std::result::Result<super::VerticalPackV1PackId, ::std::string::String>,
        name: ::std::result::Result<super::VerticalPackV1PackName, ::std::string::String>,
        tab_label: ::std::result::Result<super::VerticalPackV1PackTabLabel, ::std::string::String>,
    }
    impl ::std::default::Default for VerticalPackV1Pack {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                name: Err("no value supplied for name".to_string()),
                tab_label: Err("no value supplied for tab_label".to_string()),
            }
        }
    }
    impl VerticalPackV1Pack {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1PackId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1PackName>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn tab_label<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::VerticalPackV1PackTabLabel>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_label = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for tab_label: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1Pack> for super::VerticalPackV1Pack {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1Pack,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                id: value.id?,
                name: value.name?,
                tab_label: value.tab_label?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1Pack> for VerticalPackV1Pack {
        fn from(value: super::VerticalPackV1Pack) -> Self {
            Self {
                id: Ok(value.id),
                name: Ok(value.name),
                tab_label: Ok(value.tab_label),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1Politeness {
        default_rps_per_host: ::std::result::Result<f64, ::std::string::String>,
        respect_crawl_delay: ::std::result::Result<bool, ::std::string::String>,
        respect_robots: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for VerticalPackV1Politeness {
        fn default() -> Self {
            Self {
                default_rps_per_host: Err("no value supplied for default_rps_per_host".to_string()),
                respect_crawl_delay: Err("no value supplied for respect_crawl_delay".to_string()),
                respect_robots: Err("no value supplied for respect_robots".to_string()),
            }
        }
    }
    impl VerticalPackV1Politeness {
        pub fn default_rps_per_host<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.default_rps_per_host = value.try_into().map_err(|e| {
                format!("error converting supplied value for default_rps_per_host: {e}")
            });
            self
        }
        pub fn respect_crawl_delay<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.respect_crawl_delay = value.try_into().map_err(|e| {
                format!("error converting supplied value for respect_crawl_delay: {e}")
            });
            self
        }
        pub fn respect_robots<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.respect_robots = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for respect_robots: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1Politeness> for super::VerticalPackV1Politeness {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1Politeness,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                default_rps_per_host: value.default_rps_per_host?,
                respect_crawl_delay: value.respect_crawl_delay?,
                respect_robots: value.respect_robots?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1Politeness> for VerticalPackV1Politeness {
        fn from(value: super::VerticalPackV1Politeness) -> Self {
            Self {
                default_rps_per_host: Ok(value.default_rps_per_host),
                respect_crawl_delay: Ok(value.respect_crawl_delay),
                respect_robots: Ok(value.respect_robots),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1Ranking {
        anchor_weight: ::std::result::Result<f64, ::std::string::String>,
        bm25_b: ::std::result::Result<f64, ::std::string::String>,
        bm25_k1: ::std::result::Result<f64, ::std::string::String>,
        body_weight: ::std::result::Result<f64, ::std::string::String>,
        code_weight: ::std::result::Result<f64, ::std::string::String>,
        recency_half_life_days: ::std::result::Result<f64, ::std::string::String>,
        title_weight: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for VerticalPackV1Ranking {
        fn default() -> Self {
            Self {
                anchor_weight: Err("no value supplied for anchor_weight".to_string()),
                bm25_b: Err("no value supplied for bm25_b".to_string()),
                bm25_k1: Err("no value supplied for bm25_k1".to_string()),
                body_weight: Err("no value supplied for body_weight".to_string()),
                code_weight: Err("no value supplied for code_weight".to_string()),
                recency_half_life_days: Err(
                    "no value supplied for recency_half_life_days".to_string()
                ),
                title_weight: Err("no value supplied for title_weight".to_string()),
            }
        }
    }
    impl VerticalPackV1Ranking {
        pub fn anchor_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.anchor_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for anchor_weight: {e}"));
            self
        }
        pub fn bm25_b<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bm25_b = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bm25_b: {e}"));
            self
        }
        pub fn bm25_k1<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.bm25_k1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bm25_k1: {e}"));
            self
        }
        pub fn body_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.body_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for body_weight: {e}"));
            self
        }
        pub fn code_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.code_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for code_weight: {e}"));
            self
        }
        pub fn recency_half_life_days<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.recency_half_life_days = value.try_into().map_err(|e| {
                format!("error converting supplied value for recency_half_life_days: {e}")
            });
            self
        }
        pub fn title_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.title_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title_weight: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1Ranking> for super::VerticalPackV1Ranking {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1Ranking,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                anchor_weight: value.anchor_weight?,
                bm25_b: value.bm25_b?,
                bm25_k1: value.bm25_k1?,
                body_weight: value.body_weight?,
                code_weight: value.code_weight?,
                recency_half_life_days: value.recency_half_life_days?,
                title_weight: value.title_weight?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1Ranking> for VerticalPackV1Ranking {
        fn from(value: super::VerticalPackV1Ranking) -> Self {
            Self {
                anchor_weight: Ok(value.anchor_weight),
                bm25_b: Ok(value.bm25_b),
                bm25_k1: Ok(value.bm25_k1),
                body_weight: Ok(value.body_weight),
                code_weight: Ok(value.code_weight),
                recency_half_life_days: Ok(value.recency_half_life_days),
                title_weight: Ok(value.title_weight),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct VerticalPackV1UrlFilters {
        deny_contains: ::std::result::Result<
            ::std::vec::Vec<super::VerticalPackV1UrlFiltersDenyContainsItem>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for VerticalPackV1UrlFilters {
        fn default() -> Self {
            Self {
                deny_contains: Err("no value supplied for deny_contains".to_string()),
            }
        }
    }
    impl VerticalPackV1UrlFilters {
        pub fn deny_contains<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::vec::Vec<super::VerticalPackV1UrlFiltersDenyContainsItem>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.deny_contains = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deny_contains: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<VerticalPackV1UrlFilters> for super::VerticalPackV1UrlFilters {
        type Error = super::error::ConversionError;
        fn try_from(
            value: VerticalPackV1UrlFilters,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                deny_contains: value.deny_contains?,
            })
        }
    }
    impl ::std::convert::From<super::VerticalPackV1UrlFilters> for VerticalPackV1UrlFilters {
        fn from(value: super::VerticalPackV1UrlFilters) -> Self {
            Self {
                deny_contains: Ok(value.deny_contains),
            }
        }
    }
}
