import { BondOffer } from "@/defi/types/bonds";
import { ApiPromise } from "@polkadot/api";
import { decodeBondOffer } from "./decode";

export async function fetchBondOffers(parachainApi: ApiPromise): Promise<BondOffer[]> {
  try {
    const bondOfferCount =
      await parachainApi.query.bondedFinance.bondOfferCount();
    const _bondOfferCount = Number(bondOfferCount.toString());

    let offerPromises = [];

    for (let i = 1; i <= _bondOfferCount; i++) {
      offerPromises.push(parachainApi.query.bondedFinance.bondOffers(i));
    }

    const bonds = await Promise.all(offerPromises);
    return bonds.map((offer, index) => decodeBondOffer(offer.toHuman(), index + 1))
  } catch (ex) {
    console.error(ex);
    return [];
  }
}
