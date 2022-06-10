import { LiquidityBootstrappingPool, LiquidityBootstrappingPoolStats } from "../pools/pools.types";
import { StoreSlice } from "../types";
import { AuctionsSlice, PoolTradeHistory } from "./auctions.types";
import {
  putAuctionStatsActiveLBP,
  setActivePool,
  putAuctionHistoryLBP,
  putChartSeries,
} from "./auctions.utils";

const PLACEHOLDER_STATS: LiquidityBootstrappingPoolStats = {
  totalVolume: "0",
  liquidity: "0",
  startBalances: {
    quote: "0",
    base: "0",
  },
  currentBalances: {
    quote: "0",
    base: "0",
  },
  totalSold: "",
  totalRaised: "",
};

const PLACEHOLDER_POOL: LiquidityBootstrappingPool = {
  id: "-",
  poolId: -1,
  icon: "",
  owner: "-",
  pair: {
    base: 129,
    quote: 1,
  },
  sale: {
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
  spotPrice: "0",
  networkId: "picasso",
  auctionDescription: [],
};

const createAuctionsSlice: StoreSlice<AuctionsSlice> = (set) => ({
  auctions: {
    activeLBP: PLACEHOLDER_POOL,
    activeLBPStats: PLACEHOLDER_STATS,
    activeLBPHistory: [],
    activeChart: {
      price: [],
      predicted: [],
    },
  },
  setActiveAuctionsPool: (lbPool: LiquidityBootstrappingPool) =>
    set((prev: AuctionsSlice) => ({
      auctions: setActivePool(prev.auctions, lbPool),
    })),
  putStatsActiveLBP: (stats: Partial<LiquidityBootstrappingPoolStats>) =>
    set((prev: AuctionsSlice) => ({
      auctions: putAuctionStatsActiveLBP(prev.auctions, stats),
    })),
  putHistoryActiveLBP: (history: PoolTradeHistory[]) =>
    set((prev: AuctionsSlice) => ({
      auctions: putAuctionHistoryLBP(prev.auctions, history),
    })),
  resetActiveLBP: () =>
    set((_prev: AuctionsSlice) => ({
      auctions: {
        activeLBP: PLACEHOLDER_POOL,
        activeLBPStats: PLACEHOLDER_STATS,
        activeLBPHistory: [],
        activeChart: {
          price: [],
          predicted: []
        }
      },
    })),
  putChartSeries: (series: "price" | "predicted", data: [number, number][]) =>
    set((prev: AuctionsSlice) => ({
      auctions: putChartSeries(prev.auctions, series, data),
    })),
});

export default createAuctionsSlice;
