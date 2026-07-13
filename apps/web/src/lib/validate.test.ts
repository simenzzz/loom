/**
 * Runs the shared cross-language contract fixture corpus. The same files are
 * executed by the Go, Rust, and Python suites.
 */
import { readdirSync, readFileSync } from 'node:fs'
import { join } from 'node:path'

import { describe, expect, it } from 'vitest'

import { assertValid, parseSearchResponse } from './validate'

const FIXTURES = join(__dirname, '../../../../packages/contracts/fixtures')

const schemaDirs = readdirSync(FIXTURES, { withFileTypes: true })
  .filter((e) => e.isDirectory())
  .map((e) => e.name)

describe('shared contract fixture corpus', () => {
  it('has at least one schema', () => {
    expect(schemaDirs.length).toBeGreaterThan(0)
  })

  for (const schema of schemaDirs) {
    for (const kind of ['valid', 'invalid'] as const) {
      const dir = join(FIXTURES, schema, kind)
      const files = readdirSync(dir)
      it(`${schema}: ${kind}/ is not empty`, () => {
        expect(files.length).toBeGreaterThan(0)
      })
      for (const file of files) {
        it(`${schema}/${kind}/${file}`, () => {
          const doc = JSON.parse(readFileSync(join(dir, file), 'utf-8'))
          if (kind === 'valid') {
            expect(() => assertValid(schema, doc)).not.toThrow()
          } else {
            expect(() => assertValid(schema, doc)).toThrow()
          }
        })
      }
    }
  }
})

describe('parseSearchResponse', () => {
  it('returns a typed response for a valid body', () => {
    const doc = JSON.parse(
      readFileSync(join(FIXTURES, 'search_response.v1/valid/with-results.json'), 'utf-8'),
    )
    const parsed = parseSearchResponse(doc)
    expect(parsed.results[0].title).toBe('Array.prototype.map()')
  })

  it('throws ContractViolation for an invalid body', () => {
    expect(() => parseSearchResponse({ schema: 'search_response.v1' })).toThrow(
      /contract search_response.v1/,
    )
  })
})
