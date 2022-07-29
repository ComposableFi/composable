import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";

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
