// Package robots parses robots.txt and answers whether a path may be fetched.
//
// This is the one part of the crawler where being wrong is a wrong done to
// someone else. A bug that over-fetches is not a performance problem, it is a
// crawler ignoring a site owner's stated wishes — so the parser fails closed:
// anything it cannot understand is treated as more restrictive, never less.
package robots

import "time"

// Rules is the parsed robots.txt as it applies to one user-agent.
//
// Only the winning group's rules are retained. Which group won is decided at
// parse time because it depends on the agent, and keeping every group would
// invite a later caller to consult the wrong one.
type Rules struct {
	// allow and disallow hold path prefixes from the winning group, in file
	// order. Longest-match-wins needs the full set, not just the first hit.
	allow    []string
	disallow []string

	// crawlDelay is the group's Crawl-delay, or zero when it stated none.
	crawlDelay time.Duration
}

// Parse reads a robots.txt body and returns the rules for userAgent.
//
// TODO(you): implement.
//
// Algorithm (RFC 9309, "Robots Exclusion Protocol"):
//
//  1. Decode as UTF-8, tolerating a leading BOM. Split into lines on \n,
//     tolerating \r\n.
//  2. Strip comments: everything from an unquoted '#' to end of line (§2.2.1).
//     Trim surrounding whitespace. Skip lines that are then empty.
//  3. Split each line on the first ':' into field and value. A line without a
//     ':' is malformed — skip it, do not abort the file (§2.2.1).
//  4. Field names are case-insensitive. The three that matter: user-agent,
//     allow, disallow. Ignore any other field, including sitemap, which is not
//     scoped to a group and which P2 handles separately.
//  5. Group the file: consecutive user-agent lines start or extend a group;
//     the first allow/disallow line after them closes the agent list and begins
//     the group's rules. A rule line appearing before any user-agent line
//     belongs to no group and is discarded (§2.2.1).
//  6. Select the winning group by the MOST SPECIFIC match, not the first:
//     compare userAgent case-insensitively against each group's agents, and
//     prefer the longest agent string that is a prefix of userAgent. "*"
//     matches everything and is the fallback. Exactly one group wins; the
//     others are irrelevant (§2.2.1).
//  7. Within the winning group, collect allow and disallow values. An empty
//     Disallow value means "allow everything" and must not be stored as a
//     prefix that matches every path (§2.2.2).
//  8. Read Crawl-delay if present. It is not in RFC 9309, but it is widely
//     honored and policy.toml has respect_crawl_delay. Parse as seconds,
//     accepting a fractional value; ignore a value that will not parse.
//
// Invariants:
//   - Total: arbitrary bytes never panic. A truncated or binary robots.txt
//     yields whatever prefix parsed, not an error.
//   - Fails closed: when the file is unparseable in a way that loses rules,
//     prefer returning rules that disallow more.
//   - Absent from this function: what to do when robots.txt is unreachable.
//     That is the fetcher's decision (see the note on Allowed).
func Parse(body []byte, userAgent string) (*Rules, error) {
	panic("TODO(you): implement Parse — see the recipe above")
}

// Allowed reports whether path may be fetched.
//
// TODO(you): implement.
//
// Algorithm (RFC 9309 §2.2.2):
//
//  1. path is a path plus optional query, e.g. "/docs/x.html?q=1". It is
//     never a full URL — the caller has already canonicalized and split it.
//  2. Find the longest allow prefix and the longest disallow prefix that match
//     path. Matching is on raw octets after percent-decoding both sides
//     consistently.
//  3. Support the two wildcards: '*' matches any run of characters, '$'
//     anchors to end of path. "/*.pdf$" disallows PDFs anywhere.
//  4. The longer match wins. On an exact tie, ALLOW wins — that is what makes
//     "Disallow: /docs/" plus "Allow: /docs/public/" behave as intended.
//  5. No matching rule at all means allowed. robots.txt is a denylist.
//
// Note on the missing case: a 404 robots.txt means "everything is allowed"
// and a 5xx means "nothing is allowed until it recovers" (§2.3.1.3). That is
// the FETCHER's job, not this function's — Rules describes a file that was
// successfully read.
func (r *Rules) Allowed(path string) bool {
	panic("TODO(you): implement Allowed — see the recipe above")
}

// CrawlDelay returns the group's Crawl-delay, or zero when it stated none.
//
// TODO(you): implement. This is a trivial accessor; it is a stub only so the
// zero-value semantics are yours to decide alongside the parser.
//
// The caller combines this with policy.toml's default_rps_per_host and takes
// whichever is MORE conservative — a site asking for a 10-second delay must
// get 10 seconds even when our own budget would permit one per second.
func (r *Rules) CrawlDelay() time.Duration {
	panic("TODO(you): implement CrawlDelay — see the recipe above")
}
