import { IKeyringPair } from "@polkadot/types/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { u32, u128, Option } from "@polkadot/types-codec";
import { expect } from "chai";
import { ITuple } from "@polkadot/types-codec/types";
import { ComposableTraitsBondedFinanceBondOffer } from "@composable/types/interfaces";
import { AccountId32 } from "@polkadot/types/interfaces";

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

export async function verifyOfferCreation(api: ApiPromise, bondOfferId, requestParameters, bondOfferBeneficiaryWallet) {
  const bondInfo = <Option<ITuple<[AccountId32, ComposableTraitsBondedFinanceBondOffer]>>>(
    await api.query.bondedFinance.bondOffers(bondOfferId)
  );
  expect(bondInfo.unwrap()[1].beneficiary.toString()).to.be.equal(
    api.createType("AccountId32", bondOfferBeneficiaryWallet.publicKey).toString()
  );
  expect(bondInfo.unwrap()[1].asset).to.be.bignumber.equal(requestParameters.asset);
  expect(bondInfo.unwrap()[1].bondPrice).to.be.bignumber.equal(requestParameters.bondPrice);
  expect(bondInfo.unwrap()[1].nbOfBonds).to.be.bignumber.equal(requestParameters.nbOfBonds);
  expect(bondInfo.unwrap()[1].reward.asset.toString()).to.be.equal(requestParameters.reward.asset.toString());
  expect(bondInfo.unwrap()[1].reward.amount.toString()).to.be.equal(requestParameters.reward.amount.toString());
  expect(bondInfo.unwrap()[1].reward.maturity.toString()).to.be.equal(requestParameters.reward.maturity.toString());
}
