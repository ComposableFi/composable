export default {
    rpc: {},
    types: {
  ​
      ComposableTraitsStakingRewardPool: "Null",
      ComposableTraitsStakingRewardUpdate: "Null",
      ComposableTraitsStakingRewardConfig: "Null",
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
      }
    }
  };