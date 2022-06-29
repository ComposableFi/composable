import { EventHandlerContext } from "@subsquid/substrate-processor";
import {
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
} from "./types/events";
import { BondedFinanceBondOffer } from "./model";

interface NewOfferEvent {
  offerId: bigint;
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
    const { offerId } = event.asV2300;

    return { offerId };
  }

  const { offerId } = event.asLatest;
  return { offerId };
}

/**
 * Extracts beneficiary from a bond offer
 * @param ctx
 */
function getBeneficiary(ctx: EventHandlerContext): string {
  const { beneficiary } = ctx.event.extrinsic?.args[0]?.value as Record<
    string,
    unknown
  >;

  return beneficiary as string;
}

export async function processNewOfferEvent(
  ctx: EventHandlerContext,
  event: BondedFinanceNewOfferEvent
) {
  const { offerId } = getNewOfferEvent(event);

  const beneficiary = getBeneficiary(ctx);

  await ctx.store.save(
    new BondedFinanceBondOffer({
      id: offerId.toString(),
      purchased: BigInt(0),
      beneficiary,
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

  if (stored?.id) {
    // If offerId is already stored, add to total amount purchased
    stored.purchased += nbOfBonds;
    await ctx.store.save(stored);
  } else {
    // Otherwise, initialize new total amount purchased
    const beneficiary = getBeneficiary(ctx);
    await ctx.store.save(
      new BondedFinanceBondOffer({
        id: offerId.toString(),
        purchased: nbOfBonds,
        beneficiary,
      })
    );
  }
}
