import { ApiPromise } from "@polkadot/api";
import config from "@composable/bootstrap_pallets/constants/config.json";
import { StakingRewardsPoolConfig } from "../types/stakingRewards";
import BigNumber from "bignumber.js";
import { KeyringPair } from "@polkadot/keyring/types";

export function toStakingRewardPoolConfig(
  api: ApiPromise,
  currentBlock: string,
  owner: KeyringPair,
  poolConfig: typeof config.stakingRewardPools[number]
): StakingRewardsPoolConfig {
  const endBlock = new BigNumber(currentBlock.toString()).plus(poolConfig.endBlock);

  return {
    RewardRateBasedIncentive: {
      owner: owner.publicKey,
      assetId: api.createType("u128", poolConfig.assetId),
      // end block of the rewards
      endBlock: api.createType("u32", endBlock.toString()),
      rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", poolConfig.rewardConfigs),
      lock: api.createType("ComposableTraitsStakingLockLockConfig", {
        // time presets for locking
        durationPresets: api.createType("BTreeMap<u64, Perbill>", poolConfig.locK.durationPresets),
        // early unlock penalty
        unlockPenalty: api.createType("Perbill", poolConfig.locK.unlockPenalty)
      })
    }
  };
}
