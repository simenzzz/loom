package pack

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

const (
	devdocsDir    = "../../../../verticals/devdocs"
	goldenFixture = "../../../../packages/contracts/fixtures/vertical_pack.v1/valid/devdocs.json"
)

// TestLoadDevdocsMatchesGoldenFixture pins the loader's output to the committed
// contract fixture. The fixture is not decoration: it is the worked example of
// what merging the four files must produce, so editing the TOML without
// updating it — or vice versa — fails here rather than silently diverging.
func TestLoadDevdocsMatchesGoldenFixture(t *testing.T) {
	loaded, err := Load(devdocsDir)
	if err != nil {
		t.Fatalf("loading %s: %v", devdocsDir, err)
	}

	got := toGeneric(t, loaded)

	rawGolden, err := os.ReadFile(goldenFixture)
	if err != nil {
		t.Fatalf("reading golden fixture: %v", err)
	}
	var want any
	if err := json.Unmarshal(rawGolden, &want); err != nil {
		t.Fatalf("golden fixture is not JSON: %v", err)
	}

	if !reflect.DeepEqual(got, want) {
		gotPretty, _ := json.MarshalIndent(got, "", "  ")
		wantPretty, _ := json.MarshalIndent(want, "", "  ")
		t.Errorf("loaded pack does not match %s\n--- got ---\n%s\n--- want ---\n%s",
			goldenFixture, gotPretty, wantPretty)
	}
}

// TestLoadRejects covers every way a pack directory can be wrong. Each case
// starts from a known-good copy of verticals/devdocs and applies exactly one
// mutation, so a failure names its own cause.
func TestLoadRejects(t *testing.T) {
	tests := []struct {
		name    string
		mutate  func(t *testing.T, dir string)
		wantErr string
	}{
		{
			// A typo'd key is the case that matters: without the undecoded-key
			// check, `respect_robot` is silently ignored and the real
			// respect_robots keeps whatever it had.
			name: "misspelled key in policy.toml",
			mutate: func(t *testing.T, dir string) {
				replaceInFile(t, filepath.Join(dir, policyFileName),
					"respect_robots = true", "respect_robots = true\nrespect_robot = false")
			},
			wantErr: "unknown key",
		},
		{
			name: "unknown key in pack.toml",
			mutate: func(t *testing.T, dir string) {
				replaceInFile(t, filepath.Join(dir, packFileName),
					"bm25_b = 0.75", "bm25_b = 0.75\nbm25_delta = 0.5")
			},
			wantErr: "unknown key",
		},
		{
			// The single most important negative case: it is the only evidence
			// that merging to a map — rather than to a struct, where a missing
			// float would arrive as a plausible 0.0 — actually preserves absence.
			// Assert the named key, not just that something failed.
			name: "missing required key surfaces as contract violation",
			mutate: func(t *testing.T, dir string) {
				replaceInFile(t, filepath.Join(dir, packFileName), "bm25_k1 = 1.2", "")
			},
			wantErr: "at '/ranking': missing property 'bm25_k1'",
		},
		{
			name: "malformed TOML",
			mutate: func(t *testing.T, dir string) {
				appendLine(t, filepath.Join(dir, extractFileName), "\nthis is not toml\n")
			},
			wantErr: "parsing",
		},
		{
			name: "non-finite float",
			mutate: func(t *testing.T, dir string) {
				replaceInFile(t, filepath.Join(dir, packFileName), "bm25_k1 = 1.2", "bm25_k1 = inf")
			},
			wantErr: "ranking.bm25_k1 is +Inf",
		},
		{
			name: "seed outside the host allowlist",
			mutate: func(t *testing.T, dir string) {
				writeFile(t, filepath.Join(dir, seedsFileName), "https://evil.example/\n")
			},
			wantErr: "not in hosts.allowed",
		},
		{
			name: "seed smuggling a host via userinfo",
			mutate: func(t *testing.T, dir string) {
				writeFile(t, filepath.Join(dir, seedsFileName),
					"https://go.dev@169.254.169.254/latest/meta-data/\n")
			},
			// Caught by the contract's seed pattern, before the allowlist check
			// gets a chance. Assert that, so it cannot silently degrade into
			// being caught only by the allowlist — which also rejects it, but
			// would mean the pattern had stopped doing its job.
			wantErr: "at '/seeds/0'",
		},
		{
			// Must be reported as an absent property, not as an empty array
			// failing minItems: an empty array would mean merge() had started
			// emitting "seeds": [], destroying the absence property.
			name: "no seeds at all",
			mutate: func(t *testing.T, dir string) {
				writeFile(t, filepath.Join(dir, seedsFileName), "# every line a comment\n\n")
			},
			wantErr: "missing property 'seeds'",
		},
		{
			name: "robots ignored on a public host",
			mutate: func(t *testing.T, dir string) {
				replaceInFile(t, filepath.Join(dir, policyFileName),
					"respect_robots = true", "respect_robots = false")
			},
			wantErr: "respect_robots=false is only permitted",
		},
		{
			name: "missing seeds.txt",
			mutate: func(t *testing.T, dir string) {
				if err := os.Remove(filepath.Join(dir, seedsFileName)); err != nil {
					t.Fatalf("removing seeds: %v", err)
				}
			},
			wantErr: seedsFileName + ": open",
		},
		{
			name: "oversized pack file",
			mutate: func(t *testing.T, dir string) {
				writeFile(t, filepath.Join(dir, seedsFileName),
					strings.Repeat("# padding\n", maxPackFileBytes/10+1))
			},
			wantErr: "exceeds",
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			dir := copyPackDir(t, devdocsDir)
			tc.mutate(t, dir)

			_, err := Load(dir)
			if err == nil {
				t.Fatalf("expected load to fail, got nil error")
			}
			if !strings.Contains(err.Error(), tc.wantErr) {
				t.Errorf("error %q does not mention %q", err, tc.wantErr)
			}
		})
	}
}

