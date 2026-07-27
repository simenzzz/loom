// Package pack loads a vertical pack from disk.
//
// A pack is four files — pack.toml, policy.toml, extract.toml and seeds.txt —
// that together describe one searchable corpus: its identity, its ranking
// weights, the limits and politeness policy the crawler must obey, where to
// start, and how to extract content. Adding a vertical must never require a
// code change; that claim is what this package exists to keep true.
//
// The four files are merged into a single vertical_pack.v1 document and
// validated against the shared contract before any caller sees them, so a
// malformed pack fails at load rather than midway through a crawl.
package pack

import (
	"encoding/json"
	"fmt"
	"io"
	"math"
	"os"
	"path/filepath"

	"github.com/BurntSushi/toml"

	"loom/crawler/internal/contracts"
	"loom/crawler/internal/contracts/gen"
)

// maxPackFileBytes bounds any single file in a pack directory. Packs are
// hand-written config; a megabyte is orders of magnitude more than any of them
// needs, and the cap turns a hostile or corrupted file into a clean error
// instead of unbounded memory growth.
const maxPackFileBytes = 1 << 20

// File names within a pack directory.
const (
	packFileName    = "pack.toml"
	policyFileName  = "policy.toml"
	extractFileName = "extract.toml"
	seedsFileName   = "seeds.txt"
)

// schemaDiscriminator is injected by the loader: it identifies the contract
// and deliberately does not appear in any TOML file, since it describes the
// merged document rather than any one input.
const schemaDiscriminator = "vertical_pack.v1"

// Load reads the vertical pack in dir and returns it as a validated
// vertical_pack.v1 document.
//
// It fails rather than guesses. Unknown TOML keys are rejected, because the
// contract sets additionalProperties:false on every object and a loader that
// silently ignored a typo'd key would be laxer than the schema it claims to
// enforce — a misspelled respect_robots would read as false. Missing keys are
// left absent so schema validation names them, rather than arriving as
// plausible zero values.
func Load(dir string) (*gen.VerticalPackV1, error) {
	var pf packFile
	if err := decodeTOML(filepath.Join(dir, packFileName), &pf); err != nil {
		return nil, err
	}
	var pol policyFile
	if err := decodeTOML(filepath.Join(dir, policyFileName), &pol); err != nil {
		return nil, err
	}
	var ex extractFile
	if err := decodeTOML(filepath.Join(dir, extractFileName), &ex); err != nil {
		return nil, err
	}

	seedsPath := filepath.Join(dir, seedsFileName)
	rawSeeds, err := readCapped(seedsPath)
	if err != nil {
		return nil, err
	}
	seeds, err := parseSeeds(rawSeeds)
	if err != nil {
		return nil, fmt.Errorf("pack: parsing %s: %w", seedsPath, err)
	}

	// Validate the merged document *before* it becomes a typed struct.
	//
	// This ordering is the whole point. Go structs cannot express absence for
	// a scalar, so decoding first would turn a missing bm25_k1 into 0.0 — a
	// schema-valid value that silently produces degenerate BM25. Validating
	// the map, where an absent key is genuinely absent, lets the contract's
	// `required` list be the single authority on what a pack must contain.
	doc := merge(&pf, &pol, &ex, seeds)
	if err := rejectNonFinite(doc, ""); err != nil {
		return nil, fmt.Errorf("pack in %s: %w", dir, err)
	}
	raw, err := json.Marshal(doc)
	if err != nil {
		return nil, fmt.Errorf("pack in %s: serializing for validation: %w", dir, err)
	}
	if err := contracts.Validate(contracts.VerticalPackV1, raw); err != nil {
		return nil, fmt.Errorf("pack in %s: %w", dir, err)
	}

	var merged gen.VerticalPackV1
	if err := json.Unmarshal(raw, &merged); err != nil {
		return nil, fmt.Errorf("pack in %s: decoding validated pack: %w", dir, err)
	}
	if err := verify(&merged, dir); err != nil {
		return nil, fmt.Errorf("pack in %s: %w", dir, err)
	}
	return &merged, nil
}

// readCapped reads a pack file, refusing one larger than maxPackFileBytes.
//
// os.ReadFile would grow without bound: a pack file symlinked at /dev/zero has
// a stat size of 0 and never reaches EOF, and the schema's own size limits
// cannot help because they are only checked after the whole file is in memory.
func readCapped(path string) ([]byte, error) {
	f, err := os.Open(path)
	if err != nil {
		return nil, fmt.Errorf("pack: reading %s: %w", path, err)
	}
	defer f.Close()

	raw, err := io.ReadAll(io.LimitReader(f, maxPackFileBytes+1))
	if err != nil {
		return nil, fmt.Errorf("pack: reading %s: %w", path, err)
	}
	if len(raw) > maxPackFileBytes {
		return nil, fmt.Errorf("pack: reading %s: file exceeds %d bytes", path, maxPackFileBytes)
	}
	return raw, nil
}

