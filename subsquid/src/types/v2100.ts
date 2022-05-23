import type {Result} from './support'

export type AccountId32 = Uint8Array

export interface CurrencyPair {
  base: CurrencyId
  quote: CurrencyId
}

export type CurrencyId = bigint

export interface Fee {
  fee: bigint
  lpFee: bigint
  ownerFee: bigint
  protocolFee: bigint
  assetId: CurrencyId
}
