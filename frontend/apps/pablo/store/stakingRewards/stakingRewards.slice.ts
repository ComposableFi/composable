import {
  Stake,
  StakingPositionHistory,
  StakingRewardPool,
} from "@/defi/types/stakingRewards";
import create from "zustand";

export interface StakingRewardsSlice {
  rewardPools: Record<string, StakingRewardPool>;
  rewardPoolStakedPositionHistory: Record<
    string,
    Array<StakingPositionHistory>
  >;
  stakes: Record<string, Array<Stake>>;
}

export const useStakingRewardsSlice = create<StakingRewardsSlice>(() => ({
  rewardPools: {},
  rewardPoolStakedPositionHistory: {},
  stakes: {},
}));

export const putStakes = (stakingPoolId: string, stakes: Stake[]) =>
  useStakingRewardsSlice.setState((state) => {
    state.stakes[stakingPoolId] = stakes;
    return state;
  });

export const updateStake = (
  stakingPoolId: string,
  stake: Stake
) =>
  useStakingRewardsSlice.setState((state) => {
    state.stakes[stakingPoolId] = state.stakes[stakingPoolId].map((_stake) => {
      if (stake.fnftInstanceId === _stake.fnftInstanceId) {
        return stake;
      } else {
        return _stake;
      }
    });
    return state;
  });

export const putStakingRewardPool = (stakingRewardPool: StakingRewardPool, assetId: string) =>
  useStakingRewardsSlice.setState((state) => ({
    ...state,
    rewardPools: {
      ...state.rewardPools,
      [assetId]: {
        ...stakingRewardPool,
      },
    },
  }));

export const putStakingRewardPools = (
  stakingRewardPools: Array<{ pool: StakingRewardPool, assetId: string }>
) =>
  useStakingRewardsSlice.setState((state) => ({
    ...state,
    rewardPools: stakingRewardPools.reduce(function(acc, curr) {
      return {
        ...acc,
        [curr.assetId]: curr.pool,
      };
    }, {} as Record<string, StakingRewardPool>),
  }));

export const resetStakingRewardPools = () =>
  useStakingRewardsSlice.setState((state) => ({ ...state, rewardPools: {} }));

export const putStakingRewardPoolStakedPositionsHistory = (
  stakingRewardPositions: Record<string, Array<StakingPositionHistory>>
) =>
  useStakingRewardsSlice.setState((state) => ({
    ...state,
    rewardPoolStakedPositionHistory: stakingRewardPositions,
  }));

export const resetStakingRewardPoolStakedPositionsHistory = () =>
  useStakingRewardsSlice.setState((state) => ({
    ...state,
    rewardPoolStakedPositionHistory: {},
  }));

export const useStakingRewardPool = (
  principalAssetId: string
): StakingRewardPool | undefined =>
  useStakingRewardsSlice().rewardPools[principalAssetId] ?? undefined;

export const useStakes = (principalAssetId: string): Stake[] =>
  useStakingRewardsSlice().stakes[principalAssetId] ?? [];

export const useStakingRewardPoolCollectionId = (
  principalAssetId: string
): string | undefined =>
  useStakingRewardsSlice().rewardPools[principalAssetId]?.financialNftAssetId ??
  undefined;

export const useStakedPositionHistory = (
  principalAssetId: string
): StakingPositionHistory[] =>
  useStakingRewardsSlice().rewardPoolStakedPositionHistory[principalAssetId] ??
  ([] as StakingPositionHistory[]);
