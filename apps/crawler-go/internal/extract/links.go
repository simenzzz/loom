// Package extract pulls outbound links out of fetched HTML.
//
// Only links. Text extraction, boilerplate removal and tokenizing all live in
// the Rust engine, which owns everything the index sees; the crawler stores raw
// HTML and needs hrefs solely to know where to go next.
package extract

import "net/url"

// Links returns the canonical outbound URLs found in htmlBody, in document
// order, with duplicates removed.
//
// TODO(you): implement.
//
// Algorithm:
//
//  1. Parse with golang.org/x/net/html. Use the tokenizer (html.NewTokenizer)
//     rather than html.Parse: a tokenizer streams and cannot be made to build a
//     pathological tree, and link discovery needs no tree.
//
//     The dependency is not in go.mod yet — nothing imports it while this is a
//     stub, and `go mod tidy` drops what nothing imports. Add it when you start:
//
//     go get golang.org/x/net/html
//  2. Walk tokens. On a start tag or self-closing tag named "a", read the
//     "href" attribute. Attribute names are case-insensitive; the tokenizer
//     lowercases them already.
//  3. Skip an href that is empty or whitespace-only.
//  4. Honor rel="nofollow" by skipping the link. It is a request from the page
//     author and costs nothing to respect.
//  5. Stop descending into <script> and <style>: their contents are not markup
//     and anything that looks like a URL in them is not a link.
//  6. Resolve and canonicalize each href with urlx.Canonicalize against base.
//     Drop the ones that error — a page full of "javascript:void(0)" is normal,
//     not an error condition, and one bad href must not discard the page's
//     other links.
//  7. Deduplicate, preserving first-seen order. A nav bar repeated on every
//     page would otherwise push the same URL into the frontier a hundred times.
//
// Also handle <base href="...">: when present it replaces base for every
// subsequent link in the document (HTML §4.2.3). Ignore a second <base>, and
// ignore one that will not canonicalize.
//
// Invariants the tests assert:
//   - Total: arbitrary bytes never panic. Truncated HTML mid-tag, unclosed
//     quotes, nested forms, 10 MB of "<" — all normal input for a crawler.
//   - Every returned string is an absolute http(s) URL, already canonical.
//   - Document order is preserved and there are no duplicates.
//   - A malformed href never discards the rest of the document's links.
func Links(base *url.URL, htmlBody []byte) ([]string, error) {
	panic("TODO(you): implement Links — see the recipe above")
}
