package extract

import (
	"net/url"
	"slices"
	"strings"
	"testing"
)

func mustParse(t *testing.T, raw string) *url.URL {
	t.Helper()
	u, err := url.Parse(raw)
	if err != nil {
		t.Fatalf("parsing base %q: %v", raw, err)
	}
	return u
}

func TestLinks(t *testing.T) {
	const base = "http://fixture.test/docs/guide.html"

	tests := []struct {
		name string
		html string
		want []string
	}{
		{
			name: "absolute and relative hrefs",
			html: `<a href="http://fixture.test/a">a</a><a href="/b">b</a><a href="c">c</a>`,
			want: []string{"http://fixture.test/a", "http://fixture.test/b", "http://fixture.test/docs/c"},
		},
		{
			name: "document order preserved",
			html: `<a href="/3">3</a><a href="/1">1</a><a href="/2">2</a>`,
			want: []string{"http://fixture.test/3", "http://fixture.test/1", "http://fixture.test/2"},
		},
		{
			name: "duplicates removed, first position kept",
			html: `<a href="/a">1</a><a href="/b">2</a><a href="/a">3</a>`,
			want: []string{"http://fixture.test/a", "http://fixture.test/b"},
		},
		{
			name: "fragments collapse onto the same document",
			html: `<a href="/a#one">1</a><a href="/a#two">2</a>`,
			want: []string{"http://fixture.test/a"},
		},
		{
			name: "non-fetchable schemes dropped",
			html: `<a href="javascript:void(0)">x</a><a href="mailto:a@b.c">y</a><a href="/real">z</a>`,
			want: []string{"http://fixture.test/real"},
		},
		{
			name: "empty and whitespace hrefs dropped",
			html: `<a href="">x</a><a href="   ">y</a><a href="/real">z</a>`,
			want: []string{"http://fixture.test/real"},
		},
		{
			name: "nofollow honored",
			html: `<a href="/skip" rel="nofollow">x</a><a href="/keep">y</a>`,
			want: []string{"http://fixture.test/keep"},
		},
		{
			name: "attribute case ignored",
			html: `<A HREF="/a">x</A>`,
			want: []string{"http://fixture.test/a"},
		},
		{
			name: "single-quoted and unquoted attributes",
			html: `<a href='/a'>x</a><a href=/b>y</a>`,
			want: []string{"http://fixture.test/a", "http://fixture.test/b"},
		},
		{
			name: "script and style contents ignored",
			html: `<script>var u = "<a href='/nope'>";</script><style>/* <a href="/nope2"> */</style><a href="/yes">y</a>`,
			want: []string{"http://fixture.test/yes"},
		},
		{
			name: "base href overrides the document base",
			html: `<head><base href="http://other.test/root/"></head><body><a href="x">x</a></body>`,
			want: []string{"http://other.test/root/x"},
		},
		{
			name: "one bad href does not discard the others",
			html: `<a href="ht tp://broken">x</a><a href="/good">y</a>`,
			want: []string{"http://fixture.test/good"},
		},
		{
			name: "no links at all",
			html: `<html><body><p>nothing here</p></body></html>`,
			want: nil,
		},
		{
			name: "the fixture site's nav shape",
			html: `<header><a href="/index.html">Fixture Docs</a></header><main><h1>x</h1></main>` +
				`<nav><ul><li><a href="/js/array-filter.html">Array.prototype.filter()</a></li>` +
				`<li><a href="/js/array-map-printable.html">printable</a></li></ul></nav>`,
			want: []string{
				"http://fixture.test/index.html",
				"http://fixture.test/js/array-filter.html",
				"http://fixture.test/js/array-map-printable.html",
			},
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got, err := Links(mustParse(t, base), []byte(tc.html))
			if err != nil {
				t.Fatalf("Links: %v", err)
			}
			if !slices.Equal(got, tc.want) {
				t.Errorf("Links()\n got %q\nwant %q", got, tc.want)
			}
		})
	}
}

// Every byte here came off the network from someone we do not control.
func TestLinksIsTotal(t *testing.T) {
	base := mustParse(t, "http://fixture.test/")

	nasty := [][]byte{
		nil, {}, {0x00, 0x01, 0x02}, {0xff, 0xfe},
		[]byte("<a href="),
		[]byte(`<a href="`),
		[]byte(`<a href="unclosed`),
		[]byte("<a"),
		[]byte("<<<<>>>>"),
		[]byte(strings.Repeat("<", 100000)),
		[]byte(strings.Repeat(`<a href="/x">`, 50000)),
		[]byte(strings.Repeat("<div>", 50000)),
		[]byte(`<a href="` + strings.Repeat("a", 100000) + `">x</a>`),
		[]byte("<!-- <a href=\"/commented\"> -->"),
	}

	for i, body := range nasty {
		got, err := Links(base, body)
		if err != nil {
			continue
		}
		for _, link := range got {
			if !strings.HasPrefix(link, "http://") && !strings.HasPrefix(link, "https://") {
				t.Errorf("case %d returned a non-http link %q", i, link)
			}
		}
	}
}
