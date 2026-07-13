import type { SearchResponseV1 } from '../contracts/gen/search_response.v1'
import { parseSearchResponse } from './validate'

const API_BASE = import.meta.env.VITE_LOOM_API_BASE ?? 'http://localhost:7700'

/** Runs a search against loom-server, validating the response contract. */
export async function search(query: string, vertical = 'devdocs'): Promise<SearchResponseV1> {
  const params = new URLSearchParams({ q: query, vertical })
  const resp = await fetch(`${API_BASE}/search?${params}`)
  if (!resp.ok) {
    const body = await resp.text()
    throw new Error(`search failed (${resp.status}): ${body}`)
  }
  return parseSearchResponse(await resp.json())
}
