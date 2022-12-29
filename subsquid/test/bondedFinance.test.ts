import { expect } from "chai";
import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { instance, mock, when } from "ts-mockito";
import { BondedFinanceBondOffer } from "../src/model";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
} from "../src/types/events";
import { BOB, createAccount, createCtx } from "../src/utils";
import {
  cancelBondOffer,
  getNewBondOffer,
  updateBondOffer,
} from "../src/processors/bondedFinance";

const WHO = createAccount();
const BOB_ADDRESS = createAccount();

function createNewOfferEvent(offerId: string) {
  let eventMock = mock(BondedFinanceNewOfferEvent);
  let evt = {
    offerId: BigInt(offerId),
    beneficiary: BOB_ADDRESS,
  };
  when(eventMock.asV10002).thenReturn(evt);

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
  when(eventMock.asV10002).thenReturn(evt);

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
 * @param cancelled - True is offer has been cancelled
 * @param blockId - Last updated block id
 */
function assertBondedFinanceBondOffer(
  bondArg: BondedFinanceBondOffer,
  eventId?: string,
  offerId?: string,
  purchased?: bigint,
  beneficiary?: string,
  cancelled?: boolean,
  blockId?: string,
) {
  if (eventId) expect(bondArg.eventId).to.equal(eventId);
  if (offerId) expect(bondArg.offerId).to.equal(offerId);
  if (purchased) expect(bondArg.totalPurchased).to.equal(purchased);
  if (beneficiary) expect(bondArg.beneficiary).to.equal(beneficiary);
  if (cancelled !== undefined) expect(bondArg.cancelled).to.equal(cancelled);
  if (blockId) expect(bondArg.blockId).to.equal(blockId);
}

describe("Bonded finance Tests", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext<Store>;

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);
  });

  it("creates bond offer", async () => {
    const { event } = createNewOfferEvent("1");

    const bondOffer = getNewBondOffer(ctx, event);

    assertBondedFinanceBondOffer(bondOffer, undefined, "1", 0n, BOB, false);
  });

  it("updates bond offer", async () => {
    const { event: event1 } = createNewBondEvent("1", 20n);
    const { event: event2 } = createNewBondEvent("1", 100n);

    const bondOffer = new BondedFinanceBondOffer({
      id: "1",
      eventId: "1",
      offerId: "1",
      totalPurchased: 10n,
      beneficiary: BOB,
      cancelled: false,
    });

    updateBondOffer(ctx, bondOffer, event1);
    assertBondedFinanceBondOffer(bondOffer, "1", "1", 30n, BOB, false);

    updateBondOffer(ctx, bondOffer, event2);
    assertBondedFinanceBondOffer(bondOffer, "1", "1", 130n, BOB, false);
  });

  it("Cancels bond offer", async () => {
    const bondOffer = new BondedFinanceBondOffer({
      id: "1",
      eventId: "1",
      offerId: "1",
      totalPurchased: 10n,
      beneficiary: BOB,
      cancelled: false,
    });

    assertBondedFinanceBondOffer(bondOffer, "1", "1", 10n, BOB, false);

    cancelBondOffer(ctx, bondOffer);

    assertBondedFinanceBondOffer(bondOffer, "1", "1", 10n, BOB, true);
  });
});
