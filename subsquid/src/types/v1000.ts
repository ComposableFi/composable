import type {Result, Option} from './support'

export interface VestingSchedule {
    window: VestingWindow
    periodCount: number
    perPeriod: bigint
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
