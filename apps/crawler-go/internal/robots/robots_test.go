package robots

import (
	"strings"
	"testing"
	"time"
)

const loomBot = "LoomBot/0.1 (+https://github.com/samibk/loom)"

// The fixture site's own robots.txt. If this stops working, `make crawl-fixture`
// starts fetching /private/ and the politeness assertion in the E2E is a lie.
const fixtureRobots = "User-agent: *\nDisallow: /private/\nCrawl-delay: 1\nSitemap: /sitemap.xml\n"

func TestAllowed(t *testing.T) {
	tests := []struct {
		name string
		body string
		path string
		want bool
	}{
		{"fixture site allows a doc page", fixtureRobots, "/js/array-map.html", true},
		{"fixture site disallows the private section", fixtureRobots, "/private/secret.html", false},
		{"fixture site allows the root", fixtureRobots, "/", true},

		{"no rules means allowed", "User-agent: *\n", "/anything", true},
		{"empty file means allowed", "", "/anything", true},
		{"empty disallow means allow everything", "User-agent: *\nDisallow:\n", "/anything", true},
		{"disallow slash blocks everything", "User-agent: *\nDisallow: /\n", "/x", false},

		// Longest match wins, and Allow wins an exact tie (§2.2.2)
		{
			name: "allow carves an exception out of a disallow",
			body: "User-agent: *\nDisallow: /docs/\nAllow: /docs/public/\n",
			path: "/docs/public/x.html", want: true,
		},
		{
			name: "the disallow still covers the rest",
			body: "User-agent: *\nDisallow: /docs/\nAllow: /docs/public/\n",
			path: "/docs/private/x.html", want: false,
		},
		{
			name: "exact tie goes to allow",
			body: "User-agent: *\nDisallow: /x\nAllow: /x\n",
			path: "/x", want: true,
		},

		// Wildcards (§2.2.3)
		{"star matches any run", "User-agent: *\nDisallow: /*/private/\n", "/a/private/x", false},
		{"dollar anchors to end", "User-agent: *\nDisallow: /*.pdf$\n", "/docs/a.pdf", false},
		{"dollar does not match mid-path", "User-agent: *\nDisallow: /*.pdf$\n", "/docs/a.pdf.html", true},

		// Group selection: most specific agent wins, not the first (§2.2.1)
		{
			name: "specific agent group beats the wildcard group",
			body: "User-agent: *\nDisallow: /\n\nUser-agent: LoomBot\nDisallow: /private/\n",
			path: "/js/x.html", want: true,
		},
		{
			name: "and its own rules still apply",
			body: "User-agent: *\nDisallow: /\n\nUser-agent: LoomBot\nDisallow: /private/\n",
			path: "/private/x", want: false,
		},
		{
			name: "agent match is case-insensitive",
			body: "User-agent: loombot\nDisallow: /private/\n",
			path: "/private/x", want: false,
		},
		{
			name: "longest matching agent wins",
			body: "User-agent: Loom\nDisallow: /\n\nUser-agent: LoomBot\nAllow: /\n",
			path: "/x", want: true,
		},

		// Robustness (§2.2.1) — malformed input must not lose the valid rules
		{"comments stripped", "User-agent: * # everyone\nDisallow: /private/ # secret\n", "/private/x", false},
		{"CRLF tolerated", "User-agent: *\r\nDisallow: /private/\r\n", "/private/x", false},
		{"BOM tolerated", "\ufeffUser-agent: *\nDisallow: /private/\n", "/private/x", false},
		{"field names case-insensitive", "USER-AGENT: *\nDISALLOW: /private/\n", "/private/x", false},
		{"unknown fields ignored", "User-agent: *\nSitemap: /s.xml\nDisallow: /private/\n", "/private/x", false},
		{"malformed line skipped, rest survives", "User-agent: *\nthis has no colon\nDisallow: /private/\n", "/private/x", false},
		{"rules before any user-agent are discarded", "Disallow: /\nUser-agent: *\nAllow: /\n", "/x", true},

		// Query strings are part of the path for matching purposes
		{"query included in match", "User-agent: *\nDisallow: /search?\n", "/search?q=x", false},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			rules, err := Parse([]byte(tc.body), loomBot)
			if err != nil {
				t.Fatalf("Parse: %v", err)
			}
			if got := rules.Allowed(tc.path); got != tc.want {
				t.Errorf("Allowed(%q) = %v, want %v\n--- robots.txt ---\n%s", tc.path, got, tc.want, tc.body)
			}
		})
	}
}

func TestCrawlDelay(t *testing.T) {
	tests := []struct {
		name string
		body string
		want time.Duration
	}{
		{"absent means zero", "User-agent: *\nDisallow: /x\n", 0},
		{"whole seconds", fixtureRobots, time.Second},
		{"fractional seconds", "User-agent: *\nCrawl-delay: 0.5\n", 500 * time.Millisecond},
		{"unparseable ignored", "User-agent: *\nCrawl-delay: soon\n", 0},
		{"read from the winning group only", "User-agent: *\nCrawl-delay: 30\n\nUser-agent: LoomBot\nCrawl-delay: 2\n", 2 * time.Second},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			rules, err := Parse([]byte(tc.body), loomBot)
			if err != nil {
				t.Fatalf("Parse: %v", err)
			}
			if got := rules.CrawlDelay(); got != tc.want {
				t.Errorf("CrawlDelay() = %v, want %v", got, tc.want)
			}
		})
	}
}

// A robots.txt is fetched from a third party and can be anything at all.
// Parsing must never panic — a crash here takes the whole crawl down.
func TestParseIsTotal(t *testing.T) {
	nasty := [][]byte{
		nil, {}, {0x00}, {0xff, 0xfe, 0xfd},
		[]byte(":"), []byte("\n\n\n"), []byte("User-agent:"),
		[]byte("User-agent: *\nDisallow"),
		[]byte(strings.Repeat("User-agent: *\n", 10000)),
		[]byte(strings.Repeat("a", 1<<20)),
		[]byte("Disallow: /" + strings.Repeat("*", 1000)),
	}

	for i, body := range nasty {
		rules, err := Parse(body, loomBot)
		if err != nil {
			continue
		}
		if rules == nil {
			t.Errorf("case %d: nil Rules with nil error", i)
			continue
		}
		rules.Allowed("/x") // must not panic either
	}
}
