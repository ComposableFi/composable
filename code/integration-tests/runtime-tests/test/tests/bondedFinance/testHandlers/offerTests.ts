import { IKeyringPair } from "@polkadot/types/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { u128, u32 } from "@polkadot/types-codec";

/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param requestParameters wallet public key
 * @return Transaction event.
 */
export async function txBondedFinanceOfferSuccessTest(
  api: ApiPromise,
  wallet: IKeyringPair,
  requestParameters: {
    beneficiary: Uint8Array;
    asset: u128;
    bondPrice: u128;
    nbOfBonds: u128;
    maturity: { Finite: { returnIn: u32 } };
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
    api.tx.bondedFinance.offer(requestParameters, true)
  );
}

/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param requestParameters wallet public key
 * @return Transaction event.
 */
export async function txBondedFinanceOfferFailureTest(api: ApiPromise, wallet: IKeyringPair, requestParameters: any) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.system.ExtrinsicFailed.is,
    api.tx.bondedFinance.offer(requestParameters, true),
    true
  );
}
