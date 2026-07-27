// Package urlx canonicalizes URLs into the single string form used as document
// identity everywhere downstream.
//
// Identity is the whole point. The frontier's seen-set, the near-duplicate
// detector, the link graph and the index all key on the canonical form, so two
// spellings of one page must produce one string or the crawler fetches the same
// document twice and the index stores it twice.
package urlx

import "net/url"

// Canonicalize resolves ref against base and returns the canonical absolute URL.
//
// TODO(you): implement.
//
// Algorithm (RFC 3986 §5.2 for resolution, §6.2.2 for normalization):
//
//  1. Parse ref. A ref that will not parse is an error, not a silent drop —
//     the caller counts it.
//  2. Resolve against base with url.URL.ResolveReference, so "../x", "/x" and
//     "x" all become absolute. When base is nil, ref must already be absolute.
//  3. Reject any scheme other than http or https. javascript:, mailto:, data:
//     and file: all reach a crawler through hrefs and none is fetchable.
//  4. Lowercase the scheme and host. Both are case-insensitive (§6.2.2.1); the
//     path is NOT — "/A" and "/a" are different documents on most servers.
//  5. Drop the port when it is the scheme default (80 for http, 443 for https),
//     so http://x:80/ and http://x/ converge (§6.2.3).
//  6. Drop the fragment entirely. A fragment addresses a place within a
//     document, never a different document, and fetching "#top" refetches the
//     page (§3.5).
//  7. Remove dot segments from the path: "/a/./b/../c" becomes "/a/c"
//     (§5.2.4). ResolveReference already does this for relative refs; an
//     absolute ref with dot segments still needs it.
//  8. An empty path becomes "/" — http://x and http://x/ are the same document
//     (§6.2.3).
//  9. Normalize percent-encoding: uppercase the hex digits, and decode octets
//     that need no encoding at all (unreserved: ALPHA / DIGIT / "-" / "." /
//     "_" / "~") (§6.2.2.2, §2.3).
//
// Deliberately NOT done here, because each needs corpus evidence rather than a
// rule, and a wrong guess merges two distinct pages into one identity:
// sorting or stripping query parameters, stripping trailing slashes, and
// stripping "index.html". Those land in P2 with the golden corpus test.
//
// Invariants, each of which the tests assert:
//   - Idempotent: Canonicalize(Canonicalize(u)) == Canonicalize(u).
//   - Total: any input either returns a valid absolute http(s) URL or an
//     error. It never returns "" with a nil error, and it never panics —
//     hostile HTML is the normal case here.
//   - Never widens: the result addresses the same resource as the input.
func Canonicalize(base *url.URL, ref string) (string, error) {
	panic("TODO(you): implement Canonicalize — see the recipe above")
}
