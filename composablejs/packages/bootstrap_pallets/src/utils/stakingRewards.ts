import { ApiPromise } from "@polkadot/api";
import { StakingRewardsPoolConfig } from "../types/stakingRewards";
import { KeyringPair } from "@polkadot/keyring/types";
import config from "@composable/bootstrap_pallets/constants/config.json";
import BigNumber from "bignumber.js";

export function toStakingRewardPoolConfig(
  api: ApiPromise,
  currentBlock: string,
  owner: KeyringPair,
  poolConfig: typeof config.stakingRewardPools[number],
  startDelayBlocks = 50
): StakingRewardsPoolConfig {
  const endBlock = new BigNumber(currentBlock).plus(poolConfig.endBlock).plus(startDelayBlocks);
  const startBlock = new BigNumber(currentBlock).plus(startDelayBlocks);

  let initialConfig = { ... poolConfig.rewardConfigs };
  Object.keys(initialConfig).forEach((key: string) => {
    (initialConfig as any)[key].rewardRate.amount = "0";
  });

  return {
    RewardRateBasedIncentive: {
      owner: owner.publicKey,
      assetId: api.createType("u128", poolConfig.assetId),
      startBlock: api.createType("u32", startBlock.toString()),
      // end block of the rewards
      endBlock: api.createType("u32", endBlock.toString()),
      rewardConfigs: api.createType("BTreeMap<u128, ComposableTraitsStakingRewardConfig>", initialConfig),
      lock: api.createType("ComposableTraitsStakingLockLockConfig", {
        // time presets for locking
        durationPresets: api.createType("BTreeMap<u64, Perbill>", poolConfig.lock.durationPresets),
        // early unlock penalty
        unlockPenalty: api.createType("Perbill", poolConfig.lock.unlockPenalty)
      }),
      financialNftAssetId: api.createType("u128", poolConfig.financialNftAssetId),
      shareAssetId: api.createType("u128", poolConfig.shareAssetId),
      minimumStakingAmount: api.createType("u128", poolConfig.minimumStakingAmount)
    }
  };
}
