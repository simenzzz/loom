package pack

import (
	"fmt"
	"net"
	"net/url"
	"path/filepath"
	"strings"

	"loom/crawler/internal/contracts/gen"
)

// defaultPorts are the ports a scheme implies. A seed naming any other port on
// a non-loopback host is refused: hosts.allowed has no port dimension, so
// allowlisting a host would otherwise authorize every service on it.
var defaultPorts = map[string]string{"http": "80", "https": "443"}

// maxEchoedValue caps how much of an offending input an error message repeats.
// Pack files are operator-supplied but may be symlinks to something else, and
// an error string can travel further than the file ever would.
const maxEchoedValue = 120

// verify enforces the rules the contract states but JSON Schema cannot express,
// because they relate two fields to each other rather than constraining one.
//
// Callers must have validated the pack against the contract first: verify
// assumes the schema has already established shape, scheme and pattern.
func verify(p *gen.VerticalPackV1, dir string) error {
	if err := verifySeedsWithinAllowlist(p); err != nil {
		return err
	}
	if err := verifyRobotsPolicy(p); err != nil {
		return err
	}
	return verifyPackIDMatchesDir(p, dir)
}

// verifySeedsWithinAllowlist rejects any seed whose host is not in
// hosts.allowed, or which names a non-default port on a non-loopback host.
//
// This matters more than it looks. Seeds are depth 0 — they are the frontier's
// initial contents, so a crawler that trusts them because "they came from
// config" will fetch them without ever consulting the allowlist. That makes an
// unchecked seed a complete allowlist bypass, and seeds.txt is a data file that
// attracts far less review than code.
func verifySeedsWithinAllowlist(p *gen.VerticalPackV1) error {
	if len(p.Hosts.Allowed) == 0 {
		return fmt.Errorf("hosts.allowed is empty; a pack must name at least one host")
	}

	allowed := make(map[string]bool, len(p.Hosts.Allowed))
	for _, h := range p.Hosts.Allowed {
		norm, err := NormalizeHost(h)
		if err != nil {
			return fmt.Errorf("hosts.allowed entry %s: %w", truncate(h), err)
		}
		allowed[norm] = true
	}

	for i, seed := range p.Seeds {
		u, err := url.Parse(seed)
		if err != nil {
			return fmt.Errorf("seeds[%d] %s is not a parseable URL: %w", i, truncate(seed), err)
		}
		host, err := NormalizeHost(u.Hostname())
		if err != nil {
			return fmt.Errorf("seeds[%d] %s: %w", i, truncate(seed), err)
		}
		if host == "" {
			return fmt.Errorf("seeds[%d] %s has no host", i, truncate(seed))
		}
		if !allowed[host] {
			return fmt.Errorf(
				"seeds[%d] %s targets host %q, which is not in hosts.allowed %v — "+
					"seeds enter the frontier at depth 0, so an unlisted seed bypasses "+
					"the allowlist entirely",
				i, truncate(seed), host, p.Hosts.Allowed)
		}
		if err := verifySeedPort(i, seed, u, host); err != nil {
			return err
		}
	}
	return nil
}

// verifySeedPort refuses a seed that names a non-default port on a host that is
// not loopback.
//
// hosts.allowed cannot express a port, so allowlisting "localhost" would
// otherwise authorize http://localhost:6379/ — and Redis parses an HTTP request
// line as an inline command. The loopback exemption exists because the fixture
// site legitimately runs on :7799.
func verifySeedPort(i int, seed string, u *url.URL, host string) error {
	port := u.Port()
	if port == "" || port == defaultPorts[u.Scheme] || isLoopbackHost(host) {
		return nil
	}
	return fmt.Errorf(
		"seeds[%d] %s names port %s on non-loopback host %q — hosts.allowed cannot "+
			"express a port, so allowlisting a host must not authorize every service on it",
		i, truncate(seed), port, host)
}

// verifyRobotsPolicy refuses respect_robots=false unless every allowed host is
// loopback.
//
// The fixture site legitimately needs the escape hatch. A third party's site
// never does: ignoring its robots.txt is not a configuration choice, it is a
// defect, and the schema cannot tell the two cases apart because the answer
// depends on hosts.allowed.
//
// Note this is narrower than a compose deployment might want — the fixture host
// there is the service name "fixture-site", which is not loopback. That is
// deliberate: no pack in the tree sets this false, and a stricter rule costs
// nothing until one does. The CLI is responsible for logging a warning if a
// pack ever loads with robots disabled.
func verifyRobotsPolicy(p *gen.VerticalPackV1) error {
	if p.Politeness.RespectRobots {
		return nil
	}
	if len(p.Hosts.Allowed) == 0 {
		return fmt.Errorf("politeness.respect_robots=false with an empty hosts.allowed")
	}
	for _, h := range p.Hosts.Allowed {
		norm, err := NormalizeHost(h)
		if err != nil {
			return fmt.Errorf("hosts.allowed entry %s: %w", truncate(h), err)
		}
		if !isLoopbackHost(norm) {
			return fmt.Errorf(
				"politeness.respect_robots=false is only permitted when every allowed "+
					"host is loopback; %q is not", h)
		}
	}
	return nil
}

