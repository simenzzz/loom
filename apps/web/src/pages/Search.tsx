import { useState } from 'react'

import type { SearchResponseV1 } from '../contracts/gen/search_response.v1'
import { search } from '../lib/api'

export function SearchPage() {
  const [query, setQuery] = useState('')
  const [response, setResponse] = useState<SearchResponseV1 | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  async function onSubmit(e: React.FormEvent) {
    e.preventDefault()
    const trimmed = query.trim()
    if (!trimmed) return
    setLoading(true)
    setError(null)
    try {
      setResponse(await search(trimmed))
    } catch (err) {
      setResponse(null)
      setError(err instanceof Error ? err.message : 'search failed')
    } finally {
      setLoading(false)
    }
  }

  return (
    <main style={{ maxWidth: 640, margin: '4rem auto', fontFamily: 'system-ui' }}>
      <h1>Loom</h1>
      <form onSubmit={onSubmit}>
        <input
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search developer docs…"
          aria-label="Search query"
          style={{ width: '100%', padding: '0.6rem', fontSize: '1rem' }}
        />
      </form>
      {loading && <p>Searching…</p>}
      {error && <p role="alert">{error}</p>}
      {response && (
        <section>
          <p>
            {response.total} result{response.total === 1 ? '' : 's'} ·{' '}
            {response.took_ms.toFixed(1)} ms
            {response.degraded ? ' · degraded (BM25 only)' : ''}
          </p>
          <ol>
            {response.results.map((r) => (
              <li key={r.url}>
                <a href={r.url}>{r.title}</a>
                <p>{r.snippet}</p>
              </li>
            ))}
          </ol>
        </section>
      )}
    </main>
  )
}
