import { randomUUID } from "crypto";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
  BondedFinanceOfferCancelledEvent
} from "../types/events";
import { Context, EventItem, Block } from "../processorTypes";
import { BondedFinanceBondOffer, EventType } from "../model";
import { saveAccountAndEvent } from "../dbHelper";
import { encodeAccount } from "../utils";

interface NewOfferEvent {
  offerId: bigint;
  beneficiary: Uint8Array;
}

interface NewBondEvent {
  offerId: bigint;
  nbOfBonds: bigint;
}

interface OfferCancelledEvent {
  offerId: bigint;
}

/**
 * Extract relevant information about a bond offer.
 * @param event
 */
export function getNewOfferEvent(event: BondedFinanceNewOfferEvent): NewOfferEvent {
  if (event.isV1000) {
    const { offerId } = event.asV1000;
    return {
      offerId,
      beneficiary: new Uint8Array()
    };
  }
  return event.asV1400;
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getNewBondEvent(event: BondedFinanceNewBondEvent): NewBondEvent {
  const { offerId, nbOfBonds } = event.asV1000;
  return { offerId, nbOfBonds };
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getOfferCancelledEvent(event: BondedFinanceOfferCancelledEvent): OfferCancelledEvent {
  return event.asV1000;
}

/**
 * Handle `bondedFinances.NewOffer` event.
 *   - Create BondedFinanceBondOffer.
 *   - Create/update Account who deposits funds.
 *   - Create Event.
 *   - Create Activity.
 * @param ctx
 */
export async function processNewOfferEvent(ctx: Context, block: Block, eventItem: EventItem): Promise<void> {
  console.log("Process NewOffer");
  // TODO: check why not triggered
  if (eventItem.name !== "BondedFinance.NewOffer") {
    throw new Error("Invalid event name");
  }
  const event = new BondedFinanceNewOfferEvent(ctx, eventItem.event);

  const { offerId, beneficiary } = getNewOfferEvent(event);

  const newOffer = new BondedFinanceBondOffer({
    id: randomUUID(),
    eventId: eventItem.event.id,
    offerId: offerId.toString(),
    totalPurchased: BigInt(0),
    beneficiary: encodeAccount(beneficiary),
    cancelled: false,
    blockId: block.header.hash
  });

  await ctx.store.save(newOffer);

  await saveAccountAndEvent(ctx, block, eventItem, EventType.BONDED_FINANCE_NEW_OFFER);
}

/**
 * Handle `bondedFinance.NewBond` event.
 * - Update database with new bond information.
 * @param ctx
 */
export async function processNewBondEvent(ctx: Context, block: Block, eventItem: EventItem): Promise<void> {
  console.log("Process NewBond");
  if (eventItem.name !== "BondedFinance.NewBond") {
    throw new Error("Invalid event name");
  }
  const event = new BondedFinanceNewBondEvent(ctx, eventItem.event);
  const { offerId, nbOfBonds } = getNewBondEvent(event);

  // Get stored information (when possible) about the bond offer.
  const stored = await ctx.store.get(BondedFinanceBondOffer, {
    where: { offerId: offerId.toString() }
  });

  if (!stored?.id) {
    return;
  }

  stored.totalPurchased += nbOfBonds;
  stored.blockId = block.header.hash;

  await ctx.store.save(stored);

  await saveAccountAndEvent(ctx, block, eventItem, EventType.BONDED_FINANCE_NEW_BOND);
}

/**
 * Handle `bondedFinance.OfferCancelled` event
 *  - Set bond offer as `cancelled`
 * @param ctx
 */
export async function processOfferCancelledEvent(ctx: Context, block: Block, eventItem: EventItem): Promise<void> {
  console.log("Process OfferCancelled");
  if (eventItem.name !== "BondedFinance.OfferCancelled") {
    throw new Error("Invalid event name");
  }
  const event = new BondedFinanceOfferCancelledEvent(ctx, eventItem.event);
  const { offerId } = getOfferCancelledEvent(event);

  // Get stored information (when possible) about the bond offer.
  const stored = await ctx.store.get(BondedFinanceBondOffer, {
    where: { offerId: offerId.toString() }
  });

  if (!stored?.id) {
    return;
  }

  // Set bond offer as `cancelled`.
  stored.cancelled = true;
  stored.blockId = block.header.hash;

  // Save bond offer.
  await ctx.store.save(stored);

  await saveAccountAndEvent(ctx, block, eventItem, EventType.BONDED_FINANCE_OFFER_CANCELLED);
}