// TestLoadAcceptsRobotsOverrideOnLoopback is the other half of the robots rule:
// the fixture site legitimately needs the escape hatch.
func TestLoadAcceptsRobotsOverrideOnLoopback(t *testing.T) {
	dir := copyPackDir(t, devdocsDir)
	replaceInFile(t, filepath.Join(dir, policyFileName),
		"respect_robots = true", "respect_robots = false")
	replaceInFile(t, filepath.Join(dir, policyFileName),
		`allowed = [
    "developer.mozilla.org",
    "doc.rust-lang.org",
    "go.dev",
    "docs.python.org",
]`, `allowed = ["localhost"]`)
	writeFile(t, filepath.Join(dir, seedsFileName), "http://localhost:7799/index.html\n")

	if _, err := Load(dir); err != nil {
		t.Fatalf("loopback pack with respect_robots=false should load, got: %v", err)
	}
}

// --- helpers ---

func toGeneric(t *testing.T, v any) any {
	t.Helper()
	raw, err := json.Marshal(v)
	if err != nil {
		t.Fatalf("marshalling: %v", err)
	}
	var out any
	if err := json.Unmarshal(raw, &out); err != nil {
		t.Fatalf("unmarshalling: %v", err)
	}
	return out
}

// copyPackDir copies a pack into a temp dir so a test can mutate it freely.
//
// The copy keeps the source's directory name, because pack.id must agree with
// it — a bare t.TempDir() is named "001" and would fail that check in every
// test rather than only in the one that means to.
func copyPackDir(t *testing.T, src string) string {
	t.Helper()
	dst := filepath.Join(t.TempDir(), filepath.Base(filepath.Clean(src)))
	if err := os.MkdirAll(dst, 0o750); err != nil {
		t.Fatalf("creating pack dir: %v", err)
	}
	entries, err := os.ReadDir(src)
	if err != nil {
		t.Fatalf("reading %s: %v", src, err)
	}
	for _, e := range entries {
		if e.IsDir() {
			continue
		}
		raw, err := os.ReadFile(filepath.Join(src, e.Name()))
		if err != nil {
			t.Fatalf("reading %s: %v", e.Name(), err)
		}
		if err := os.WriteFile(filepath.Join(dst, e.Name()), raw, 0o600); err != nil {
			t.Fatalf("writing %s: %v", e.Name(), err)
		}
	}
	return dst
}

func writeFile(t *testing.T, path, content string) {
	t.Helper()
	if err := os.WriteFile(path, []byte(content), 0o600); err != nil {
		t.Fatalf("writing %s: %v", path, err)
	}
}

func appendLine(t *testing.T, path, content string) {
	t.Helper()
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("reading %s: %v", path, err)
	}
	writeFile(t, path, string(raw)+content)
}

func replaceInFile(t *testing.T, path, old, new string) {
	t.Helper()
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("reading %s: %v", path, err)
	}
	if !strings.Contains(string(raw), old) {
		t.Fatalf("%s does not contain %q — the fixture pack changed shape", path, old)
	}
	writeFile(t, path, strings.Replace(string(raw), old, new, 1))
}
