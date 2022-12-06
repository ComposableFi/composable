import BigNumber from "bignumber.js";

export type RewardPoolRewardRatePeriod = "PerSecond";

export type StakingRewardPoolRewardRate = {
  period: RewardPoolRewardRatePeriod;
  amount: BigNumber;
};

export type StakingRewardPoolRewardConfig = {
  assetId: BigNumber;
  claimedRewards: BigNumber;
  lastUpdatedTimestamp: number;
  maxRewards: BigNumber;
  rewardRate: StakingRewardPoolRewardRate;
  totalDilutionAdjustment: BigNumber;
  totalRewards: BigNumber;
};

export type StakingRewardPoolLockConfig = {
  durationPresets: Record<string, BigNumber>;
  unlockPenalty: BigNumber;
};

export type StakingRewardPool = {
  owner: string;
  rewards: Record<string, StakingRewardPoolRewardConfig>;
  claimedShares: BigNumber;
  startBlock: BigNumber;
  endBlock: BigNumber;
  lock: StakingRewardPoolLockConfig;
  shareAssetId: string;
  financialNftAssetId: string;
  minimumStakingAmount: BigNumber;
};

export type Stake = {
  fnftInstanceId: string;
  lock: {
    duration: BigNumber;
    startedAt: BigNumber;
    unlockPenalty: BigNumber;
  };
  reductions: Record<string, BigNumber>;
  rewardPoolId: string;
  share: BigNumber;
  stake: BigNumber;
};

export interface StakingPositionHistory {
  startTimestamp: string;
  fnftCollectionId: string;
  fnftInstanceId: string;
  endTimestamp: string;
  assetId: string;
  amount:string;
  owner: string;
  source: string;
  id: string;
}

export interface StakedFinancialNftPosition {
  lockedPrincipalAsset: BigNumber;
  nftId: string;
  expiryDate: string;
  isExpired: boolean;
  multiplier: string;
  xTokenBalance: BigNumber;
}
