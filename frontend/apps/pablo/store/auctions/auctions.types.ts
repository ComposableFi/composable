import { PabloLiquidityBootstrappingPool } from "shared";
import BigNumber from "bignumber.js";

export type LiquidityBootstrappingPoolStatistics = {
  totalVolume: BigNumber;
  totalLiquidity: BigNumber;
  liquidity: {
    baseAmount: BigNumber;
    quoteAmount: BigNumber;
  };
  startLiquidity: {
    baseAmount: BigNumber;
    quoteAmount: BigNumber;
  };
};
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
  activePool: PabloLiquidityBootstrappingPool | null;
  activePoolStats: LiquidityBootstrappingPoolStatistics;
  spotPrices: Record<string, BigNumber>;
  activePoolTradeHistory: PoolTradeHistory[];
}