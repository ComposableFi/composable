import { u128 } from "@polkadot/types-codec";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param api Connect ApiPromise
 * @param sender Connected API Promise w/ sudo rights.
 * @param {u128} stake Staking amount.
 */
export async function txOracleAddStakeSuccessTest(api: ApiPromise, sender: KeyringPair, stake: u128) {
  return await sendAndWaitForSuccess(
    api,
    sender,
    api.events.oracle.StakeAdded.is,
    api.tx.oracle.addStake(stake),
    false
  );
}
