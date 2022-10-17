import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { u128 } from "@polkadot/types-codec";

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param api Connected ApiPromise
 * @param signer Connected API Promise w/ sudo rights.
 * @param price Price to be submitted.
 * @param assetId Specifies asset id.
 */
export async function txOracleSubmitPriceSuccessTest(api: ApiPromise, signer: KeyringPair, price: u128, assetId: u128) {
  return await sendAndWaitForSuccess(
    api,
    signer,
    api.events.oracle.PriceSubmitted.is,
    api.tx.oracle.submitPrice(price, assetId),
    false
  );
}
