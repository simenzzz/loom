package pack

import (
	"slices"
	"strings"
	"testing"
)

func TestFixtureOverride(t *testing.T) {
	tests := []struct {
		name     string
		base     string
		wantHost string
		wantSeed string
	}{
		{
			name:     "local fixture site with port",
			base:     "http://localhost:7799",
			wantHost: "localhost",
			wantSeed: "http://localhost:7799/index.html",
		},
		{
			name:     "compose service name, no port, no dot",
			base:     "http://fixture-site",
			wantHost: "fixture-site",
			wantSeed: "http://fixture-site/index.html",
		},
		{
			name:     "trailing slash on the base is not doubled",
			base:     "http://localhost:7799/",
			wantHost: "localhost",
			wantSeed: "http://localhost:7799/index.html",
		},
		{
			name:     "uppercase host is normalized",
			base:     "http://LOCALHOST:7799",
			wantHost: "localhost",
			wantSeed: "http://localhost:7799/index.html",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			base, err := Load(devdocsDir)
			if err != nil {
				t.Fatalf("loading base pack: %v", err)
			}

			got, err := FixtureOverride(base, tc.base)
			if err != nil {
				t.Fatalf("FixtureOverride(%q): %v", tc.base, err)
			}

			if !slices.Equal(got.Hosts.Allowed, []string{tc.wantHost}) {
				t.Errorf("hosts.allowed = %q, want [%q]", got.Hosts.Allowed, tc.wantHost)
			}
			if !slices.Equal(got.Seeds, []string{tc.wantSeed}) {
				t.Errorf("seeds = %q, want [%q]", got.Seeds, tc.wantSeed)
			}
			// The fixture site's robots.txt disallows /private/, and the E2E
			// asserts no segment names it. That assertion is only meaningful
			// while the override leaves this alone.
			if !got.Politeness.RespectRobots {
				t.Error("respect_robots must stay true — /private/ has to remain forbidden")
			}
		})
	}
}

// The house immutability rule: FixtureOverride returns a new pack and the
// caller's original is untouched, including its slice backing arrays.
func TestFixtureOverrideDoesNotMutateInput(t *testing.T) {
	original, err := Load(devdocsDir)
	if err != nil {
		t.Fatalf("loading base pack: %v", err)
	}
	beforeHosts := slices.Clone(original.Hosts.Allowed)
	beforeSeeds := slices.Clone(original.Seeds)
	beforeSelectors := slices.Clone(original.Content.MainSelectors)

	got, err := FixtureOverride(original, "http://localhost:7799")
	if err != nil {
		t.Fatalf("FixtureOverride: %v", err)
	}

	if !slices.Equal(original.Hosts.Allowed, beforeHosts) {
		t.Errorf("input hosts.allowed mutated: %q, want %q", original.Hosts.Allowed, beforeHosts)
	}
	if !slices.Equal(original.Seeds, beforeSeeds) {
		t.Errorf("input seeds mutated: %q, want %q", original.Seeds, beforeSeeds)
	}

	// Mutating the copy's shared-looking slices must not reach the original.
	got.Content.MainSelectors[0] = "MUTATED"
	if !slices.Equal(original.Content.MainSelectors, beforeSelectors) {
		t.Errorf("copy and original share a slice backing array: original now %q",
			original.Content.MainSelectors)
	}
	if got.UrlFilters != nil && original.UrlFilters != nil {
		got.UrlFilters.DenyContains[0] = "MUTATED"
		if original.UrlFilters.DenyContains[0] == "MUTATED" {
			t.Error("url_filters.deny_contains is shared between copy and original")
		}
	}
}

func TestFixtureOverrideRejects(t *testing.T) {
	tests := []struct {
		name    string
		base    string
		wantErr string
	}{
		{name: "non-http scheme", base: "file:///etc/passwd", wantErr: "must be http or https"},
		{name: "no scheme", base: "localhost:7799", wantErr: "must be http or https"},
		{name: "empty", base: "", wantErr: "must be http or https"},
		{name: "base carries a path", base: "http://localhost:7799/docs", wantErr: "no path"},
		{name: "unparseable", base: "http://[::1", wantErr: "not a parseable URL"},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			base, err := Load(devdocsDir)
			if err != nil {
				t.Fatalf("loading base pack: %v", err)
			}
			if _, err := FixtureOverride(base, tc.base); err == nil {
				t.Fatalf("expected FixtureOverride(%q) to fail", tc.base)
			} else if !strings.Contains(err.Error(), tc.wantErr) {
				t.Errorf("error %q does not mention %q", err, tc.wantErr)
			}
		})
	}
}

func TestFixtureOverrideRejectsNilPack(t *testing.T) {
	if _, err := FixtureOverride(nil, "http://localhost:7799"); err == nil {
		t.Fatal("expected an error for a nil pack")
	}
}
