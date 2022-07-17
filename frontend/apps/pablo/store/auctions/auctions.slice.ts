import { LiquidityBootstrappingPool } from "@/defi/types";
import { LiquidityBootstrappingPoolStats } from "../pools/pools.types";
import { StoreSlice } from "../types";
import { AuctionsSlice, PoolTradeHistory } from "./auctions.types";
import {
  putAuctionStatsActiveLBP,
  setActivePool,
  putAuctionHistoryLBP
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
  totalSold: "0",
  totalRaised: "0",
};

const PLACEHOLDER_POOL: LiquidityBootstrappingPool = {
  id: "-",
  poolId: -1,
  owner: "-",
  pair: {
    base: 129,
    quote: 1,
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

const createAuctionsSlice: StoreSlice<AuctionsSlice> = (set) => ({
  auctions: {
    activeLBP: PLACEHOLDER_POOL,
    activeLBPStats: PLACEHOLDER_STATS,
    activeLBPHistory: [],
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
    }))
});

export default createAuctionsSlice;
