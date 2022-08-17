import { IKeyringPair } from "@polkadot/types/types";
import { sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { u64 } from "@polkadot/types-codec";

/**
 * Tests tx.bondedFinance.cancel with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
export async function txBondedFinanceCancelSuccessTest(api: ApiPromise, wallet: IKeyringPair, offerId: u64 | number) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.bondedFinance.OfferCancelled.is,
    api.tx.bondedFinance.cancel(offerId)
  );
}

/**
 * Tests tx.bondedFinance.cancel with provided parameters that should fail.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
export async function txBondedFinanceCancelFailureTest(api: ApiPromise, wallet: IKeyringPair, offerId: u64 | number) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.system.ExtrinsicFailed.is,
    api.tx.bondedFinance.cancel(offerId),
    true
  );
}

/**
 * Tests tx.bondedFinance.cancel as SUDO with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise w/ sudo rights.
 * @param offerIds array of offer ids to cancel
 * @return Transaction event.
 */
export async function txBondedFinanceCancelSudoSuccessTest(
  api: ApiPromise,
  wallet: IKeyringPair,
  offerIds: u64[] | number[]
) {
  const offers = [];
  offerIds.forEach(offerId => {
    offers.push(api.tx.sudo.sudo(api.tx.bondedFinance.cancel(offerId)));
  });
  return await sendWithBatchAndWaitForSuccess(api, wallet, api.events.sudo.Sudid.is, offers, false);
}
