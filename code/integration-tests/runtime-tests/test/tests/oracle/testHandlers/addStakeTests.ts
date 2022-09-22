import { u128 } from "@polkadot/types-codec";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

/**
 * Provides funds for Oracle tests.
 * @param api Connect ApiPromise
 * @param sudoKey KeyringPair with sudo rights
 * @param wallet1 Wallet to provide funds to
 * @param wallet2 Wallet to provide funds to
 */
export async function runBeforeTxOracleAddStake(
  api: ApiPromise,
  sudoKey: KeyringPair,
  wallet1: KeyringPair,
  wallet2: KeyringPair
) {
  await mintAssetsToWallet(api, wallet1, sudoKey, [1]);
  await mintAssetsToWallet(api, wallet2, sudoKey, [1]);
}

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
