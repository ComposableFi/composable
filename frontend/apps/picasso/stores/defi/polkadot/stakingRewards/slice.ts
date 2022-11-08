import BigNumber from "bignumber.js";
import { StoreSlice } from "@/stores/types";
import { StakingPosition } from "@/apollo/queries/stakingPositions";

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
export type PortfolioItem = {
  instanceId: string;
  collectionId: string;
  assetId: string;
  endTimestamp: BigInt;
  id: string;
  multiplier: BigNumber;
  share: BigNumber;
  stake: BigNumber;
  unlockPenalty: BigNumber;
};
export type PortfolioTuple = [string, string, PortfolioItem];
export type StakingPortfolio = Array<PortfolioItem>;
export type StakingRewardsSlice = {
  // TODO: key better to be in string as we might have types not cast-able to number.
  rewardPools: {
    [key in number]: RewardPool;
  };
  setRewardPool: (assetId: number, pool: RewardPool) => void;
  stakingPositions: StakingPosition[];
  isStakingPositionsLoadingState: boolean;
  setStakingPositions: (positions: StakingPosition[]) => void;
  setStakingPositionLoadingState: (k: boolean) => void;
  stakingPortfolio: StakingPortfolio;
  setStakingPortfolio: (value: StakingPortfolio) => void;
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
  stakingPositions: [],
  isStakingPositionsLoadingState: false,
  stakingPortfolio: [],
};

export const createStakingRewardsSlice: StoreSlice<StakingRewardsSlice> = (
  set
) => ({
  ...initialState,
  setRewardPool: (assetId: number, pool: RewardPool) =>
    set((state) => {
      state.rewardPools[assetId] = pool;

      return state;
    }),
  setStakingPositionLoadingState: (status: boolean) =>
    set((state) => {
      state.isStakingPositionsLoadingState = status;
    }),
  setStakingPositions: (positions) =>
    set((state) => {
      state.stakingPositions = positions;
    }),
  setStakingPortfolio: (portfolio) =>
    set((state) => {
      state.stakingPortfolio = portfolio;
    }),
});
