package pack

import (
	"slices"
	"testing"
)

func TestParseSeeds(t *testing.T) {
	tests := []struct {
		name string
		in   string
		want []string
	}{
		{
			name: "comments and blanks dropped, order preserved",
			in: "# One seed URL per line.\n" +
				"https://go.dev/doc/\n" +
				"\n" +
				"# another comment\n" +
				"https://docs.python.org/3/\n",
			want: []string{"https://go.dev/doc/", "https://docs.python.org/3/"},
		},
		{
			name: "surrounding whitespace trimmed",
			in:   "  https://go.dev/doc/  \n\t\n\thttps://example.com/\t\n",
			want: []string{"https://go.dev/doc/", "https://example.com/"},
		},
		{
			name: "indented comment is still a comment",
			in:   "   # indented\nhttps://go.dev/doc/\n",
			want: []string{"https://go.dev/doc/"},
		},
		{
			name: "a fragment is not a comment",
			in:   "https://go.dev/doc/#section\n",
			want: []string{"https://go.dev/doc/#section"},
		},
		{
			name: "comment-only file yields nothing",
			in:   "# nothing here\n\n# nor here\n",
			want: nil,
		},
		{
			name: "empty file yields nothing",
			in:   "",
			want: nil,
		},
		{
			name: "final line without newline is kept",
			in:   "https://go.dev/doc/",
			want: []string{"https://go.dev/doc/"},
		},
		{
			name: "CRLF line endings",
			in:   "https://go.dev/doc/\r\nhttps://example.com/\r\n",
			want: []string{"https://go.dev/doc/", "https://example.com/"},
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			got, err := parseSeeds([]byte(tc.in))
			if err != nil {
				t.Fatalf("parseSeeds: %v", err)
			}
			if !slices.Equal(got, tc.want) {
				t.Errorf("parseSeeds() = %q, want %q", got, tc.want)
			}
		})
	}
}

// A single absurdly long line is malformed input, not a long URL. It must
// surface as an error rather than being silently truncated into a
// different-but-plausible URL.
func TestParseSeedsRejectsOverlongLine(t *testing.T) {
	long := make([]byte, maxSeedLine+1)
	for i := range long {
		long[i] = 'a'
	}
	if _, err := parseSeeds(long); err == nil {
		t.Fatal("expected an error for a line past maxSeedLine, got nil")
	}
}
