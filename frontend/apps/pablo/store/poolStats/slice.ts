import { StoreSlice } from "../types";
import { PoolStatsSlice } from "./types";

const createPoolStatsSlice: StoreSlice<PoolStatsSlice> = (set) => ({
  poolStats: {},
  poolStatsValue: {},
  putPoolStats: (poolId: number, poolStats) =>
    set((state) => {
      let stats: PoolStatsSlice["poolStats"][number] = {
        totalVolume: poolStats.totalVolume
          ? poolStats.totalVolume
          : state.poolStats[poolId]
          ? state.poolStats[poolId].totalVolume
          : "0",
        _24HrFee: poolStats._24HrFee
          ? poolStats._24HrFee
          : state.poolStats[poolId]
          ? state.poolStats[poolId]._24HrFee
          : "0",
        _24HrVolume: poolStats._24HrVolume
          ? poolStats._24HrVolume
          : state.poolStats[poolId]
          ? state.poolStats[poolId]._24HrVolume
          : "0",
        _24HrTransactionCount: poolStats._24HrTransactionCount
          ? poolStats._24HrTransactionCount
          : state.poolStats[poolId]
          ? state.poolStats[poolId]._24HrTransactionCount
          : 0,
        dailyRewards: [],
        apr: poolStats.apr
          ? poolStats.apr
          : state.poolStats[poolId]
          ? state.poolStats[poolId].apr
          : "0",
      };

      state.poolStats[poolId] = stats;
      return state;
    }),
  putPoolStatsValue: (poolId: number, poolStatsValue) =>
    set((state) => {
      let stats: PoolStatsSlice["poolStatsValue"][number] = {
        _24HrFeeValue: poolStatsValue._24HrFeeValue
          ? poolStatsValue._24HrFeeValue
          : state.poolStatsValue[poolId]
          ? state.poolStatsValue[poolId]._24HrFeeValue
          : "0",
        _24HrVolumeValue: poolStatsValue._24HrVolumeValue
          ? poolStatsValue._24HrVolumeValue
          : state.poolStatsValue[poolId]
          ? state.poolStatsValue[poolId]._24HrVolumeValue
          : "0",
        totalVolumeValue: poolStatsValue.totalVolumeValue
          ? poolStatsValue.totalVolumeValue
          : state.poolStatsValue[poolId]
          ? state.poolStatsValue[poolId].totalVolumeValue
          : "0",
      };
      state.poolStatsValue[poolId] = stats;
      return state;
    }),
});

export default createPoolStatsSlice;
