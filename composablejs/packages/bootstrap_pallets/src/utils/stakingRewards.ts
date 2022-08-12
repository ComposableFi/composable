
import { ApiPromise } from "@polkadot/api";
import config from "@composable/bootstrap_pallets/constants/config.json";
import { StakingRewardsPoolConfig } from "../types/stakingRewards";
import BigNumber from "bignumber.js";

export function toStakingRewardPoolConfig(api: ApiPromise, currentBlock: string, poolConfig: typeof config.stakingRewardPools[number]): StakingRewardsPoolConfig {
  const endBlock = new BigNumber(currentBlock.toString()).plus(poolConfig.endBlock);

  return {
    RewardRateBasedIncentive: {
      assetId: api.createType("u128", poolConfig.assetId),
      // end block of the rewards
      endBlock: api.createType("u32", endBlock.toString()),
      rewardConfigs: {
        // reward asset id
        assetId: api.createType("u128", poolConfig.rewardConfigs.assetId),
        // maximum rewards to be distributed
        maxRewards: api.createType("u128", poolConfig.rewardConfigs.maxRewards),
        rewardRate: {
          period: "PerSecond",
          amount: api.createType("Balance", poolConfig.rewardConfigs.rewardRate.amount)
        }
      },
      locK: {
        // time presets for locking
        durationPresets: api.createType("BTreeMap<u64, Perbill>", poolConfig.locK.durationPresets),
        // early unlock penalty
        unlockPenalty: api.createType("Perbill", poolConfig.locK.unlockPenalty)
      }

    }
  };
}
