import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";


export async function runBeforeTxOracleSetSigner(sudoKey, signer) {
  return await mintAssetsToWallet(
    signer,
    sudoKey,
    [1]
  );
}

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param controller Keyring which is a controller.
 * @param signer Keyring which will be set as a signer.
 */
export async function txOracleSetSignerSuccessTest(controller, signer) {
  return await sendAndWaitForSuccess(
    api,
    controller,
    api.events.oracle.SignerSet.is,
    api.tx.oracle.setSigner(signer.address),
    false
  );
}
