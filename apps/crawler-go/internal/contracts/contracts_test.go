package contracts

import (
	"os"
	"path/filepath"
	"testing"
)

// fixturesRoot points at the shared cross-language fixture corpus. Every
// language's test suite runs the same corpus; a schema change without fixture
// updates fails all four suites at once.
const fixturesRoot = "../../../../packages/contracts/fixtures"

func TestSharedFixtureCorpus(t *testing.T) {
	entries, err := os.ReadDir(fixturesRoot)
	if err != nil {
		t.Fatalf("reading fixture corpus root: %v", err)
	}
	if len(entries) == 0 {
		t.Fatal("fixture corpus is empty")
	}
	for _, schemaDir := range entries {
		if !schemaDir.IsDir() {
			continue
		}
		name := Name(schemaDir.Name())
		for _, kind := range []string{"valid", "invalid"} {
			dir := filepath.Join(fixturesRoot, schemaDir.Name(), kind)
			files, err := os.ReadDir(dir)
			if err != nil {
				t.Fatalf("schema %s has no %s/ fixtures: %v", name, kind, err)
			}
			if len(files) == 0 {
				t.Fatalf("schema %s: %s/ is empty", name, kind)
			}
			for _, f := range files {
				t.Run(string(name)+"/"+kind+"/"+f.Name(), func(t *testing.T) {
					raw, err := os.ReadFile(filepath.Join(dir, f.Name()))
					if err != nil {
						t.Fatalf("reading fixture: %v", err)
					}
					verr := Validate(name, raw)
					if kind == "valid" && verr != nil {
						t.Errorf("expected valid, got: %v", verr)
					}
					if kind == "invalid" && verr == nil {
						t.Error("expected validation failure, got nil")
					}
				})
			}
		}
	}
}

func TestUnknownSchemaRejected(t *testing.T) {
	if err := Validate(Name("nonexistent.v1"), []byte(`{}`)); err == nil {
		t.Fatal("expected error for unknown schema")
	}
}
