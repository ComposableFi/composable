import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { bool, Bool, Option, u128 } from "@polkadot/types-codec";
import { AssertionError, expect } from "chai";
import { FrameSystemEventRecord } from "@polkadot/types/lookup";
import { AnyNumber } from "@polkadot/types-codec/types";
import BN from "bn.js";
import { IEvent } from "@polkadot/types/types";
import { AccountId32 } from "@polkadot/types/interfaces";

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
  signers: KeyringPair[],
  asset: AnyNumber | BN | number | u128,
  price: BN | number | u128,
  slashablePrice: boolean | Bool | bool = false,
  checkPriceChangedEvent = false
) {
  const priceInput: u128 = api.createType("u128", price);
  const assetId: u128 = api.createType("u128", asset);
  let slashedStakeBalanceBefore;
  if (slashablePrice)
    slashedStakeBalanceBefore = <Option<u128>>await api.query.oracle.oracleStake(signers[0].publicKey);

  const transactions: Promise<IEvent<[AccountId32, u128, u128]>>[] = [];
  signers.forEach(function (signer, i) {
    if (i == 1 && slashablePrice) {
      transactions.push(
        txOracleSubmitPriceSuccessTest(api, signer, <u128>priceInput.add(new BN("999999999999")), assetId)
      );
      return;
    }
    transactions.push(txOracleSubmitPriceSuccessTest(api, signer, priceInput, assetId));
  });
  console.debug(signers.toString());

  const results = await Promise.all(transactions);
  console.debug(results.toString());
  signers.forEach(function (signer, i) {
    expect(results[i].data[0].toString()).to.be.equal(api.createType("AccountId32", signer.publicKey).toString());
    expect(results[i].data[2]).to.be.bignumber.equal(
      slashablePrice && i == 1 ? priceInput.add(new BN("999999999999")) : priceInput
    );
  });

  if (checkPriceChangedEvent) {
    const eventResult = await priceEventVerification(api);
    expect(eventResult).to.not.be.an("Error");
  } else await waitForBlocks(api);

  if (slashablePrice) {
    const slashedStakeBalanceAfter = <Option<u128>>await api.query.oracle.oracleStake(signers[0].publicKey);
    if (!slashedStakeBalanceBefore) throw new AssertionError("Stake amount was unexpectedly `undefined`!");
    expect(slashedStakeBalanceBefore.unwrap()).to.be.bignumber.greaterThan(slashedStakeBalanceAfter.unwrap());
  }

  const oraclePriceAfter = await api.query.oracle.prices(assetId);
  expect(oraclePriceAfter.price.toString()).to.be.equal(price.toString());
  return results;
}
//music.youtube.com/watch?v=VWQ-e0qXX0M&feature=share

export async function getRewardEvents(api: ApiPromise) {
  let eventFound = false;
  const resultEvents = [];
  do {
    const currentEvents = await api.query.system.events();
    for (const currEvent of currentEvents) {
      console.error("EVENT LOOP!");
      // Extract the phase, event and the event types
      const { event } = currEvent;
      if (event.method.toString().includes("OracleRewarded")) {
        eventFound = true;
        resultEvents.push(currEvent);
      }

      await waitForBlocks(api);
    }
  } while (!eventFound);
  return resultEvents;
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
