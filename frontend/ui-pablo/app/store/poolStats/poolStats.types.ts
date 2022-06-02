import { AssetId } from "@/defi/polkadot/types";

export interface DailyRewards {
    symbol: string;
    icon: string;
    name: string;
    assetId: AssetId;
    rewardAmount: string;
    rewardAmountLeft: string;
}

export interface PoolStats {
    totalVolume: string;
    totalValueLocked: string;
    apr: string;
    _24HrFee: string;
    _24HrVolume: string;
    _24HrTransactionCount: number;
    dailyRewards: DailyRewards[]; 
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
    putPoolStats: (poolId: number, stats: Partial<PoolStats>) => void;
}