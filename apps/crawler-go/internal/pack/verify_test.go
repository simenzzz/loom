package pack

import (
	"strings"
	"testing"
)

// TestNormalizeHost pins the equivalence classes the allowlist comparison
// depends on. The frontier must reach the identical verdict, so these cases are
// the contract between the loader and the crawler.
func TestNormalizeHost(t *testing.T) {
	tests := []struct {
		name    string
		in      string
		want    string
		wantErr bool
	}{
		{name: "already normal", in: "go.dev", want: "go.dev"},
		{name: "uppercase", in: "GO.DEV", want: "go.dev"},
		{name: "mixed case", in: "Developer.Mozilla.ORG", want: "developer.mozilla.org"},
		{name: "root-anchoring dot", in: "go.dev.", want: "go.dev"},
		{name: "surrounding whitespace", in: "  go.dev  ", want: "go.dev"},
		{name: "dotless service name", in: "fixture-site", want: "fixture-site"},
		{name: "loopback literal", in: "127.0.0.1", want: "127.0.0.1"},
		{name: "punycode passes through", in: "xn--r8jz45g.jp", want: "xn--r8jz45g.jp"},
		{name: "empty", in: "", want: ""},

		// A double dot is malformed. Trimming only one leaves a dot behind, the
		// comparison fails, and the host is refused — the right outcome.
		{name: "double trailing dot keeps one", in: "go.dev..", want: "go.dev."},

		// The bypass this function exists to prevent. strings.ToLower folds
		// U+0130 to 'i', so Unicode lowering would turn this into
		// "developer.mozilla.org" — matching the allowlist while resolving to
		// developer.xn--mozilla-9he.org, a different registerable domain.
		{name: "U+0130 must not fold into ASCII", in: "developer.mozİlla.org", wantErr: true},
		{name: "U+212A Kelvin must not fold into ASCII", in: "Kelvin.example", wantErr: true},
		{name: "raw non-ASCII", in: "münchen.example", wantErr: true},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got, err := NormalizeHost(tc.in)
			if tc.wantErr {
				if err == nil {
					t.Fatalf("NormalizeHost(%q) = %q, want an error", tc.in, got)
				}
				return
			}
			if err != nil {
				t.Fatalf("NormalizeHost(%q): %v", tc.in, err)
			}
			if got != tc.want {
				t.Errorf("NormalizeHost(%q) = %q, want %q", tc.in, got, tc.want)
			}
		})
	}
}

// TestLoadRejectsUnicodeConfusableSeed is the end-to-end form of the bypass:
// percent-encoded UTF-8 survives the contract's seed pattern, url.Parse decodes
// host bytes >= 0x80, and Unicode folding would then map the host onto an
// allowlisted one.
func TestLoadRejectsUnicodeConfusableSeed(t *testing.T) {
	confusables := []struct {
		name string
		seed string
	}{
		{"U+0130 in mozilla", "https://developer.moz%C4%B0lla.org/"},
		{"U+212A in a label", "https://%E2%84%AAelvin.go.dev/"},
	}

	for _, tc := range confusables {
		t.Run(tc.name, func(t *testing.T) {
			dir := copyPackDir(t, devdocsDir)
			writeFile(t, dir+"/"+seedsFileName, tc.seed+"\n")

			_, err := Load(dir)
			if err == nil {
				t.Fatal("a confusable host normalized into the allowlist and was accepted")
			}
			if !strings.Contains(err.Error(), "non-ASCII") {
				t.Errorf("error %q should name the non-ASCII host", err)
			}
		})
	}
}

func TestVerifySeedPortRules(t *testing.T) {
	tests := []struct {
		name    string
		host    string
		seed    string
		wantErr bool
	}{
		{name: "no port", host: "go.dev", seed: "https://go.dev/doc/"},
		{name: "default https port", host: "go.dev", seed: "https://go.dev:443/doc/"},
		{name: "default http port", host: "go.dev", seed: "http://go.dev:80/doc/"},
		{name: "loopback with fixture port", host: "localhost", seed: "http://localhost:7799/index.html"},
		{name: "loopback literal with port", host: "127.0.0.1", seed: "http://127.0.0.1:7799/"},

		// hosts.allowed cannot express a port, so allowlisting a host must not
		// authorize every service listening on it.
		{name: "redis port on a public host", host: "go.dev", seed: "https://go.dev:6379/", wantErr: true},
		{name: "ssh port on a public host", host: "go.dev", seed: "https://go.dev:22/", wantErr: true},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			dir := copyPackDir(t, devdocsDir)
			replaceInFile(t, dir+"/"+policyFileName, `allowed = [
    "developer.mozilla.org",
    "doc.rust-lang.org",
    "go.dev",
    "docs.python.org",
]`, `allowed = ["`+tc.host+`"]`)
			writeFile(t, dir+"/"+seedsFileName, tc.seed+"\n")

			_, err := Load(dir)
			if tc.wantErr {
				if err == nil {
					t.Fatalf("seed %q should have been refused", tc.seed)
				}
				if !strings.Contains(err.Error(), "names port") {
					t.Errorf("error %q should explain the port rule", err)
				}
				return
			}
			if err != nil {
				t.Fatalf("seed %q should load: %v", tc.seed, err)
			}
		})
	}
}

func TestHostAllowed(t *testing.T) {
	p, err := Load(devdocsDir)
	if err != nil {
		t.Fatalf("loading pack: %v", err)
	}

	tests := []struct {
		host string
		want bool
	}{
		{"go.dev", true},
		{"GO.DEV", true},
		{"go.dev.", true},
		{"docs.python.org", true},
		{"evil.example", false},
		{"", false},
		{"developer.mozİlla.org", false}, // must not fold into the allowlist
	}

	for _, tc := range tests {
		if got := HostAllowed(p, tc.host); got != tc.want {
			t.Errorf("HostAllowed(%q) = %v, want %v", tc.host, got, tc.want)
		}
	}
}

func TestLoadRejectsPackIDDirectoryMismatch(t *testing.T) {
	dir := copyPackDir(t, devdocsDir)
	replaceInFile(t, dir+"/"+packFileName, `id = "devdocs"`, `id = "recipes"`)

	_, err := Load(dir)
	if err == nil {
		t.Fatal("a pack whose id disagrees with its directory should be refused")
	}
	if !strings.Contains(err.Error(), "pack.id") {
		t.Errorf("error %q should name pack.id", err)
	}
}
