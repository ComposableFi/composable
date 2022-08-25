import BigNumber from "bignumber.js";

export type RewardPoolRewardRatePeriod = 
  "PerSecond";

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
  rewardPoolId: BigNumber;
  assetId: BigNumber;
  claimedShares: BigNumber;
  endBlock: BigNumber;
  lock: StakingRewardPoolLockConfig;
  owner: string;
  rewards: Record<string, StakingRewardPoolRewardConfig>;
  totalShares: BigNumber;
};
