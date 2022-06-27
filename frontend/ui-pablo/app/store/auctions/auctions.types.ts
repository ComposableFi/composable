import { LiquidityBootstrappingPool } from "@/defi/types";
import { LiquidityBootstrappingPoolStats } from "@/store/pools/pools.types";
export interface PoolTradeHistory {
    baseAssetAmount: string;
    baseAssetId: number;
    id: string;
    quoteAssetAmount: string;
    quoteAssetId: number;
    receivedTimestamp: number;
    spotPrice: string;
    side: "SELL" | "BUY";
    walletAddress: string;
}
export interface AuctionsSlice {
    auctions: {
        activeLBP: LiquidityBootstrappingPool;
        activeLBPStats: LiquidityBootstrappingPoolStats;
        activeLBPHistory: PoolTradeHistory[];
    }
    setActiveAuctionsPool: (
        lbPool: LiquidityBootstrappingPool
    ) => void;
    putStatsActiveLBP: (
        auctionStats: Partial<LiquidityBootstrappingPoolStats>
    ) => void;
    putHistoryActiveLBP: (
        auctionHistory: PoolTradeHistory[]
    ) => void;
    resetActiveLBP: () => void;
}