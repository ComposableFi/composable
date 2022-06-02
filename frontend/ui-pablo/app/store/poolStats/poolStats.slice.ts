import { StoreSlice } from "../types";
import { PoolStatsSlice } from "./poolStats.types";
import { putPoolStats } from "./poolStats.utils";

const createPoolStatsSlice: StoreSlice<PoolStatsSlice> = (set) => ({
  poolStats: {},
  putPoolStats: (poolId: number, stats) =>
    set((prev: PoolStatsSlice) => ({
      poolStats: putPoolStats(prev.poolStats, poolId, stats),
    })),
});

export default createPoolStatsSlice;
