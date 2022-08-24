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
import { randomUUID } from "crypto";
import { BondedFinanceBondOffer } from "../src/model";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
} from "../src/types/events";
import { createAccount, createCtx } from "../src/utils";
import {
  processNewBondEvent,
  processNewOfferEvent,
} from "../src/bondedFinanceProcessor";
import { encodeAccount } from "../src/utils";

const OFFER_ID_1 = "1";
const OFFER_ID_2 = "2";
const NB_OF_BONDS_FIRST = BigInt(100);
const NB_OF_BONDS_SECOND = BigInt(20);
const NB_OF_BONDS_THIRD = BigInt(50);
const WHO = createAccount();
const MOCK_ADDRESS = createAccount();
const MOCK_NEW_OFFER_EXTRINSIC = {
  id: "0000000029-000002-3a0b2",
  name: "bondedFinance.offer",
  method: "offer",
  section: "bondedFinance",
  versionInfo: "132",
  era: { mortalEra: "0xb401" },
  signer: "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL",
  args: [
    {
      name: "offer",
      type: '{"beneficiary":"AccountId32","asset":"u128","bondPrice":"u128","nbOfBonds":"u128","maturity":"ComposableTraitsBondedFinanceBondDuration","reward":"ComposableTraitsBondedFinanceBondOfferReward"}',
      value: {
        asset: 1,
        reward: [Object],
        maturity: [Object],
        bondPrice: "0x0000000000000000016345785d8a0000",
        nbOfBonds: 15,
        beneficiary: MOCK_ADDRESS,
      },
    },
    { name: "keepAlive", type: "bool", value: true },
  ],
  hash: "0x807a508abeda083d9cb6f7751329881f2d45981a82deb261f6aa19e37d3d3e63",
  tip: BigInt(0),
  indexInBlock: 2,
};

function createNewOfferEvent(offerId: string) {
  let eventMock = mock(BondedFinanceNewOfferEvent);
  let evt = {
    offerId: BigInt(OFFER_ID_1),
    beneficiary: MOCK_ADDRESS,
  };
  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);

  let event = instance(eventMock);

  return { event };
}

function createNewBondEvent(offerId: string, nbOfBonds: bigint) {
  let eventMock = mock(BondedFinanceNewBondEvent);
  let evt = {
    offerId: BigInt(offerId),
    who: WHO,
    nbOfBonds,
  };
  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);

  let event = instance(eventMock);

  return { event };
}

/**
 * Check if bond offer has expected values
 * @param bondArg
 * @param eventId - Event id
 * @param offerId - Offer id
 * @param purchased - Amount of purchased bonds
 * @param beneficiary - Bond beneficiary
 */
function assertBondedFinanceBondOffer(
  bondArg: BondedFinanceBondOffer,
  eventId: string,
  offerId: string,
  purchased: bigint,
  beneficiary: string
) {
  expect(bondArg.eventId).to.equal(eventId);
  expect(bondArg.offerId).to.equal(offerId);
  expect(bondArg.totalPurchased).to.equal(purchased);
  expect(bondArg.beneficiary).to.equal(beneficiary);
}

/**
 * @param ctx - Event context
 * @param storeMock - Store mock
 * @param offerId
 */
async function assertNewOfferEvent(
  ctx: EventHandlerContext,
  storeMock: Store,
  offerId: string
) {
  const { event } = createNewOfferEvent(offerId);

  await processNewOfferEvent(ctx, event);

  // Check if the offer has been stored
  const [arg] = capture(storeMock.save).last();
  assertBondedFinanceBondOffer(
    arg as unknown as BondedFinanceBondOffer,
    ctx.event.id,
    offerId,
    BigInt(0),
    encodeAccount(MOCK_ADDRESS)
  );
}

async function assertNewBondEvent(
  ctx: EventHandlerContext,
  storeMock: Store,
  offerId: string,
  purchased: bigint
) {
  // Assert last save
  const [arg] = capture(storeMock.save).last();
  assertBondedFinanceBondOffer(
    arg as unknown as BondedFinanceBondOffer,
    ctx.event.id,
    offerId,
    purchased,
    encodeAccount(MOCK_ADDRESS)
  );
}

describe("Bonded finance events", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext;

  // Actual total bonds purchased for each offer
  let totalPurchased: Record<string, BondedFinanceBondOffer>;

  // Total bonds purchased for each offer, as stored in the database
  let totalPurchasedStored: Record<string, BondedFinanceBondOffer>;

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);

    totalPurchased = {};
    totalPurchasedStored = {};

    // Stub store.get() to return the total purchased bonds in the database
    when(
      storeMock.get<BondedFinanceBondOffer>(BondedFinanceBondOffer, anything())
    ).thenCall((_, { where: { offerId } }) => {
      return Promise.resolve(totalPurchasedStored[offerId]);  
    });

    // Stub store.save() to update the total purchased bonds in the database
    when(storeMock.save<BondedFinanceBondOffer>(anything())).thenCall(
      ({ id, totalPurchased, beneficiary, eventId, offerId }) => {
        totalPurchasedStored[id] = new BondedFinanceBondOffer({
          id,
          eventId,
          offerId,
          totalPurchased,
          beneficiary,
        });
      }
    );
  });

  it("Should create new offer events correctly", async () => {
    ctx.event.extrinsic = MOCK_NEW_OFFER_EXTRINSIC;
    await assertNewOfferEvent(ctx, storeMock, OFFER_ID_1);

    // The store should have saved once in the database
    verify(storeMock.save(anyOfClass(BondedFinanceBondOffer))).times(1);
  });

  it("Should create new bond events correctly", async () => {
    // Total bonds purchased for each offer, as stored in the database
    totalPurchasedStored = {
      [OFFER_ID_1]: new BondedFinanceBondOffer({
        id: randomUUID(),
        eventId: ctx.event.id,
        offerId: OFFER_ID_1,
        totalPurchased: BigInt(0),
        beneficiary: encodeAccount(MOCK_ADDRESS),
      }),
      [OFFER_ID_2]: new BondedFinanceBondOffer({
        id: randomUUID(),
        eventId: ctx.event.id,
        offerId: OFFER_ID_2,
        totalPurchased: BigInt(0),
        beneficiary: encodeAccount(MOCK_ADDRESS),
      }),
    };

    const { event: event1 } = createNewBondEvent(OFFER_ID_1, NB_OF_BONDS_FIRST);

    await processNewBondEvent(ctx, event1);

    await assertNewBondEvent(ctx, storeMock, OFFER_ID_1, NB_OF_BONDS_FIRST);

    // The store should have saved twice in the database
    verify(storeMock.save(anyOfClass(BondedFinanceBondOffer))).times(1);

    const { event: event2 } = createNewBondEvent(
      OFFER_ID_1,
      NB_OF_BONDS_SECOND
    );

    await processNewBondEvent(ctx, event2);

    await assertNewBondEvent(
      ctx,
      storeMock,
      OFFER_ID_1,
      NB_OF_BONDS_FIRST + NB_OF_BONDS_SECOND
    );

    // The store should have saved twice in the database
    verify(storeMock.save(anyOfClass(BondedFinanceBondOffer))).times(2);

    const { event: event3 } = createNewBondEvent(OFFER_ID_2, NB_OF_BONDS_THIRD);

    await processNewBondEvent(ctx, event3);

    await assertNewBondEvent(ctx, storeMock, OFFER_ID_2, NB_OF_BONDS_THIRD);

    // The store should have saved three times in the database
    verify(storeMock.save(anyOfClass(BondedFinanceBondOffer))).times(3);
  });
});
