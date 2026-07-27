package pack

import (
	"bufio"
	"bytes"
	"errors"
	"fmt"
	"strings"
)

const (
	// maxSeedLine bounds a single line of seeds.txt. The contract caps a seed
	// URL at 4096 characters; the allowance here is deliberately larger so an
	// over-long URL is reported by the contract, which names the field, rather
	// than by the scanner, which can only say "too long".
	maxSeedLine = 8192

	// maxSeeds bounds the seed count. The contract's maxItems is 10000, but it
	// is only checked after the whole list is built, so this stops a runaway
	// file before it becomes a large allocation.
	maxSeeds = 10000
)

// parseSeeds extracts seed URLs from the bytes of a seeds.txt.
//
// Lines whose first non-space character is '#' are comments and are dropped,
// as are blank lines. Surrounding whitespace is trimmed. Order is preserved,
// because seeds enter the frontier in file order and a reordered frontier is a
// different crawl.
//
// Note '#' is only a comment marker at the start of a line: it is a legal URL
// fragment delimiter, so a trailing "# note" is NOT stripped from a URL. Put
// comments on their own line.
func parseSeeds(raw []byte) ([]string, error) {
	var seeds []string
	scanner := bufio.NewScanner(bytes.NewReader(raw))
	scanner.Buffer(make([]byte, 0, 4096), maxSeedLine)

	line := 0
	for scanner.Scan() {
		line++
		text := strings.TrimSpace(scanner.Text())
		if text == "" || strings.HasPrefix(text, "#") {
			continue
		}
		if len(seeds) == maxSeeds {
			return nil, fmt.Errorf("more than %d seeds (line %d)", maxSeeds, line)
		}
		seeds = append(seeds, text)
	}
	if err := scanner.Err(); err != nil {
		// bufio reports only "token too long"; the line number and the cap are
		// what make it actionable.
		if errors.Is(err, bufio.ErrTooLong) {
			return nil, fmt.Errorf("line %d exceeds %d bytes", line+1, maxSeedLine)
		}
		return nil, fmt.Errorf("reading seeds: %w", err)
	}
	return seeds, nil
}
