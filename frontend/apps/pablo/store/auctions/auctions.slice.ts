import { AuctionsSlice } from "./auctions.types";
import BigNumber from "bignumber.js";
import create from "zustand";

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

export const useAuctionsSlice = create<AuctionsSlice>(() => ({
  activePool: null,
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