import { IKeyringPair } from "@polkadot/types/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";

/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128} nbOfBonds
 */

export async function txBondedFinanceBondSuccessTest(wallet: IKeyringPair, offerId, nbOfBonds) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.bondedFinance.NewBond.is,
    api.tx.bondedFinance.bond(offerId, nbOfBonds, true)
  );
}

/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128} nbOfBonds
 */
export async function txBondedFinanceBondFailureTest(wallet: IKeyringPair, offerId, nbOfBonds) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.system.ExtrinsicFailed.is,
    api.tx.bondedFinance.bond(offerId, nbOfBonds, true),
    true
  );
}