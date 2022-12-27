import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
  BondedFinanceOfferCancelledEvent,
} from "../types/events";
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
export function getNewOfferEvent(
  event: BondedFinanceNewOfferEvent
): NewOfferEvent {
  if (event.isV1000) {
    const { offerId } = event.asV1000;
    return {
      offerId,
      beneficiary: new Uint8Array(),
    };
  }
  const { offerId, beneficiary } = event.asV1400;
  return {
    offerId,
    beneficiary,
  };
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getNewBondEvent(
  event: BondedFinanceNewBondEvent
): NewBondEvent {
  const { offerId, nbOfBonds } = event.asV1000;
  return { offerId, nbOfBonds };
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getOfferCancelledEvent(
  event: BondedFinanceOfferCancelledEvent
): OfferCancelledEvent {
  return event.asV1000;
}

/**
 * Based on the event, return a new BondedFinanceBondOffer.
 * @param ctx
 * @param event
 */
export function getNewBondOffer(
  ctx: EventHandlerContext<Store>,
  event: BondedFinanceNewOfferEvent
): BondedFinanceBondOffer {
  const { offerId, beneficiary } = getNewOfferEvent(event);

  return new BondedFinanceBondOffer({
    id: randomUUID(),
    eventId: ctx.event.id,
    offerId: offerId.toString(),
    totalPurchased: BigInt(0),
    beneficiary: encodeAccount(beneficiary),
    cancelled: false,
  });
}

/**
 * Handle `bondedFinances.NewOffer` event.
 *   - Create BondedFinanceBondOffer.
 *   - Create/update Account who deposits funds.
 *   - Create Event.
 *   - Create Activity.
 * @param ctx
 */
export async function processNewOfferEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process NewOffer");
  // TODO: check why not triggered
  const event = new BondedFinanceNewOfferEvent(ctx);

  if (event.isV1000) {
    // no-op
    return;
  }

  const newOffer = getNewBondOffer(ctx, event);

  await ctx.store.save(newOffer);

  await saveAccountAndEvent(ctx, EventType.BONDED_FINANCE_NEW_OFFER);
}

/**
 * Based on the event, update the BondedFinanceBondOffer.
 * @param stored
 * @param event
 */
export function updateBondOffer(
  stored: BondedFinanceBondOffer,
  event: BondedFinanceNewBondEvent
): void {
  const { nbOfBonds } = getNewBondEvent(event);

  stored.totalPurchased += nbOfBonds;
}

/**
 * Handle `bondedFinance.NewBond` event.
 * - Update database with new bond information.
 * @param ctx
 */
export async function processNewBondEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process NewBond");
  const event = new BondedFinanceNewBondEvent(ctx);
  const { offerId } = getNewBondEvent(event);

  // Get stored information (when possible) about the bond offer.
  const stored = await ctx.store.get(BondedFinanceBondOffer, {
    where: { offerId: offerId.toString() },
  });

  if (!stored?.id) {
    return;
  }

  // If offerId is already stored, add to total amount purchased.
  updateBondOffer(stored, event);

  await ctx.store.save(stored);

  await saveAccountAndEvent(ctx, EventType.BONDED_FINANCE_NEW_BOND);
}

/**
 * Cancel the bond offer.
 *  - Set `cancelled` to true.
 * @param stored
 */
export function cancelBondOffer(stored: BondedFinanceBondOffer): void {
  stored.cancelled = true;
}

/**
 * Handle `bondedFinance.OfferCancelled` event
 *  - Set bond offer as `cancelled`
 * @param ctx
 */
export async function processOfferCancelledEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process OfferCancelled");
  const event = new BondedFinanceOfferCancelledEvent(ctx);
  const { offerId } = getOfferCancelledEvent(event);

  // Get stored information (when possible) about the bond offer.
  const stored = await ctx.store.get(BondedFinanceBondOffer, {
    where: { offerId: offerId.toString() },
  });

  if (!stored?.id) {
    return;
  }

  // Set bond offer as `cancelled`.
  cancelBondOffer(stored);

  // Save bond offer.
  await ctx.store.save(stored);

  await saveAccountAndEvent(ctx, EventType.BONDED_FINANCE_OFFER_CANCELLED);
}
