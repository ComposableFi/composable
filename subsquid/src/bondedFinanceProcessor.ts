import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
} from "./types/events";
import { BondedFinanceBondOffer, PicassoTransactionType } from "./model";
import { trySaveAccount, saveActivity, saveTransaction } from "./dbHelper";
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
  const { offerId, nbOfBonds } = event.asV2401 ?? event.asLatest;
  return { offerId, nbOfBonds };
}

function getNewOfferEvent(event: BondedFinanceNewOfferEvent): NewOfferEvent {
  const { offerId, beneficiary } = event.asV2401 ?? event.asLatest;

  return { offerId, beneficiary };
}

export async function processNewOfferEvent(
  ctx: EventHandlerContext,
  event: BondedFinanceNewOfferEvent
) {
  // const event = new BondedFinanceNewOfferEvent(ctx);
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

  const accountId = await trySaveAccount(ctx);

  if (accountId) {
    const txId = await saveTransaction(
      ctx,
      accountId,
      PicassoTransactionType.BONDED_FINANCE_NEW_OFFER
    );
    await saveActivity(ctx, txId, accountId);
  }
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
  // const event = new BondedFinanceNewBondEvent(ctx);

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

  const accountId = await trySaveAccount(ctx);

  if (accountId) {
    const txId = await saveTransaction(
      ctx,
      accountId,
      PicassoTransactionType.BONDED_FINANCE_NEW_BOND
    );
    await saveActivity(ctx, txId, accountId);
  }
}

// TODO: remove offer from database?
export async function processOfferCancelledEvent(ctx: EventHandlerContext) {
  const accountId = await trySaveAccount(ctx);

  if (accountId) {
    const txId = await saveTransaction(
      ctx,
      accountId,
      PicassoTransactionType.BONDED_FINANCE_OFFER_CANCELLED
    );
    await saveActivity(ctx, txId, accountId);
  }
}
