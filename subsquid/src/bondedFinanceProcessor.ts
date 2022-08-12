import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
} from "./types/events";
import {
  Account,
  BondedFinanceBondOffer,
  PicassoTransactionType,
} from "./model";
import { createTransaction, encodeAccount } from "./utils";
import { getOrCreate } from "./dbHelper";

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

async function saveAccountAndTransaction(
  ctx: EventHandlerContext,
  transactionType: PicassoTransactionType
) {
  const signer = ctx.extrinsic?.signer;

  if (signer) {
    const account = await getOrCreate(ctx.store, Account, signer);

    const tx = createTransaction(ctx, signer, transactionType);

    await ctx.store.save(account);
    await ctx.store.save(tx);
  }
}

export async function processNewOfferEvent(ctx: EventHandlerContext) {
  const event = new BondedFinanceNewOfferEvent(ctx);
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

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.BONDED_FINANCE_NEW_OFFER
  );
}

/**
 * Updates database with new bond information
 * @param ctx
 */
export async function processNewBondEvent(ctx: EventHandlerContext) {
  const event = new BondedFinanceNewBondEvent(ctx);

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

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.BONDED_FINANCE_NEW_BOND
  );
}

// TODO: remove offer from database?
export async function processOfferCancelledEvent(ctx: EventHandlerContext) {
  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.BONDED_FINANCE_OFFER_CANCELLED
  );
}
