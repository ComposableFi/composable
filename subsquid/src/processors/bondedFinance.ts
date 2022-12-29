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
  return event.asV10002;
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getNewBondEvent(
  event: BondedFinanceNewBondEvent
): NewBondEvent {
  const { offerId, nbOfBonds } = event.asV10002;
  return { offerId, nbOfBonds };
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getOfferCancelledEvent(
  event: BondedFinanceOfferCancelledEvent
): OfferCancelledEvent {
  return event.asV10002;
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
    blockId: ctx.block.id,
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

  const newOffer = getNewBondOffer(ctx, event);

  await ctx.store.save(newOffer);

  await saveAccountAndEvent(ctx, EventType.BONDED_FINANCE_NEW_OFFER);
}

/**
 * Based on the event, update the BondedFinanceBondOffer.
 * @param ctx
 * @param stored
 * @param event
 */
export function updateBondOffer(
  ctx: EventHandlerContext<Store>,
  stored: BondedFinanceBondOffer,
  event: BondedFinanceNewBondEvent
): void {
  const { nbOfBonds } = getNewBondEvent(event);

  stored.totalPurchased += nbOfBonds;
  stored.blockId = ctx.block.id;
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
  updateBondOffer(ctx, stored, event);

  await ctx.store.save(stored);

  await saveAccountAndEvent(ctx, EventType.BONDED_FINANCE_NEW_BOND);
}

/**
 * Cancel the bond offer.
 *  - Set `cancelled` to true.
 * @param ctx
 * @param stored
 */
export function cancelBondOffer(
  ctx: EventHandlerContext<Store>,
  stored: BondedFinanceBondOffer
): void {
  stored.cancelled = true;
  stored.blockId = ctx.block.id;
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
  cancelBondOffer(ctx, stored);

  // Save bond offer.
  await ctx.store.save(stored);

  await saveAccountAndEvent(ctx, EventType.BONDED_FINANCE_OFFER_CANCELLED);
}
