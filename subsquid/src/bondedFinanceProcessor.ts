import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
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
  if (event.asV2400) {
    const { offerId, nbOfBonds } = event.asV2400;
    return { offerId, nbOfBonds };
  }

  const { offerId, nbOfBonds } = event.asLatest;
  return { offerId, nbOfBonds };
}

function getNewOfferEvent(event: BondedFinanceNewOfferEvent): NewOfferEvent {
  if (event.isV2400) {
    const { offerId, beneficiary } = event.asV2400;

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
      id: randomUUID(),
      eventId: ctx.event.id,
      offerId: offerId.toString(),
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
    where: { offerId: offerId.toString() },
  });

  if (!stored?.id) {
    return;
  }

  // If offerId is already stored, add to total amount purchased
  stored.totalPurchased += nbOfBonds;
  await ctx.store.save(stored);
}
