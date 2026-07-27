// Package contracts validates data crossing service boundaries against the
// shared JSON Schemas in packages/contracts (copied here by codegen and
// embedded so the binary is self-contained).
package contracts

import (
	"bytes"
	"embed"
	"fmt"
	"sync"

	jsonschema "github.com/santhosh-tekuri/jsonschema/v6"
)

//go:embed gen/schemas/*.schema.json
var schemaFS embed.FS

// Name identifies a contract, e.g. "crawl_record.v1".
type Name string

const (
	CrawlRecordV1     Name = "crawl_record.v1"
	SearchRequestV1   Name = "search_request.v1"
	SearchResponseV1  Name = "search_response.v1"
	SegmentManifestV1 Name = "segment_manifest.v1"
	VerticalPackV1    Name = "vertical_pack.v1"
)

var (
	mu         sync.Mutex
	validators = map[Name]*jsonschema.Schema{}
)

func compiled(name Name) (*jsonschema.Schema, error) {
	mu.Lock()
	defer mu.Unlock()
	if s, ok := validators[name]; ok {
		return s, nil
	}
	path := fmt.Sprintf("gen/schemas/%s.schema.json", name)
	raw, err := schemaFS.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("contracts: unknown schema %q: %w", name, err)
	}
	doc, err := jsonschema.UnmarshalJSON(bytes.NewReader(raw))
	if err != nil {
		return nil, fmt.Errorf("contracts: schema %q is not valid JSON: %w", name, err)
	}
	c := jsonschema.NewCompiler()
	// format is assertion, not annotation — all four languages must agree
	// on what the corpus accepts (cross-language consistency rule).
	c.AssertFormat()
	url := fmt.Sprintf("loom:///%s.schema.json", name)
	if err := c.AddResource(url, doc); err != nil {
		return nil, fmt.Errorf("contracts: registering schema %q: %w", name, err)
	}
	s, err := c.Compile(url)
	if err != nil {
		return nil, fmt.Errorf("contracts: compiling schema %q: %w", name, err)
	}
	validators[name] = s
	return s, nil
}

// Validate checks raw JSON bytes against the named contract. It returns nil
// only when the document conforms. Both writers and readers of a boundary
// must call this (house rule: validate on both sides).
func Validate(name Name, raw []byte) error {
	s, err := compiled(name)
	if err != nil {
		return err
	}
	doc, err := jsonschema.UnmarshalJSON(bytes.NewReader(raw))
	if err != nil {
		return fmt.Errorf("contracts: %s: document is not valid JSON: %w", name, err)
	}
	if err := s.Validate(doc); err != nil {
		return fmt.Errorf("contracts: %s: %w", name, err)
	}
	return nil
}
