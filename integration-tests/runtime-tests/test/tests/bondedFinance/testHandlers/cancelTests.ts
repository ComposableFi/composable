/**
 * Tests tx.bondedFinance.cancel with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 */
import { IKeyringPair } from "@polkadot/types/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";

export async function txBondedFinanceCancelSuccessTest(wallet: IKeyringPair, offerId) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.bondedFinance.OfferCancelled.is,
    api.tx.bondedFinance.cancel(offerId)
  );
}

/**
 * Tests tx.bondedFinance.cancel with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 */
export async function txBondedFinanceCancelFailureTest(wallet: IKeyringPair, offerId) {
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
 * @param {IKeyringPair} wallet Connected API Promise w/ sudo rights.
 * @param {u64} offerId
 */
export async function txBondedFinanceCancelSudoSuccessTest(wallet: IKeyringPair, offerId) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.sudo.Sudid.is, api.tx.sudo.sudo(
      api.tx.bondedFinance.cancel(offerId)
    )
  );
}