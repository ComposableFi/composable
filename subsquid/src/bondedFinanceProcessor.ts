import { EventHandlerContext } from "@subsquid/substrate-processor";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
} from "./types/events";
import { BondedFinanceBondOffer } from "./model";
import { encodeAccount } from "./utils";

interface NewOfferEvent {
  offerId: bigint;
  beneficiary: Uint8Array;
}

interface NewBondEvent {
  offerId: bigint;
  nbOfBonds: bigint;
}

/**
 * Extracts relevant information about a new bond
 * @param event
 */
function getNewBondEvent(event: BondedFinanceNewBondEvent): NewBondEvent {
  if (event.isV2300) {
    const { offerId, nbOfBonds } = event.asV2300;
    return { offerId, nbOfBonds };
  }

  const { offerId, nbOfBonds } = event.asLatest;
  return { offerId, nbOfBonds };
}

function getNewOfferEvent(event: BondedFinanceNewOfferEvent): NewOfferEvent {
  if (event.isV2300) {
    const { offerId, beneficiary } = event.asV2300;

    return { offerId, beneficiary };
  }

  const { offerId, beneficiary } = event.asLatest;
  return { offerId, beneficiary };
}

export async function processNewOfferEvent(
  ctx: EventHandlerContext,
  event: BondedFinanceNewOfferEvent
) {
  const { offerId, beneficiary } = getNewOfferEvent(event);

  await ctx.store.save(
    new BondedFinanceBondOffer({
      id: offerId.toString(),
      totalPurchased: BigInt(0),
      beneficiary: encodeAccount(beneficiary),
    })
  );
}

/**
 * Updates database with new bond information
 * @param ctx
 * @param event
 */
export async function processNewBondEvent(
  ctx: EventHandlerContext,
  event: BondedFinanceNewBondEvent
) {
  const { offerId, nbOfBonds } = getNewBondEvent(event);

  // Get stored information (when possible) about the bond offer
  const stored = await ctx.store.get(BondedFinanceBondOffer, {
    where: { id: offerId.toString() },
  });

  if (!stored?.id) {
    return;
  }

  // If offerId is already stored, add to total amount purchased
  stored.totalPurchased += nbOfBonds;
  await ctx.store.save(stored);
}
