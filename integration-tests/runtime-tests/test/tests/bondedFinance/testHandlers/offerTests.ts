import { IKeyringPair } from "@polkadot/types/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";

/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {} requestParameters wallet public key
 */
export async function txBondedFinanceOfferSuccessTest(wallet: IKeyringPair, requestParameters) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.bondedFinance.NewOffer.is,
    api.tx.bondedFinance.offer(requestParameters, true)
  );
}

/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {} requestParameters wallet public key
 */
export async function txBondedFinanceOfferFailureTest(wallet: IKeyringPair, requestParameters) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.system.ExtrinsicFailed.is,
    api.tx.bondedFinance.offer(requestParameters, true),
    true
  );
}
