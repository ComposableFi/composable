import { u128, u32, BTreeMap, u64 } from "@polkadot/types-codec";
import { Perbill } from "@polkadot/types/interfaces";

export type StakingRewardsPoolConfig = {
  RewardRateBasedIncentive: {
    // asset that will be staked
    assetId: u128;
    // end block of the rewards
    endBlock: u32;
    rewardConfigs: {
      // reward asset id
      assetId: u128;
      // maximum rewards to be distributed
      maxRewards: u128;
      rewardRate: {
        // reward hand out tick
        period: u128; // enum RewardRatePeriod::PerSecond
        // amount per tick
        amount: u128;
      } | any;
    };
    locK: {
      // time presets for locking
      durationPresets: BTreeMap<u64, Perbill>;
      // early unlock penalty
      unlockPenalty: Perbill;
    };
  }
};
