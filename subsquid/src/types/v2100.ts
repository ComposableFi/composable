import type {Result} from './support'

export type AccountId32 = Uint8Array

export interface CurrencyPair {
  base: CurrencyId
  quote: CurrencyId
}

export type CurrencyId = bigint
