import { u128, u32, BTreeMap, u64 } from "@polkadot/types-codec";
import { Perbill } from "@polkadot/types/interfaces";

export type StakingRewardsPoolConfig = {
  RewardRateBasedIncentive: {
    owner: Uint8Array;
    // asset that will be staked
    assetId: u128;
    // end block of the rewards
    endBlock: u32;
    rewardConfigs: any;
    locK: {
      // time presets for locking
      durationPresets: BTreeMap<u64, Perbill>;
      // early unlock penalty
      unlockPenalty: Perbill;
    };
  }
};
