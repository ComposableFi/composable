import { StakingPosition } from "@/defi/subsquid/stakingRewards/queries";
import { StakingRewardPool } from "@/defi/types/stakingRewards";
import BigNumber from "bignumber.js";
import create from "zustand";

export interface StakingRewardsSlice {
  rewardPools: Record<string, StakingRewardPool>;
  rewardPoolStakedPositions: Record<string, Array<StakingPosition>>;
  pabloStaking: {
    totalPBLOLocked: BigNumber;
    totalFnftMinted: BigNumber;
    averageLockMultiplier: BigNumber;
    averageLockTime: BigNumber;
  };
}

export const useStakingRewardsSlice = create<StakingRewardsSlice>(() => ({
  rewardPools: {},
  rewardPoolStakedPositions: {},
  pabloStaking: {
    totalPBLOLocked: new BigNumber(0),
    totalFnftMinted: new BigNumber(0),
    averageLockMultiplier: new BigNumber(0),
    averageLockTime: new BigNumber(0),
  },
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

export const putStakingRewardPoolStakedPositions = (
  stakingRewardPositions: Record<string, Array<StakingPosition>>
) =>
  useStakingRewardsSlice.setState((state) => ({
    ...state,
    rewardPoolStakedPositions: stakingRewardPositions
  }));

export const useStakingRewardPool = (
  principalAssetId: string
): StakingRewardPool | null =>
  useStakingRewardsSlice().rewardPools[principalAssetId] ?? null;


export const useStakedPositions = (
    principalAssetId: string
): StakingPosition[] =>
    useStakingRewardsSlice().rewardPoolStakedPositions[principalAssetId] ?? [] as StakingPosition[];
