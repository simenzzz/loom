package pack

import (
	"reflect"
	"testing"

	"loom/crawler/internal/contracts/gen"
)

// The absence paths are the reason merge builds a map instead of the generated
// struct, and verticals/devdocs populates every section — so loading it exercises
// none of them. These tests hit merge directly, with no filesystem involved.
func TestMergeOmitsAbsentKeys(t *testing.T) {
	tests := []struct {
		name        string
		pf          packFile
		pol         policyFile
		ex          extractFile
		seeds       []string
		wantAbsent  []string
		wantPresent map[string]any
	}{
		{
			name:       "everything absent",
			wantAbsent: []string{"pack", "ranking", "limits", "politeness", "hosts", "url_filters", "content", "code", "seeds"},
		},
		{
			name:       "absent url_filters is omitted, not emitted empty",
			pol:        policyFile{Hosts: &hostsSection{Allowed: []string{"go.dev"}}},
			wantAbsent: []string{"url_filters"},
		},
		{
			name:       "nil seeds omitted so the schema reports absence, not minItems",
			seeds:      nil,
			wantAbsent: []string{"seeds"},
		},
		{
			// A zero that was actually written must survive. This is the exact
			// case that a struct round trip cannot distinguish from absence.
			name:        "explicit zero is preserved",
			pf:          packFile{Ranking: &rankingSection{Bm25K1: ptr(0.0)}},
			wantPresent: map[string]any{"ranking": map[string]any{"bm25_k1": 0.0}},
		},
		{
			// `main_selectors = []` is a meaningful value: it says "no hints",
			// which differs from "the key was never written".
			name:        "empty-but-present list is preserved",
			ex:          extractFile{Content: &contentSection{MainSelectors: []string{}}},
			wantPresent: map[string]any{"content": map[string]any{"main_selectors": []string{}}},
		},
		{
			name:        "partial section keeps only what was written",
			pf:          packFile{Pack: &packSection{ID: ptr("devdocs")}},
			wantPresent: map[string]any{"pack": map[string]any{"id": "devdocs"}},
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got := merge(&tc.pf, &tc.pol, &tc.ex, tc.seeds)

			for _, key := range tc.wantAbsent {
				if _, present := got[key]; present {
					t.Errorf("key %q should be absent, got %#v", key, got[key])
				}
			}
			for key, want := range tc.wantPresent {
				if !reflect.DeepEqual(got[key], want) {
					t.Errorf("key %q = %#v, want %#v", key, got[key], want)
				}
			}
			// The discriminator is loader-injected and always present.
			if got["schema"] != schemaDiscriminator {
				t.Errorf("schema = %v, want %q", got["schema"], schemaDiscriminator)
			}
		})
	}
}

// TestCloneCopiesEveryReferenceField walks the generated type reflectively.
//
// Naming fields explicitly would let a new []string in the schema regenerate the
// struct, keep compiling, and silently alias between a pack and its clone. The
// unknown-kind failure is the part that makes this a guard rather than a
// snapshot: a schema change that introduces a map or an interface forces someone
// to decide how Clone should handle it.
func TestCloneCopiesEveryReferenceField(t *testing.T) {
	original, err := Load(devdocsDir)
	if err != nil {
		t.Fatalf("loading pack: %v", err)
	}
	copied := Clone(original)

	assertNoSharedReferences(t, reflect.ValueOf(*original), reflect.ValueOf(*copied), "VerticalPackV1")
}

func assertNoSharedReferences(t *testing.T, orig, cloned reflect.Value, path string) {
	t.Helper()

	switch orig.Kind() {
	case reflect.Struct:
		for i := 0; i < orig.NumField(); i++ {
			assertNoSharedReferences(t, orig.Field(i), cloned.Field(i),
				path+"."+orig.Type().Field(i).Name)
		}
	case reflect.Slice:
		if orig.IsNil() || orig.Len() == 0 {
			return
		}
		if orig.UnsafePointer() == cloned.UnsafePointer() {
			t.Errorf("%s shares a backing array between the pack and its clone", path)
		}
		for i := 0; i < orig.Len(); i++ {
			assertNoSharedReferences(t, orig.Index(i), cloned.Index(i), path+"[]")
		}
	case reflect.Pointer:
		if orig.IsNil() {
			return
		}
		if orig.Pointer() == cloned.Pointer() {
			t.Errorf("%s is the same pointer in the pack and its clone", path)
		}
		assertNoSharedReferences(t, orig.Elem(), cloned.Elem(), path+".*")
	case reflect.String, reflect.Bool,
		reflect.Int, reflect.Int8, reflect.Int16, reflect.Int32, reflect.Int64,
		reflect.Uint, reflect.Uint8, reflect.Uint16, reflect.Uint32, reflect.Uint64,
		reflect.Float32, reflect.Float64:
		// Value types; copying the struct copies them. Strings are immutable.
	default:
		t.Fatalf("%s has kind %s, which Clone has never been taught to copy — "+
			"decide how it should be handled before adding it to the schema",
			path, orig.Kind())
	}
}

// Clone must be complete against the current schema; if this ever fails, a field
// was added to the contract and Clone was not updated.
func TestCloneHandlesNilOptionalSection(t *testing.T) {
	p := &gen.VerticalPackV1{Schema: schemaDiscriminator, UrlFilters: nil}
	if got := Clone(p); got.UrlFilters != nil {
		t.Errorf("nil url_filters should stay nil, got %#v", got.UrlFilters)
	}
}

func ptr[T any](v T) *T { return &v }