// verifyPackIDMatchesDir rejects a pack whose id disagrees with its directory.
//
// The id names the vertical in every downstream document — segment paths, the
// vertical field on every CrawlRecord, the UI tab — so a pack in
// verticals/devdocs/ declaring id="recipes" would silently write its output
// into another vertical's tree.
func verifyPackIDMatchesDir(p *gen.VerticalPackV1, dir string) error {
	if dir == "" {
		return nil // caller has no directory to compare against (see FixtureOverride)
	}
	want := filepath.Base(filepath.Clean(dir))
	if p.Pack.Id != want {
		return fmt.Errorf(
			"pack.id is %q but the directory is %q — the id names the vertical in "+
				"every downstream document, so the two must agree",
			p.Pack.Id, want)
	}
	return nil
}

// HostAllowed reports whether host may be fetched under this pack.
//
// Exported because the frontier must reach the identical verdict as the loader.
// Two implementations of one allowlist comparison in a single binary is the bug
// this prevents: a seed accepted at load and dropped at depth 0, or worse, a
// host the loader would have refused being accepted later.
func HostAllowed(p *gen.VerticalPackV1, host string) bool {
	norm, err := NormalizeHost(host)
	if err != nil || norm == "" {
		return false
	}
	for _, h := range p.Hosts.Allowed {
		if allowed, err := NormalizeHost(h); err == nil && allowed == norm {
			return true
		}
	}
	return false
}

// NormalizeHost puts a host into the form used for allowlist comparison:
// ASCII-lowercased, with the root-anchoring trailing dot removed. "EXAMPLE.com."
// and "example.com" address the same server and must compare equal.
//
// Non-ASCII input is an error rather than being folded. strings.ToLower applies
// Unicode case folding, under which U+0130 ('İ') becomes 'i' and U+212A ('K')
// becomes 'k' — so "developer.mozİlla.org" would normalize *into*
// "developer.mozilla.org", match the allowlist, and then resolve to
// developer.xn--mozilla-9he.org: a different, registerable domain. Since every
// hosts.allowed entry is ASCII by contract, a non-ASCII host can never legitimately
// match one, and refusing it costs nothing. Callers with an internationalized
// host must convert to punycode first.
//
// Ports are already absent: callers pass either a hosts.allowed entry, whose
// contract pattern excludes ':', or url.URL.Hostname, which strips the port and
// the brackets around an IPv6 literal.
func NormalizeHost(host string) (string, error) {
	h := strings.TrimSpace(host)
	for i := 0; i < len(h); i++ {
		if h[i] >= 0x80 {
			return "", fmt.Errorf(
				"host %s contains a non-ASCII byte; hosts must be given in punycode "+
					"(xn--) form, because Unicode case folding can map a distinct domain "+
					"onto an allowlisted one", truncate(host))
		}
	}
	// TrimSuffix, not TrimRight: exactly one root-anchoring dot is legitimate.
	// "go.dev.." keeps a dot, fails the allowlist comparison, and is refused —
	// which is the correct outcome for a malformed host.
	return strings.TrimSuffix(asciiLower(h), "."), nil
}

// asciiLower lowercases A-Z and leaves every other byte alone. See NormalizeHost
// for why Unicode folding is unsafe here.
func asciiLower(s string) string {
	b := []byte(s)
	for i := range b {
		if b[i] >= 'A' && b[i] <= 'Z' {
			b[i] += 'a' - 'A'
		}
	}
	return string(b)
}

// isLoopbackHost reports whether a normalized host names the local machine.
// Membership test, not a blocklist: an unrecognized form is treated as remote,
// so an incomplete list costs usability and never safety.
func isLoopbackHost(host string) bool {
	if host == "localhost" {
		return true
	}
	ip := net.ParseIP(host)
	return ip != nil && ip.IsLoopback()
}

// truncate renders a value for an error message, bounded so a pack file that is
// really a symlink to something else cannot spill its contents into a log.
func truncate(s string) string {
	if len(s) > maxEchoedValue {
		return fmt.Sprintf("%q(+%d more bytes)", s[:maxEchoedValue], len(s)-maxEchoedValue)
	}
	return fmt.Sprintf("%q", s)
}
