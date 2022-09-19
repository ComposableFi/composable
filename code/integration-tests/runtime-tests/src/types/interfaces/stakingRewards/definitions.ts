export default {
  rpc: {},
  types: {
    ComposableTraitsStakingRewardPool: {
      owner: "AccountId32",
      assetId: "u128",
      rewards: "BTreeMap<u128, ComposableTraitsStakingRewardConfig>",
      totalShares: "u128",
      claimedShares: "u128",
      startBlock: "u32",
      endBlock: "u32",
      lock: "ComposableTraitsStakingLockLockConfig",
      shareAssetId: "u128",
      financialNftAssetId: "u128",
      minimumStakingAmount: "u128"
    },
    ComposableTraitsStakingRewardUpdate: "Null",
    ComposableTraitsStakingRewardConfig: {
      totalRewards: "u128",
      claimedRewards: "u128",
      totalDilutionAdjustment: "u128",
      maxRewards: "u32",
      rewardRate: "Null",//"ComposableTraitsStakingRewardRate",
      lastUpdatedTimestamp: "u64"
    },
    ComposableTraitsStakingLockLockConfig: {
      durationPresets: "BTreeMap<u64, Perbill>",
      unlockPenalty: "Perbill"
    },
    ComposableTraitsStakingRewardPoolConfiguration: {
      RewardRateBasedIncentive: {
        owner: "AccountId32",
        assetId: "u128",
        endBlock: "u32",
        rewardConfigs: "BTreeMap<u128, ComposableTraitsStakingRewardConfig>",
        lock: "ComposableTraitsStakingLockLockConfig"
      }
    },
    ComposableTraitsStakingStake: {
      owner: "AccountId",
      rewardPoolId: "u16",
      stake: "Balance",
      share: "Balance",
      reductions: "BoundedBTreeMap<AssetId, Balance, Limit>",
      lock: "ComposableTraitsStakingLockLockConfig"
    },
    PalletStakingRewardsRewardAccumulationHookError: "Null"
  }
};
