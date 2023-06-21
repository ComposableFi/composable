import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";

/**
 * Provides funds for Oracle tests.
 * @param api Connect ApiPromise
 * @param sudoKey KeyringPair with sudo rights
 * @param wallet1 Wallet to provide funds to
 * @param wallet2 Wallet to provide funds to
 */
export async function runBeforeTxOracleSetSigner(api: ApiPromise, sudoKey: KeyringPair, signer: KeyringPair) {
  return await mintAssetsToWallet(api, signer, sudoKey, [1]);
}

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param controller Keyring which is a controller.
 * @param signer Keyring which will be set as a signer.
 */
export async function txOracleSetSignerSuccessTest(api: ApiPromise, controller: KeyringPair, signer: KeyringPair) {
  return await sendAndWaitForSuccess(
    api,
    controller,
    api.events.oracle.SignerSet.is,
    api.tx.oracle.setSigner(signer.address),
    false
  );
}
