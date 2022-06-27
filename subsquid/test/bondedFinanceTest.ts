import { expect } from "chai";
import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import {
  anyOfClass,
  anything,
  capture,
  instance,
  mock,
  verify,
  when,
} from "ts-mockito";
import { BondedFinanceTotalPurchased } from "../src/model";
import { BondedFinanceNewBondEvent } from "../src/types/events";
import { createAccount, createCtx } from "./common";
import { processNewBondEvent } from "../src/bondedFinanceProcessor";

const OFFER_ID_1 = "1";
const OFFER_ID_2 = "2";
const NB_OF_BONDS_FIRST = BigInt(100);
const NB_OF_BONDS_SECOND = BigInt(20);
const NB_OF_BONDS_THIRD = BigInt(50);
const WHO = createAccount();

function createNewBondEvent(offerId: string, nbOfBonds: bigint) {
  let eventMock = mock(BondedFinanceNewBondEvent);
  let evt = {
    offerId: BigInt(offerId),
    who: WHO,
    nbOfBonds,
  };
  when(eventMock.asV2300).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);

  let event = instance(eventMock);

  return { event };
}

function assertBondedFinanceTotalPurchased(
  bondArg: BondedFinanceTotalPurchased,
  id: string,
  purchased: bigint
) {
  expect(bondArg.id).to.equal(id);
  expect(bondArg.purchased).to.equal(purchased);
}

async function assertNewBondEvent(
  ctx: EventHandlerContext,
  storeMock: Store,
  totalPurchased: Record<string, bigint>,
  totalPurchasedStored: Record<string, bigint>,
  offerId: string,
  purchased: bigint
) {
  const { event } = createNewBondEvent(offerId, purchased);

  // Update the total purchased bonds
  totalPurchased[offerId] += purchased;

  await processNewBondEvent(ctx, event);

  // The database should have the actual total purchased bonds
  expect(totalPurchased[offerId]).to.equal(totalPurchasedStored[offerId]);

  // Assert last save
  const [arg] = capture(storeMock.save).last();
  assertBondedFinanceTotalPurchased(
    arg as unknown as BondedFinanceTotalPurchased,
    offerId,
    totalPurchased[offerId]
  );
}

describe("NewBondEvent", () => {
  it("Should create new bond events correctly", async () => {
    // Actual total bonds purchased for each offer
    let totalPurchased: Record<string, bigint> = {
      [OFFER_ID_1]: BigInt(0),
      [OFFER_ID_2]: BigInt(0),
    };
    // Total bonds purchased for each offer, as stored in the database
    let totalPurchasedStored: Record<string, bigint> = {
      ...totalPurchased,
    };

    let storeMock: Store = mock<Store>();
    let ctx = createCtx(storeMock, 1);

    // Stub store.get() to return the total purchased bonds in the database
    when(
      storeMock.get<BondedFinanceTotalPurchased>(
        BondedFinanceTotalPurchased,
        anything()
      )
    ).thenCall((_, { where: { id } }) => {
      return Promise.resolve(
        new BondedFinanceTotalPurchased({
          id,
          purchased: totalPurchasedStored[id],
        })
      );
    });

    // Stub store.save() to update the total purchased bonds in the database
    when(storeMock.save<BondedFinanceTotalPurchased>(anything())).thenCall(
      ({ id, purchased }) => {
        totalPurchasedStored[id] = purchased;
      }
    );

    await assertNewBondEvent(
      ctx,
      storeMock,
      totalPurchased,
      totalPurchasedStored,
      OFFER_ID_1,
      NB_OF_BONDS_FIRST
    );

    // The store should have saved twice in the database
    verify(storeMock.save(anyOfClass(BondedFinanceTotalPurchased))).times(1);

    await assertNewBondEvent(
      ctx,
      storeMock,
      totalPurchased,
      totalPurchasedStored,
      OFFER_ID_1,
      NB_OF_BONDS_SECOND
    );

    // The store should have saved twice in the database
    verify(storeMock.save(anyOfClass(BondedFinanceTotalPurchased))).times(2);

    await assertNewBondEvent(
      ctx,
      storeMock,
      totalPurchased,
      totalPurchasedStored,
      OFFER_ID_2,
      NB_OF_BONDS_THIRD
    );

    // The store should have saved three times in the database
    verify(storeMock.save(anyOfClass(BondedFinanceTotalPurchased))).times(3);
  });
});
