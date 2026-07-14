#!/usr/bin/env python3
"""Generates the deterministic synthetic fixture site.

The site imitates a developer-docs vertical without shipping anyone else's
content (no licensing questions, no live network in tests). It includes the
hazards a real crawler must survive:

  * robots.txt with a disallowed section
  * sitemap.xml listing a subset of pages
  * a deep chain of "calendar" pages (depth trap — max_depth must stop it)
  * duplicate pages with near-identical text (SimHash dedup target)
  * pages behind a redirect chain (configured in nginx.conf)

Output is regenerated at Docker build time; only this generator is committed.
Determinism matters: golden eval queries point at these URLs.
"""

from __future__ import annotations

import argparse
import random
from pathlib import Path

SEED = 0x1005  # fixed forever; eval golden data depends on this layout

# (section, slug, title, lead) — the "real" documentation pages.
# js/array-map is the P1 exit-criterion page: "array map" must return it top-3.
METHODS = {
    "js": [
        ("array-map", "Array.prototype.map()", "Creates a new array populated with the results of calling a provided function on every element of the calling array."),
        ("array-filter", "Array.prototype.filter()", "Creates a shallow copy of a portion of an array, filtered down to the elements that pass the test."),
        ("array-reduce", "Array.prototype.reduce()", "Executes a reducer callback on each element of the array, passing in the return value from the preceding calculation."),
        ("promise-all", "Promise.all()", "Takes an iterable of promises and returns a single promise that fulfils when all of the input promises fulfil."),
        ("string-split", "String.prototype.split()", "Divides a string into an ordered list of substrings by searching for a pattern."),
        ("object-entries", "Object.entries()", "Returns an array of a given object's own enumerable string-keyed property key-value pairs."),
    ],
    "py": [
        ("list-comprehensions", "List comprehensions", "A concise way to build lists by mapping and filtering an iterable in a single expression."),
        ("dict-get", "dict.get()", "Returns the value for a key if the key is in the dictionary, else a default value."),
        ("itertools-chain", "itertools.chain()", "Makes an iterator that returns elements from the first iterable until exhausted, then proceeds to the next."),
        ("asyncio-gather", "asyncio.gather()", "Runs awaitable objects concurrently and aggregates their results in order."),
        ("pathlib-glob", "Path.glob()", "Globs the given relative pattern in the directory represented by this path, yielding matching files."),
    ],
    "go": [
        ("goroutines", "Goroutines", "A goroutine is a lightweight thread managed by the Go runtime, started with the go statement."),
        ("channels", "Channels", "Channels are typed conduits through which you can send and receive values between goroutines."),
        ("errgroup", "errgroup.Group", "Provides synchronization, error propagation, and context cancelation for groups of goroutines."),
        ("slices-sort", "slices.Sort()", "Sorts a slice of any ordered type in ascending order in place."),
    ],
    "rust": [
        ("iterators", "Iterators", "The Iterator trait implements lazily evaluated sequences; adapters like map and filter compose without allocating."),
        ("ownership", "Ownership", "Ownership is Rust's mechanism for memory safety without garbage collection: each value has a single owner."),
        ("result-and-option", "Result and Option", "Result expresses recoverable errors and Option expresses nullable values, both handled with match or combinators."),
        ("vec-retain", "Vec::retain()", "Retains only the elements specified by the predicate, operating in place and preserving order."),
    ],
}

FILLER_WORDS = (
    "value function element callback iterator argument returns parameter "
    "sequence collection lazily evaluates chained pattern immutable slice "
    "reference index string number allocation buffer stream token parse"
).split()

TRAP_DEPTH = 60  # deep enough to wedge a depth-unlimited crawler in tests


def page(title: str, lead: str, body: str, links: list[tuple[str, str]]) -> str:
    nav = "\n".join(f'<li><a href="{href}">{text}</a></li>' for href, text in links)
    return f"""<!doctype html>
<html lang="en">
<head><meta charset="utf-8"><title>{title}</title></head>
<body>
<header><a href="/index.html">Fixture Docs</a></header>
<main>
<h1>{title}</h1>
<p class="lead">{lead}</p>
{body}
</main>
<nav><ul>
{nav}
</ul></nav>
<footer>Synthetic fixture content for Loom tests — not real documentation.</footer>
</body>
</html>
"""


def paragraphs(rng: random.Random, n: int) -> str:
    out = []
    for _ in range(n):
        words = " ".join(rng.choice(FILLER_WORDS) for _ in range(40))
        out.append(f"<p>{words}.</p>")
    return "\n".join(out)


def build_site(out: Path) -> int:
    rng = random.Random(SEED)
    count = 0

    all_pages: list[tuple[str, str]] = []  # (href, title) for index + sitemap
    for section, entries in METHODS.items():
        for slug, title, lead in entries:
            all_pages.append((f"/{section}/{slug}.html", title))

    for section, entries in METHODS.items():
        for i, (slug, title, lead) in enumerate(entries):
            siblings = [
                (f"/{section}/{s}.html", t)
                for s, t, _ in entries
                if s != slug
            ]
            cross = [all_pages[rng.randrange(len(all_pages))] for _ in range(3)]
            body = f"<h2>Description</h2>{paragraphs(rng, 3)}<h2>Examples</h2>{paragraphs(rng, 2)}"
            path = out / section / f"{slug}.html"
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(page(title, lead, body, siblings + cross))
            count += 1

            # Near-duplicate page: same text, one word changed (SimHash target)
            if i == 0:
                dup = page(title, lead.replace("array", "list", 1), body, siblings)
                (out / section / f"{slug}-printable.html").write_text(dup)
                count += 1

    # Depth trap: calendar pages that only link one level deeper
    cal = out / "calendar"
    cal.mkdir(parents=True, exist_ok=True)
    for day in range(TRAP_DEPTH):
        body = f"<p>Events for day {day}. Nothing scheduled.</p>"
        links = [(f"/calendar/day-{day + 1}.html", f"Day {day + 1}")]
        (cal / f"day-{day}.html").write_text(page(f"Calendar day {day}", "Archive.", body, links))
        count += 1

    # Disallowed-by-robots section that a polite crawler must never fetch
    private = out / "private"
    private.mkdir(parents=True, exist_ok=True)
    (private / "secret.html").write_text(
        page("Private", "If this URL appears in a segment file, politeness is broken.", "", [])
    )
    count += 1

    index_links = "\n".join(
        f'<li><a href="{href}">{title}</a></li>' for href, title in all_pages
    )
    (out / "index.html").write_text(
        page(
            "Fixture Docs",
            "Synthetic developer documentation for Loom integration tests.",
            f"<ul>{index_links}</ul>"
            '<p><a href="/redirect/a">Redirected article</a> · '
            '<a href="/calendar/day-0.html">Calendar archive</a></p>',
            [],
        )
    )
    count += 1

    (out / "robots.txt").write_text(
        "User-agent: *\nDisallow: /private/\nCrawl-delay: 1\nSitemap: /sitemap.xml\n"
    )
    sitemap_urls = "\n".join(
        f"  <url><loc>http://fixture.loom.test{href}</loc></url>" for href, _ in all_pages
    )
    (out / "sitemap.xml").write_text(
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n'
        f"{sitemap_urls}\n</urlset>\n"
    )
    count += 2
    return count


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True, help="output directory")
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    n = build_site(args.out)
    print(f"fixture-site: wrote {n} files to {args.out}")


if __name__ == "__main__":
    main()
