export interface LiquidityBootstrappingPoolTrade {
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