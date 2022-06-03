import produce from "immer";
import { LiquidityBootstrappingPool, LiquidityBootstrappingPoolStats } from "@/store/pools/pools.types";
import { AuctionsSlice, PoolTradeHistory } from "./auctions.types";

export const setActivePool = (
  lbpState: AuctionsSlice["auctions"],
  lbPool: LiquidityBootstrappingPool
) => {
  return produce(lbpState, (draft) => {
    draft.activeLBP = lbPool;
  });
};

export const putAuctionStatsActiveLBP = (
  lbpState: AuctionsSlice["auctions"],
  stats: Partial<LiquidityBootstrappingPoolStats>
) => {
  return produce(lbpState, (draft) => {
    draft.activeLBPStats.totalSold = stats.totalSold ?? "0";
    draft.activeLBPStats.totalRaised = stats.totalRaised ?? "0";
    draft.activeLBPStats.totalVolume = stats.totalVolume ?? "0";
    draft.activeLBPStats.liquidity = stats.liquidity ?? "0";
    draft.activeLBPStats.startBalances = stats.startBalances ?? lbpState.activeLBPStats.startBalances;
    draft.activeLBPStats.currentBalances = stats.currentBalances ?? lbpState.activeLBPStats.currentBalances;
  });
}

export const putAuctionHistoryLBP = (
  lbpState: AuctionsSlice["auctions"],
  history: PoolTradeHistory[]
) => {
  return produce(lbpState, (draft) => {
    draft.activeLBPHistory = [...history];
  });
}

export const putChartSeries = (
  lbpState: AuctionsSlice["auctions"],
  series: "price" | "predicted",
  data: [number, number][]
) => {
  return produce(lbpState, (draft) => {
    if (series === "predicted") draft.activeChart.predicted = data;
    if (series === "price") draft.activeChart.price = data;
  });
}