// rejectNonFinite refuses Inf and NaN, which are legal TOML floats but have no
// JSON representation.
//
// Without this, `bm25_k1 = inf` fails at json.Marshal with a message that blames
// the loader's plumbing and names no key — the one input class that escapes the
// contract's authority. Rejecting it here keeps the error pointed at the file.
func rejectNonFinite(doc map[string]any, prefix string) error {
	for key, value := range doc {
		path := key
		if prefix != "" {
			path = prefix + "." + key
		}
		switch v := value.(type) {
		case float64:
			if math.IsInf(v, 0) || math.IsNaN(v) {
				return fmt.Errorf("%s is %v; TOML inf and nan cannot be represented "+
					"in the contract", path, v)
			}
		case map[string]any:
			if err := rejectNonFinite(v, path); err != nil {
				return err
			}
		}
	}
	return nil
}

// decodeTOML reads one TOML file into out, rejecting any key that out did not
// consume. BurntSushi reports those through MetaData.Undecoded.
//
// Read failures and parse failures are reported distinctly: "reading" for I/O,
// "parsing" for syntax. Collapsing them makes an error message that cannot tell
// a missing file from a malformed one, and makes a test asserting on either
// indistinguishable from one asserting on the other.
func decodeTOML(path string, out any) error {
	raw, err := readCapped(path)
	if err != nil {
		return err
	}
	md, err := toml.Decode(string(raw), out)
	if err != nil {
		return fmt.Errorf("pack: parsing %s: %w", path, err)
	}
	if undecoded := md.Undecoded(); len(undecoded) > 0 {
		keys := make([]string, 0, len(undecoded))
		for _, k := range undecoded {
			keys = append(keys, k.String())
		}
		return fmt.Errorf("pack: %s: unknown key(s) %v — packs reject unknown keys, "+
			"the same way the contract rejects unknown fields", path, keys)
	}
	return nil
}

// merge assembles the four inputs into the contract's single-document shape as
// a generic map.
//
// A map rather than the generated struct, because absence must survive: a key
// missing from the TOML must be missing from the document, so the contract's
// `required` list reports it instead of a zero value impersonating a real
// setting. Nothing is defaulted here — the schema decides what a pack must
// contain, and this function only reports what the files actually said.
func merge(pf *packFile, pol *policyFile, ex *extractFile, seeds []string) map[string]any {
	doc := map[string]any{"schema": schemaDiscriminator}
	if seeds != nil {
		doc["seeds"] = seeds
	}

	if p := pf.Pack; p != nil {
		doc["pack"] = fields(
			field("id", p.ID),
			field("name", p.Name),
			field("tab_label", p.TabLabel),
		)
	}
	if r := pf.Ranking; r != nil {
		doc["ranking"] = fields(
			field("title_weight", r.TitleWeight),
			field("body_weight", r.BodyWeight),
			field("code_weight", r.CodeWeight),
			field("anchor_weight", r.AnchorWeight),
			field("recency_half_life_days", r.RecencyHalfLifeDays),
			field("bm25_k1", r.Bm25K1),
			field("bm25_b", r.Bm25B),
		)
	}
	if l := pol.Limits; l != nil {
		doc["limits"] = fields(
			field("max_pages", l.MaxPages),
			field("max_depth", l.MaxDepth),
			field("max_body_bytes", l.MaxBodyBytes),
		)
	}
	if p := pol.Politeness; p != nil {
		doc["politeness"] = fields(
			field("default_rps_per_host", p.DefaultRpsPerHost),
			field("respect_robots", p.RespectRobots),
			field("respect_crawl_delay", p.RespectCrawlDelay),
		)
	}
	if h := pol.Hosts; h != nil {
		doc["hosts"] = fields(slice("allowed", h.Allowed))
	}
	if u := pol.URLFilters; u != nil {
		doc["url_filters"] = fields(slice("deny_contains", u.DenyContains))
	}
	if c := ex.Content; c != nil {
		doc["content"] = fields(
			slice("main_selectors", c.MainSelectors),
			slice("title_selectors", c.TitleSelectors),
		)
	}
	if c := ex.Code; c != nil {
		doc["code"] = fields(slice("preserve_selectors", c.PreserveSelectors))
	}
	return doc
}

// validate checks an already-typed pack against the shared contract. Used
// after Load, when the document is known complete and only its values change
// (see FixtureOverride).
func validate(p *gen.VerticalPackV1) error {
	raw, err := json.Marshal(p)
	if err != nil {
		return fmt.Errorf("serializing for validation: %w", err)
	}
	return contracts.Validate(contracts.VerticalPackV1, raw)
}

// entry is one optionally-present key in a merged section.
type entry struct {
	key     string
	value   any
	present bool
}

// field yields an entry that is present only when the TOML supplied the key.
func field[T any](key string, v *T) entry {
	if v == nil {
		return entry{key: key}
	}
	return entry{key: key, value: *v, present: true}
}

// slice yields an entry for a list-valued key. A nil slice means the key was
// absent; an empty-but-non-nil slice means the file said `[]`, which is a
// meaningful value the schema may well accept.
func slice[T any](key string, v []T) entry {
	if v == nil {
		return entry{key: key}
	}
	return entry{key: key, value: v, present: true}
}

// fields collects the present entries into a section map.
func fields(entries ...entry) map[string]any {
	out := make(map[string]any, len(entries))
	for _, e := range entries {
		if e.present {
			out[e.key] = e.value
		}
	}
	return out
}
