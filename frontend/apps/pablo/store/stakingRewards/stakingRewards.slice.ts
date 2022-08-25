import { StakingRewardPool } from "@/defi/types/stakingRewards";
import create from "zustand";

export interface StakingRewardsSlice {
  rewardPools: Record<string, StakingRewardPool>;
}

export const useStakingRewardsSlice = create<StakingRewardsSlice>(() => ({
  rewardPools: {},
}));

export const putStakingRewardPool = (stakingRewardPool: StakingRewardPool) =>
  useStakingRewardsSlice.setState((state) => ({
    ...state,
    rewardPools: {
      ...state.rewardPools,
      [stakingRewardPool.assetId.toString()]: {
        ...stakingRewardPool,
      },
    },
  }));

export const putStakingRewardPools = (
  stakingRewardPools: StakingRewardPool[]
) =>
  useStakingRewardsSlice.setState((state) => ({
    ...state,
    rewardPools: stakingRewardPools.reduce(function (acc, curr) {
      return {
        ...acc,
        [curr.assetId.toString()]: curr,
      };
    }, {} as Record<string, StakingRewardPool>),
  }));

export const useStakingRewardPool = (
  principalAssetId: string
): StakingRewardPool | null =>
  useStakingRewardsSlice().rewardPools[principalAssetId] ?? null;
