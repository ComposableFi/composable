import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
  BondedFinanceOfferCancelledEvent,
} from "../types/events";
import { BondedFinanceBondOffer, PicassoTransactionType } from "../model";
import { saveAccountAndTransaction } from "../dbHelper";
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
  const { offerId, beneficiary } = event.asV2401 ?? event.asLatest;

  return { offerId, beneficiary };
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getNewBondEvent(
  event: BondedFinanceNewBondEvent
): NewBondEvent {
  const { offerId, nbOfBonds } = event.asV2401 ?? event.asLatest;
  return { offerId, nbOfBonds };
}

/**
 * Extract relevant information about a new bond.
 * @param event
 */
export function getOfferCancelledEvent(
  event: BondedFinanceOfferCancelledEvent
): OfferCancelledEvent {
  const { offerId } = event.asV2401 ?? event.asLatest;
  return { offerId };
}

/**
 * Based on the event, return a new BondedFinanceBondOffer.
 * @param ctx
 * @param event
 */
export function getNewBondOffer(
  ctx: EventHandlerContext,
  event: BondedFinanceNewOfferEvent
): BondedFinanceBondOffer {
  const { offerId, beneficiary } = getNewOfferEvent(event);

  return new BondedFinanceBondOffer({
    id: randomUUID(),
    eventId: ctx.event.id,
    offerId: offerId.toString(),
    totalPurchased: BigInt(0),
    beneficiary: encodeAccount(beneficiary),
  });
}

/**
 * Handle `bondedFinances.NewOffer` event.
 *   - Create BondedFinanceBondOffer.
 *   - Create/update Account who deposits funds.
 *   - Create Transaction.
 *   - Create Activity.
 * @param ctx
 */
export async function processNewOfferEvent(ctx: EventHandlerContext) {
  const event = new BondedFinanceNewOfferEvent(ctx);

  const newOffer = getNewOfferEvent(event);

  await ctx.store.save(newOffer);

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.BONDED_FINANCE_NEW_OFFER
  );
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
export async function processNewBondEvent(ctx: EventHandlerContext) {
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

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.BONDED_FINANCE_NEW_BOND
  );
}

/**
 * Cancel the bond offer.
 *  - Set `cancelled` to true.
 * @param stored
 */
export function cancelBondOffer(stored: BondedFinanceBondOffer): void {
  stored.cancelled = true;
}

export async function processOfferCancelledEvent(ctx: EventHandlerContext) {
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

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.BONDED_FINANCE_OFFER_CANCELLED
  );
}
