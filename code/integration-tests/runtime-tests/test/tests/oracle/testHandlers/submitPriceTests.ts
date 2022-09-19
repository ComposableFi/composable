import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { bool, Bool, Option, u128 } from "@polkadot/types-codec";
import { AssertionError, expect } from "chai";
import { FrameSystemEventRecord } from "@polkadot/types/lookup";
import { AnyNumber } from "@polkadot/types-codec/types";
import BN from "bn.js";

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

export async function txOracleSubmitPriceSuccessTestHandler(
  api: ApiPromise,
  controllerWallet: KeyringPair,
  signerWallet1: KeyringPair,
  signerWallet2: KeyringPair,
  signerWallet3: KeyringPair,
  signerWallet4: KeyringPair,
  asset: AnyNumber | BN | number | u128,
  price: BN | number | u128,
  slashablePrice: boolean | Bool | bool = false,
  checkPriceChangedEvent = false
) {
  const priceInput: u128 = api.createType("u128", price);
  const assetId: u128 = api.createType("u128", asset);
  let slashedStakeBalanceBefore;
  if (slashablePrice)
    slashedStakeBalanceBefore = <Option<u128>>await api.query.oracle.oracleStake(signerWallet4.publicKey);

  const [result1, result2, result3, result4, result5] = await Promise.all([
    txOracleSubmitPriceSuccessTest(api, controllerWallet, priceInput, assetId),
    txOracleSubmitPriceSuccessTest(api, signerWallet1, priceInput, assetId),
    txOracleSubmitPriceSuccessTest(api, signerWallet2, priceInput, assetId),
    txOracleSubmitPriceSuccessTest(api, signerWallet3, priceInput, assetId),
    txOracleSubmitPriceSuccessTest(
      api,
      signerWallet4,
      // If we want someone to get slashed, we'll just add a huge amount to the correct price here.
      slashablePrice ? <u128>priceInput.add(new BN("999999999")) : priceInput,
      assetId
    )
  ]);
  expect(result1.data[0].toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
  expect(result2.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet1.publicKey).toString());
  expect(result3.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet2.publicKey).toString());
  expect(result4.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet3.publicKey).toString());
  expect(result5.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet4.publicKey).toString());

  expect(priceInput)
    .to.be.bignumber.equal(result1.data[2])
    .to.be.bignumber.equal(result2.data[2])
    .to.be.bignumber.equal(result3.data[2])
    .to.be.bignumber.equal(result4.data[2]);
  expect(result5.data[2]).to.be.bignumber.equal(slashablePrice ? priceInput.add(new BN("999999999")) : priceInput);

  if (checkPriceChangedEvent) {
    const eventResult = await priceEventVerification(api);
    expect(eventResult).to.not.be.an("Error");
  } else await waitForBlocks(api);

  if (slashablePrice) {
    const slashedStakeBalanceAfter = <Option<u128>>await api.query.oracle.oracleStake(signerWallet4.publicKey);
    if (!slashedStakeBalanceBefore) throw new Error("Stake amount was unexpectedly `undefined`!");
    expect(slashedStakeBalanceBefore.unwrap()).to.be.bignumber.greaterThan(slashedStakeBalanceAfter.unwrap());
  }

  const oraclePriceAfter = await api.query.oracle.prices(assetId);
  expect(oraclePriceAfter.price.toString()).to.be.equal(price.toString());
}

async function priceEventVerification(api: ApiPromise) {
  const currentBlockNum = await api.query.system.number();
  let found = false;
  let eventResult: FrameSystemEventRecord | undefined;
  do {
    await waitForBlocks(api);
    const events = await api.query.system.events();
    eventResult = events.find(function (event: FrameSystemEventRecord) {
      if (event.event.method === "PriceChanged") {
        found = true;
        return event;
      }
    });
  } while (!found && (await api.query.system.number()) < currentBlockNum.addn(10)); // ToDo: Update!
  if (!eventResult) throw new AssertionError("No `PriceChanged` event in time!");
  return eventResult;
}
