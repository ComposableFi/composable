import { IKeyringPair } from "@polkadot/types/types";
import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/bootstrap_pallets/lib";
import { StakingRewardsPoolConfig } from "@composable/bootstrap_pallets/types/stakingRewards";
import { u128 } from "@polkadot/types-codec";

/**
 * Create a staking reward pool.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param rewardPoolConfigs create staking pool config.
 */
export async function createRewardPool(
  api: ApiPromise,
  wallet: IKeyringPair,
  rewardPoolConfig: StakingRewardsPoolConfig
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.stakingRewards.createRewardPool(
        api.createType("ComposableTraitsStakingRewardPoolConfiguration", rewardPoolConfig)
      )
    )
  );
}

/**
 * Update Reward Config.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u128} assetId principal asset id of the staking pool.
 * @param {Record<string, { rewardRate: { period: "PerSecond", amount: u128 } }>} config reward config.
 */
export async function updateStakingRewardPoolRewardConfig(
  api: ApiPromise,
  wallet: IKeyringPair,
  assetId: string,
  rewardUpdateConfig: Record<
    string,
    {
      rewardRate: {
        period: "PerSecond";
        amount: u128;
      };
    }
  >
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.stakingRewards.RewardPoolUpdated.is,
    api.tx.sudo.sudo(
      api.tx.stakingRewards.updateRewardsPool(
        assetId,
        api.createType("BTreeMap<u128, ComposableTraitsStakingRewardUpdate>", rewardUpdateConfig),
      )
    )
  );
}

/**
 * Add rewards to pot.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {rewardPoolConfig} { poolId: u128, rewardAssetId: u128, rewardAssetAmount: u128}
 * @param {keepAlive} keep account alive or not
 */
 export async function addRewardsToPot(
  api: ApiPromise,
  wallet: IKeyringPair,
  rewardPoolConfig: {
    poolId: u128,
    rewardAssetId: u128,
    rewardAssetAmount: u128,
  },
  keepAlive = true
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.stakingRewards.RewardsPotIncreased.is,
    api.tx.stakingRewards.addToRewardsPot(
      rewardPoolConfig.poolId,
      rewardPoolConfig.rewardAssetId,
      rewardPoolConfig.rewardAssetAmount,
      keepAlive
    )
  );
}