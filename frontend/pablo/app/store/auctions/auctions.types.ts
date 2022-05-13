import { LiquidityBootstrappingPool, LiquidityBootstrappingPoolStats } from "../pools/liquidityBootstrapping/liquidityBootstrapping.types";

export enum LiquidityBootstrappingPoolTransactionType { "SWAP", "ADD_LIQUIDITY","CREATE_POOL" };
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
        activeChart: {
            price: [number, number][],
            predicted: [number, number][]
        }
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
    putChartSeries: (
        series: "price" | "predicted",
        data: [number, number][]   
    ) => void;
    resetActiveLBP: () => void;
}