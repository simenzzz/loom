package urlx

import (
	"net/url"
	"strings"
	"testing"
)

func mustParse(t *testing.T, raw string) *url.URL {
	t.Helper()
	u, err := url.Parse(raw)
	if err != nil {
		t.Fatalf("parsing base %q: %v", raw, err)
	}
	return u
}

// The golden table. Each row is one canonicalization rule; when a row fails,
// its name says which rule broke.
func TestCanonicalize(t *testing.T) {
	const base = "http://fixture.test/docs/guide/index.html"

	tests := []struct {
		name string
		base string
		ref  string
		want string
	}{
		// Resolution (RFC 3986 §5.2)
		{"absolute ref ignores base", base, "http://other.test/x", "http://other.test/x"},
		{"root-relative", base, "/js/array-map.html", "http://fixture.test/js/array-map.html"},
		{"path-relative", base, "sibling.html", "http://fixture.test/docs/guide/sibling.html"},
		{"parent-relative", base, "../intro.html", "http://fixture.test/docs/intro.html"},
		{"scheme-relative", base, "//other.test/x", "http://other.test/x"},
		{"empty ref is the base without its fragment", base, "", "http://fixture.test/docs/guide/index.html"},

		// Case normalization (§6.2.2.1) — host yes, path no
		{"scheme lowercased", base, "HTTP://Fixture.TEST/x", "http://fixture.test/x"},
		{"host lowercased", base, "http://FIXTURE.test/x", "http://fixture.test/x"},
		{"path case preserved", base, "/Docs/Guide.html", "http://fixture.test/Docs/Guide.html"},

		// Port (§6.2.3)
		{"default http port dropped", base, "http://fixture.test:80/x", "http://fixture.test/x"},
		{"default https port dropped", base, "https://fixture.test:443/x", "https://fixture.test/x"},
		{"non-default port kept", base, "http://fixture.test:7799/x", "http://fixture.test:7799/x"},

		// Fragment (§3.5)
		{"fragment dropped", base, "/x#section", "http://fixture.test/x"},
		{"fragment-only ref addresses the base document", base, "#section", "http://fixture.test/docs/guide/index.html"},

		// Dot segments (§5.2.4)
		{"single dot removed", base, "http://fixture.test/a/./b", "http://fixture.test/a/b"},
		{"double dot removed", base, "http://fixture.test/a/b/../c", "http://fixture.test/a/c"},
		{"dot segments cannot escape the root", base, "/../../../etc/passwd", "http://fixture.test/etc/passwd"},

		// Empty path (§6.2.3)
		{"empty path becomes slash", base, "http://fixture.test", "http://fixture.test/"},

		// Percent-encoding (§6.2.2.2, §2.3)
		{"hex digits uppercased", base, "/a%2fb", "http://fixture.test/a%2Fb"},
		{"unreserved octets decoded", base, "/%7Euser", "http://fixture.test/~user"},
		{"reserved octets stay encoded", base, "/a%3Fb", "http://fixture.test/a%3Fb"},

		// Query is preserved verbatim in P1 — see the recipe on why sorting waits
		{"query preserved", base, "/search?q=map&lang=en", "http://fixture.test/search?q=map&lang=en"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got, err := Canonicalize(mustParse(t, tc.base), tc.ref)
			if err != nil {
				t.Fatalf("Canonicalize(%q, %q): %v", tc.base, tc.ref, err)
			}
			if got != tc.want {
				t.Errorf("Canonicalize(%q, %q)\n got %q\nwant %q", tc.base, tc.ref, got, tc.want)
			}
		})
	}
}

func TestCanonicalizeRejects(t *testing.T) {
	const base = "http://fixture.test/docs/"

	tests := []struct {
		name string
		ref  string
	}{
		{"javascript scheme", "javascript:alert(1)"},
		{"mailto scheme", "mailto:someone@example.com"},
		{"data scheme", "data:text/html,<h1>x</h1>"},
		{"file scheme", "file:///etc/passwd"},
		{"ftp scheme", "ftp://fixture.test/x"},
		{"control character", "http://fixture.test/\x00"},
		{"not a URL at all", "http://[::1"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got, err := Canonicalize(mustParse(t, base), tc.ref)
			if err == nil {
				t.Fatalf("Canonicalize(%q) = %q, want an error", tc.ref, got)
			}
		})
	}
}

// Canonicalizing an already-canonical URL must change nothing. Without this the
// seen-set can hold two spellings of one page and the crawler fetches it twice.
func TestCanonicalizeIsIdempotent(t *testing.T) {
	base := mustParse(t, "http://fixture.test/docs/")
	refs := []string{
		"/js/array-map.html", "HTTP://FIXTURE.test:80/a/./b/../c",
		"/%7Euser/x#frag", "http://fixture.test", "/search?q=map",
	}

	for _, ref := range refs {
		once, err := Canonicalize(base, ref)
		if err != nil {
			t.Fatalf("Canonicalize(%q): %v", ref, err)
		}
		twice, err := Canonicalize(nil, once)
		if err != nil {
			t.Fatalf("Canonicalize(%q) on its own output: %v", once, err)
		}
		if once != twice {
			t.Errorf("not idempotent for %q: %q then %q", ref, once, twice)
		}
	}
}

// Hostile hrefs are the normal case for a crawler, not an edge case. Whatever
// comes out must be usable or an error — never a panic, never a non-http URL.
func TestCanonicalizeIsTotal(t *testing.T) {
	base := mustParse(t, "http://fixture.test/docs/")
	nasty := []string{
		"", " ", "\t\n", "//", "///", "http://", "http:///x", ":", "?", "#",
		"http://x/" + strings.Repeat("a/", 2000),
		"http://user:pass@fixture.test/x",
		"HtTpS://Fixture.Test./x",
		strings.Repeat("../", 500) + "x",
	}

	for _, ref := range nasty {
		got, err := Canonicalize(base, ref)
		if err != nil {
			continue // rejecting is always allowed
		}
		if got == "" {
			t.Errorf("Canonicalize(%q) returned empty string with nil error", ref)
		}
		if !strings.HasPrefix(got, "http://") && !strings.HasPrefix(got, "https://") {
			t.Errorf("Canonicalize(%q) = %q, which is not an http(s) URL", ref, got)
		}
	}
}
