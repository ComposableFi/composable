import { EventHandlerContext } from "@subsquid/substrate-processor";
import { BondedFinanceNewBondEvent } from "./types/events";
import { BondedFinanceTotalPurchased } from "./model";

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
  const stored = await ctx.store.get(BondedFinanceTotalPurchased, {
    where: { id: offerId.toString() },
  });

  if (stored?.id) {
    // If offerId is already stored, add to total amount purchased
    stored.purchased += nbOfBonds;
    await ctx.store.save(stored);
  } else {
    // Otherwise, initialize new total amount purchased
    console.log("oooo");
    await ctx.store.save(
      new BondedFinanceTotalPurchased({
        id: offerId.toString(),
        purchased: nbOfBonds,
      })
    );
  }
}
