/**
 * Boundary validation: every response from loom-server is checked against
 * the shared contract schemas before rendering (house rule: validate on
 * both sides of every boundary).
 */
import { Ajv2020, type ValidateFunction } from 'ajv/dist/2020'
import addFormats from 'ajv-formats'

import crawlRecordV1 from '../contracts/gen/schemas/crawl_record.v1.schema.json'
import searchRequestV1 from '../contracts/gen/schemas/search_request.v1.schema.json'
import searchResponseV1 from '../contracts/gen/schemas/search_response.v1.schema.json'
import type { SearchResponseV1 } from '../contracts/gen/search_response.v1'

const ajv = new Ajv2020({ allErrors: true })
addFormats(ajv)

const validators: Record<string, ValidateFunction> = {
  'crawl_record.v1': ajv.compile(crawlRecordV1),
  'search_request.v1': ajv.compile(searchRequestV1),
  'search_response.v1': ajv.compile(searchResponseV1),
}

export class ContractViolation extends Error {
  constructor(contract: string, detail: string) {
    super(`contract ${contract}: ${detail}`)
    this.name = 'ContractViolation'
  }
}

/** Validates a document against a named contract, throwing on violation. */
export function assertValid(contract: string, doc: unknown): void {
  const validator = validators[contract]
  if (!validator) {
    throw new ContractViolation(contract, 'unknown contract')
  }
  if (!validator(doc)) {
    const detail = ajv.errorsText(validator.errors, { separator: '; ' })
    throw new ContractViolation(contract, detail)
  }
}

/** Parses and validates a search response body. */
export function parseSearchResponse(doc: unknown): SearchResponseV1 {
  assertValid('search_response.v1', doc)
  return doc as SearchResponseV1
}
