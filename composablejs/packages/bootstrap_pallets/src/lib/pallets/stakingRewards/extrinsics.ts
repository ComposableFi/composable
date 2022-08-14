import { IKeyringPair } from "@polkadot/types/types";
import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/bootstrap_pallets/lib";
import { StakingRewardsPoolConfig } from "@composable/bootstrap_pallets/types/stakingRewards";

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
      api.tx.stakingRewards.createRewardPool(api.createType("ComposableTraitsStakingRewardPoolConfiguration", rewardPoolConfig))
    )
  );
}
