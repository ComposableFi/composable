import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "@/stores/types";
import BigNumber from "bignumber.js";

export type RewardPool = {
  owner: string;
  assetId: number;
  totalShares: BigNumber;
  claimedShares: BigNumber;
  endBlock: BigNumber;
  lock: {
    durationPresets: {
      [key in string]: BigNumber;
    };
    unlockPenalty: string;
  };
  shareAssetId: string;
  financialNftAssetId: string;
};

export type StakingRewardsSlice = {
  rewardPools: {
    [key in number]: RewardPool;
  };
  setRewardPool: (assetId: number, pool: RewardPool) => void;
};
const initialState = {
  rewardPools: {
    1: {
      owner: "",
      assetId: 1,
      totalShares: new BigNumber(0),
      claimedShares: new BigNumber(0),
      endBlock: new BigNumber(0),
      lock: {
        durationPresets: {},
        unlockPenalty: "",
      },
      shareAssetId: "",
      financialNftAssetId: "",
    },
  },
};

export const createStakingRewardsSlice: StoreSlice<StakingRewardsSlice> = (
  set: NamedSet<StakingRewardsSlice>
) => ({
  ...initialState,
  setRewardPool: (assetId: number, pool: RewardPool) =>
    set((state) => {
      state.rewardPools[assetId] = pool;

      return state;
    }),
});
