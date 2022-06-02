import { DailyRewards, PoolStats, PoolStatsSlice } from "./poolStats.types";
import produce from "immer";

let defaultState = {
  totalVolume: "0",
  totalValueLocked: "0",
  apr: "0",
  _24HrFee: "0",
  _24HrVolume: "0",
  _24HrTransactionCount: 0,
  dailyRewards: [] as DailyRewards[],
  _24HrFeeValue: "0",
  _24HrVolumeValue: "0",
  totalVolumeValue: "0",
};

export const putPoolStats = (
  poolStatsSlice: PoolStatsSlice["poolStats"],
  poolId: number,
  poolStats: Partial<PoolStats>
) => {
  return produce(poolStatsSlice, (draft) => {
    let fallbackState = defaultState;
    if (poolStatsSlice[poolId]) {
      fallbackState = poolStatsSlice[poolId];
    } else {
      poolStatsSlice[poolId] = defaultState;
    }

    draft[poolId].totalVolume =
      poolStats.totalVolume ?? fallbackState.totalVolume;
    draft[poolId].totalValueLocked =
      poolStats.totalValueLocked ?? fallbackState.totalValueLocked;
    draft[poolId].apr = poolStats.apr ?? fallbackState.apr;
    draft[poolId]._24HrFee = poolStats._24HrFee ?? fallbackState._24HrFee;
    draft[poolId]._24HrTransactionCount =
      poolStats._24HrTransactionCount ?? fallbackState._24HrTransactionCount;
    draft[poolId]._24HrVolume =
      poolStats._24HrVolume ?? fallbackState._24HrVolume;
    draft[poolId].dailyRewards =
      poolStats.dailyRewards ?? fallbackState.dailyRewards;
    draft[poolId]._24HrFeeValue =
      poolStats._24HrFeeValue ?? fallbackState._24HrFeeValue;
    draft[poolId]._24HrVolumeValue =
      poolStats._24HrVolumeValue ?? fallbackState._24HrVolumeValue;
    draft[poolId].totalVolumeValue =
      poolStats.totalVolumeValue ?? fallbackState.totalVolumeValue;
  });
};
