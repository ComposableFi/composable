import { LiquidityBootstrappingPool } from "@/defi/types";
import BigNumber from "bignumber.js";
import create from "zustand";
import { AuctionsSlice } from "./auctions.types";

const DEFAULT_STATS = {
  totalVolume: new BigNumber(0),
  totalLiquidity: new BigNumber(0),
  liquidity: {
    baseAmount: new BigNumber(0),
    quoteAmount: new BigNumber(0),
  },
  startLiquidity: {
    baseAmount: new BigNumber(0),
    quoteAmount: new BigNumber(0),
  },
  totalSold: new BigNumber(0),
  totalRaised: new BigNumber(0)
};

const DEFAULT_POOL: LiquidityBootstrappingPool = {
  id: "-",
  poolId: -1,
  owner: "-",
  pair: {
    base: -1,
    quote: -1,
  },
  sale: {
    startBlock: "0",
    endBlock: "0",
    start: Date.now() - 30 * 24 * 60 * 60 * 1000,
    end: Date.now() + 30 * 24 * 60 * 60 * 1000,
    duration: 60,
    initialWeight: 0,
    finalWeight: 0,
  },
  feeConfig: {
    feeRate: "1",
    protocolFeeRate: "1",
    ownerFeeRate: "1"
  },
  networkId: "picasso",
  auctionDescription: [],
};

export const useAuctionsSlice = create<AuctionsSlice>(() => ({
  activePool: DEFAULT_POOL,
  activePoolStats: DEFAULT_STATS,
  spotPrices: {},
  activePoolTradeHistory: []
}));

export const setAuctionsSlice = (auctionsState: Partial<AuctionsSlice>) => useAuctionsSlice.setState((state) => ({
  ...state,
  ...auctionsState
}))

export const setAuctionsSpotPrices = (spotPrices: Record<string, BigNumber>) => useAuctionsSlice.setState((state) => ({
  activePool: state.activePool,
  activePoolStats: state.activePoolStats,
  activePoolTradeHistory: state.activePoolTradeHistory,
  spotPrices: { ... spotPrices }
}));

export const setAuctionsSpotPrice = (auctionPoolId: string, spotPrice: BigNumber) => useAuctionsSlice.setState((state) => ({
  activePool: state.activePool,
  activePoolStats: state.activePoolStats,
  activePoolTradeHistory: state.activePoolTradeHistory,
  spotPrices: { ... state.spotPrices, [auctionPoolId]: spotPrice }
}));