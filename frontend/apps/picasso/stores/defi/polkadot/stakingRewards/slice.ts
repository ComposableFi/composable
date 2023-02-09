import BigNumber from "bignumber.js";
import { StoreSlice } from "@/stores/types";
import { StakingPosition } from "@/apollo/queries/stakingPositions";

export type RewardPool = {
  owner: string;
  claimedShares: BigNumber;
  // endBlock: BigNumber;
  lock: {
    durationPresets: {
      [key in string]: BigNumber;
    };
    unlockPenalty: string;
  };
  shareAssetId: string;
  financialNftAssetId: string;
  minimumStakingAmount: BigNumber;
};
export type PortfolioItem = {
  instanceId: string;
  collectionId: string;
  assetId: string;
  endTimestamp: string;
  id: string;
  multiplier: BigNumber;
  share: BigNumber;
  stake: BigNumber;
  unlockPenalty: BigNumber;
};
export type StakingPortfolio = Array<PortfolioItem>;
export type StakingRewardsSlice = {
  isRewardPoolLoaded: boolean;
  rewardPools: {
    [key in string]: RewardPool;
  };
  setRewardPool: (assetId: string, pool: RewardPool) => void;
  stakingPositions: StakingPosition[];
  isStakingPositionsLoading: boolean;
  setStakingPositions: (positions: StakingPosition[]) => void;
  setStakingPositionLoading: (k: boolean) => void;
  stakingPortfolio: StakingPortfolio;
  setStakingPortfolio: (value: StakingPortfolio) => void;
  setClaimableRewards: (key: string, reward: ClaimableReward) => void;
  resetClaimableRewards: () => void;
  claimableRewards: ClaimableRewards;
  isClaimableLoading: boolean;
};

export type ClaimableReward = { assetId: string; balance: BigNumber };
export type ClaimableRewards = {
  [key in string]: ClaimableReward[];
};
const initialState = {
  isRewardPoolLoaded: false,
  isStakingPositionsLoading: false,
  isClaimableLoading: false,
  rewardPools: {
    "1": {
      owner: "",
      assetId: "1",
      totalShares: new BigNumber(0),
      claimedShares: new BigNumber(0),
      endBlock: new BigNumber(0),
      lock: {
        durationPresets: {},
        unlockPenalty: "",
      },
      shareAssetId: "",
      financialNftAssetId: "",
      minimumStakingAmount: new BigNumber(0),
    },
  },
  stakingPositions: [],
  stakingPortfolio: [],
  claimableRewards: {},
};

export const createStakingRewardsSlice: StoreSlice<StakingRewardsSlice> = (
  set,
  get
) => ({
  ...initialState,
  setRewardPool: (assetId: string, pool: RewardPool) =>
    set((state) => {
      state.rewardPools[assetId] = pool;
      state.isRewardPoolLoaded = true;

      return state;
    }),
  setStakingPositionLoading: (status: boolean) =>
    set((state) => {
      state.isStakingPositionsLoading = status;
    }),
  setStakingPositions: (positions) =>
    set((state) => {
      state.stakingPositions = positions;
    }),
  setStakingPortfolio: (portfolio) =>
    set((state) => {
      state.stakingPortfolio = portfolio;
    }),

  setClaimableRewards: (
    key: string,
    reward: {
      assetId: string;
      balance: BigNumber;
    }
  ) =>
    set((state) => {
      state.isClaimableLoading = false;
      if (Array.isArray(state.claimableRewards[key])) {
        state.claimableRewards[key] = [...get().claimableRewards[key], reward];
      }

      state.claimableRewards[key] = [reward];
    }),

  resetClaimableRewards: () => {
    set((state) => {
      state.claimableRewards = {};
      state.isClaimableLoading = true;
    });
  },
});
