import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param signer Connected API Promise w/ sudo rights.
 * @param price Price to be submitted.
 * @param assetId Specifies asset id.
 */
export async function txOracleSubmitPriceSuccessTest(signer, price, assetId) {
  return await sendAndWaitForSuccess(
    api,
    signer,
    api.events.oracle.PriceSubmitted.is,
    api.tx.oracle.submitPrice(price, assetId),
    false
  );
}
