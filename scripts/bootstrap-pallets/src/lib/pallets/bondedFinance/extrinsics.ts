import { IKeyringPair } from "@polkadot/types/types";
import { ApiPromise } from "@polkadot/api";
import { u32, u64, u128 } from "@polkadot/types-codec";
import { sendAndWaitForSuccess } from "@bootstrap-pallets/lib";

/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param requestParameters wallet public key
 * @return Transaction event.
 */
export async function createOffer(
  api: ApiPromise,
  wallet: IKeyringPair,
  requestParameters: {
    beneficiary: Uint8Array;
    asset: u128;
    bondPrice: u128;
    nbOfBonds: u128;
    maturity: { Finite: { returnIn: u32 } } | "Infinite";
    reward: {
      asset: u128;
      amount: u128;
      maturity: u32;
    };
  }
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.bondedFinance.NewOffer.is,
    api.tx.bondedFinance.offer(requestParameters, false)
  );
}

/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 *
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128|number} nbOfBonds
 * @return Transaction event.
 */
export async function txBondedFinanceBondSuccessTest(
  api: ApiPromise,
  wallet: IKeyringPair,
  offerId: u64,
  nbOfBonds: u128 | number
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.bondedFinance.NewBond.is,
    api.tx.bondedFinance.bond(offerId, nbOfBonds, true)
  );
}
