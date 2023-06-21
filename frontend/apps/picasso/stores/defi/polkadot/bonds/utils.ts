import { ActiveBond } from "@/stores/defi/polkadot/bonds/slice";

/*
 * This is a function check whether bond has offerId
 *
 * @returns bool
 */
export function findCurrentBond(b: ActiveBond, offerId: string): boolean {
  return b.bond.bondOfferId.toString() === offerId;
}
