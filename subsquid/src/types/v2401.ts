import type {Result} from './support'

export interface CurrencyPair {
  base: bigint
  quote: bigint
}

export interface Fee {
  fee: bigint
  lpFee: bigint
  ownerFee: bigint
  protocolFee: bigint
  assetId: bigint
}

export interface VestingSchedule {
  vestingScheduleId: bigint
  window: VestingWindow
  periodCount: number
  perPeriod: bigint
  alreadyClaimed: bigint
}

export type VestingWindow = VestingWindow_MomentBased | VestingWindow_BlockNumberBased

export interface VestingWindow_MomentBased {
  __kind: 'MomentBased'
  start: bigint
  period: bigint
}

export interface VestingWindow_BlockNumberBased {
  __kind: 'BlockNumberBased'
  start: number
  period: number
}
