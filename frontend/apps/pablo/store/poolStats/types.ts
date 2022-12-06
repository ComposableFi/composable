import { TokenId } from "tokens";

export interface DailyRewards {
    symbol: string;
    icon: string;
    name: string;
    assetId: TokenId;
    rewardAmount: string;
    rewardAmountLeft: string;
}

export interface PoolStats {
    totalVolume: string;
    _24HrFee: string;
    _24HrVolume: string;
    _24HrTransactionCount: number;
    dailyRewards: DailyRewards[]; 
    apr: string;
}

export interface PoolStatsValue {
    /** All of these are fetched in 
     * quote asset so price conversion
     * to USD is needed
     */
    _24HrFeeValue: string;
    _24HrVolumeValue: string;
    totalVolumeValue: string;
}

export interface PoolStatsSlice {
    poolStats: {
        [poolId: number]: PoolStats,
    },
    poolStatsValue: {
        [poolId: number]: PoolStatsValue
    },
    putPoolStats: (poolId: number, stats: Partial<PoolStats>) => void;
    putPoolStatsValue: (poolId: number, stats: Partial<PoolStatsValue>) => void;
}