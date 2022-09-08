import { StoreSlice } from "../types";
import { PoolStatsSlice } from "./poolStats.types";
import { putPoolStats, putPoolStatsValue } from "./poolStats.utils";

const createPoolStatsSlice: StoreSlice<PoolStatsSlice> = (set) => ({
  poolStats: {},
  poolStatsValue: {},
  putPoolStats: (poolId: number, stats) =>
    set((prev: PoolStatsSlice) => ({
      poolStats: putPoolStats(prev.poolStats, poolId, stats),
    })),
  putPoolStatsValue: (poolId: number, stats) => set((prev: PoolStatsSlice) => ({
    poolStatsValue: putPoolStatsValue(prev.poolStatsValue, poolId, stats),
  }))
});

export default createPoolStatsSlice;
