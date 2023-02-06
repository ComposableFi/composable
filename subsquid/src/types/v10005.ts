import type {Result, Option} from './support'

export type RewardPoolConfiguration = RewardPoolConfiguration_RewardRateBasedIncentive

export interface RewardPoolConfiguration_RewardRateBasedIncentive {
    __kind: 'RewardRateBasedIncentive'
    owner: Uint8Array
    assetId: bigint
    startBlock: number
    rewardConfigs: [bigint, RewardConfig][]
    lock: LockConfig
    shareAssetId: bigint
    financialNftAssetId: bigint
    minimumStakingAmount: bigint
}

export interface RewardUpdate {
    rewardRate: RewardRate
}

export interface RewardConfig {
    rewardRate: RewardRate
}

export interface LockConfig {
    durationMultipliers: DurationMultipliers
    unlockPenalty: number
}

export interface RewardRate {
    period: RewardRatePeriod
    amount: bigint
}

export type DurationMultipliers = DurationMultipliers_Presets

export interface DurationMultipliers_Presets {
    __kind: 'Presets'
    value: [bigint, bigint][]
}

export type RewardRatePeriod = RewardRatePeriod_PerSecond

export interface RewardRatePeriod_PerSecond {
    __kind: 'PerSecond'
}
