package pack

// TOML-shaped intermediate types, one per file on disk.
//
// These exist because the generated contract type carries json/yaml/
// mapstructure tags but no toml tag, and BurntSushi matches on a toml tag or a
// case-insensitive field name — `MaxPages` never matches `max_pages`. They are
// also the honest shape: the contract is one merged document, the pack is
// three files plus a seed list.
//
// Every field is a pointer or slice where absence must be distinguishable from
// a zero value. A missing `max_depth` has to reach the schema as *missing* so
// validation rejects it, rather than silently arriving as a legitimate-looking
// depth of 0.

type packFile struct {
	Pack    *packSection    `toml:"pack"`
	Ranking *rankingSection `toml:"ranking"`
}

type packSection struct {
	ID       *string `toml:"id"`
	Name     *string `toml:"name"`
	TabLabel *string `toml:"tab_label"`
}

type rankingSection struct {
	TitleWeight         *float64 `toml:"title_weight"`
	BodyWeight          *float64 `toml:"body_weight"`
	CodeWeight          *float64 `toml:"code_weight"`
	AnchorWeight        *float64 `toml:"anchor_weight"`
	RecencyHalfLifeDays *float64 `toml:"recency_half_life_days"`
	Bm25K1              *float64 `toml:"bm25_k1"`
	Bm25B               *float64 `toml:"bm25_b"`
}

type policyFile struct {
	Limits     *limitsSection     `toml:"limits"`
	Politeness *politenessSection `toml:"politeness"`
	Hosts      *hostsSection      `toml:"hosts"`
	URLFilters *urlFiltersSection `toml:"url_filters"`
}

type limitsSection struct {
	MaxPages     *int `toml:"max_pages"`
	MaxDepth     *int `toml:"max_depth"`
	MaxBodyBytes *int `toml:"max_body_bytes"`
}

type politenessSection struct {
	DefaultRpsPerHost *float64 `toml:"default_rps_per_host"`
	RespectRobots     *bool    `toml:"respect_robots"`
	RespectCrawlDelay *bool    `toml:"respect_crawl_delay"`
}

type hostsSection struct {
	Allowed []string `toml:"allowed"`
}

type urlFiltersSection struct {
	DenyContains []string `toml:"deny_contains"`
}

type extractFile struct {
	Content *contentSection `toml:"content"`
	Code    *codeSection    `toml:"code"`
}

type contentSection struct {
	MainSelectors  []string `toml:"main_selectors"`
	TitleSelectors []string `toml:"title_selectors"`
}

type codeSection struct {
	PreserveSelectors []string `toml:"preserve_selectors"`
}
