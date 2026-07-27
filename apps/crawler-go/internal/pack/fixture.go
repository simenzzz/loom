package pack

import (
	"fmt"
	"net/url"
	"slices"
	"strings"

	"loom/crawler/internal/contracts/gen"
)

// fixtureSeedPath is the fixture site's entry point. Every other page is
// reachable from it by link, which is the point: the fixture crawl exercises
// real link discovery rather than being handed a page list.
const fixtureSeedPath = "/index.html"

// FixtureOverride returns a copy of p aimed at the local fixture site instead
// of the real corpus.
//
// It rewrites exactly two things — the seed list and the host allowlist — and
// returns a new pack; p is never modified. Everything else is deliberately
// left alone, especially politeness.respect_robots: the fixture site serves a
// robots.txt that disallows /private/, and a crawl that fetched it anyway
// would be a politeness bug. Keeping the flag true is what gives the E2E
// assertion ("no segment record names /private/secret.html") its meaning.
//
// baseURL is the origin the fixture site is reachable at, e.g.
// http://localhost:7799 locally or http://fixture-site under compose. It is
// passed in rather than read from configuration so this stays a pure function;
// the CLI owns that lookup, and should gate the whole override behind an
// explicit fixture mode rather than the mere presence of a value.
func FixtureOverride(p *gen.VerticalPackV1, baseURL string) (*gen.VerticalPackV1, error) {
	if p == nil {
		return nil, fmt.Errorf("pack: FixtureOverride called with nil pack")
	}

	base, err := url.Parse(strings.TrimSpace(baseURL))
	if err != nil {
		return nil, fmt.Errorf("pack: fixture base %s is not a parseable URL: %w",
			truncate(baseURL), err)
	}
	if base.Scheme != "http" && base.Scheme != "https" {
		return nil, fmt.Errorf("pack: fixture base %s must be http or https, got %q",
			truncate(baseURL), base.Scheme)
	}
	if base.Path != "" && base.Path != "/" {
		return nil, fmt.Errorf("pack: fixture base %s must be an origin with no path, got %q",
			truncate(baseURL), base.Path)
	}
	// Reject rather than silently discard. The origin is rebuilt from scheme and
	// host below, so any of these would vanish without trace — and an operator
	// debugging why their credentials or query string had no effect would have
	// nothing to go on.
	if base.User != nil || base.RawQuery != "" || base.Fragment != "" || base.Opaque != "" {
		return nil, fmt.Errorf("pack: fixture base %s must be a bare origin: "+
			"userinfo, query and fragment are not permitted", truncate(baseURL))
	}
	if strings.Contains(base.Hostname(), ":") {
		return nil, fmt.Errorf("pack: fixture base %s is an IPv6 literal, which cannot be "+
			"expressed in hosts.allowed; use a name", truncate(baseURL))
	}
	host, err := NormalizeHost(base.Hostname())
	if err != nil {
		return nil, fmt.Errorf("pack: fixture base %s: %w", truncate(baseURL), err)
	}
	if host == "" {
		return nil, fmt.Errorf("pack: fixture base %s has no host", truncate(baseURL))
	}
	// A fixture site is loopback or a compose service name. Without this, a
	// function whose stated purpose is "aim at the local fixture site" will
	// cheerfully aim at http://169.254.169.254 — and the verify() call below
	// cannot object, because it would be checking the seed against an allowlist
	// derived from the same input.
	if !isLoopbackHost(host) && strings.Contains(host, ".") {
		return nil, fmt.Errorf("pack: fixture base %s must be loopback or a dot-free "+
			"service name; %q is neither", truncate(baseURL), host)
	}

	// Rebuild the origin from normalized parts rather than reusing
	// base.String(): url.Parse preserves the host's original case, so a base of
	// http://LOCALHOST:7799 would yield a seed whose host does not match the
	// lowercased allowlist entry beside it.
	origin := url.URL{Scheme: base.Scheme, Host: host}
	if port := base.Port(); port != "" {
		origin.Host = host + ":" + port
	}

	out := Clone(p)
	out.Hosts.Allowed = []string{host}
	out.Seeds = []string{origin.String() + fixtureSeedPath}

	if err := validate(out); err != nil {
		return nil, fmt.Errorf("pack: fixture override for %s: %w", truncate(baseURL), err)
	}
	// Empty dir: the override is not tied to a directory, so the pack.id rule
	// does not apply — the id is inherited from the pack that was loaded, and
	// was already checked against its own directory then.
	if err := verify(out, ""); err != nil {
		return nil, fmt.Errorf("pack: fixture override for %s: %w", truncate(baseURL), err)
	}
	return out, nil
}

// Clone deep-copies a pack. Every slice is reallocated, so a mutation of the
// copy can never reach the original — callers hold onto a loaded pack and pass
// it around, and a shared backing array turns one caller's edit into another
// caller's bug.
//
// Exported because Load hands out a mutable pack: anything that wants to adjust
// one must copy first to honor the house immutability rule.
//
// A new slice or pointer field in the schema would regenerate the struct, keep
// compiling, and silently alias. TestCloneCopiesEveryReferenceField walks the
// type reflectively to catch exactly that.
func Clone(p *gen.VerticalPackV1) *gen.VerticalPackV1 {
	out := *p // scalars and nested structs of scalars copy by value

	out.Seeds = slices.Clone(p.Seeds)
	out.Hosts.Allowed = slices.Clone(p.Hosts.Allowed)
	out.Content.MainSelectors = slices.Clone(p.Content.MainSelectors)
	out.Content.TitleSelectors = slices.Clone(p.Content.TitleSelectors)
	out.Code.PreserveSelectors = slices.Clone(p.Code.PreserveSelectors)

	if p.UrlFilters != nil {
		out.UrlFilters = &gen.VerticalPackV1UrlFilters{
			DenyContains: slices.Clone(p.UrlFilters.DenyContains),
		}
	}
	return &out
}
