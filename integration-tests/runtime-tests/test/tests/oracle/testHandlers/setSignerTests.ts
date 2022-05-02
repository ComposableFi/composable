import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";


export async function runBeforeTxOracleSetSigner(sudoKey, signer) {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(1, signer.publicKey, 555555555555)
    )
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
