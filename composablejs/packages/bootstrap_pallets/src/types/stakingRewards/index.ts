import { u128, u32 } from "@polkadot/types-codec";

export type StakingRewardsPoolConfig = {
  RewardRateBasedIncentive: {
    owner: Uint8Array;
    // asset that will be staked
    assetId: u128;
    // end block of the rewards
    startBlock: u32;
    endBlock: u32;
    rewardConfigs: any;
    lock: any;
    financialNftAssetId: u128,
    shareAssetId: u128,
    minimumStakingAmount: u128
  }
};
