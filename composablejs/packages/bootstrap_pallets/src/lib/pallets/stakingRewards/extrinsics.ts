import { IKeyringPair } from "@polkadot/types/types";
import { ApiPromise } from "@polkadot/api";
import { u32, u128, u64, BTreeMap } from "@polkadot/types-codec";
import { sendAndWaitForSuccess } from "@composable/bootstrap_pallets/lib";
import { Perbill } from "@polkadot/types/interfaces";

/**
 * Create a staking reward pool.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param rewardPoolConfigs create staking pool config.
 */
export async function createRewardPool(
  api: ApiPromise,
  wallet: IKeyringPair,
  rewardPoolConfig: {
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
      };
    },
    locK: {
      // time presets for locking
      durationPresets: BTreeMap<u64, Perbill>,
      // early unlock penalty
      unlockPenalty: Perbill
    }
  }
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.stakingRewards.RewardPoolCreated.is,
    api.tx.stakingRewards.createRewardPool({
        RewardRateBasedIncentive: {
            owner: wallet.publicKey,
            ...rewardPoolConfig
        }
    })
  );
}