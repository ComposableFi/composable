import { ApiPromise } from "@polkadot/api";

export async function fetchBonds(parachainApi: ApiPromise) {
  try {
    const bondOfferCount =
      await parachainApi.query.bondedFinance.bondOfferCount();
    const _bondOfferCount = Number(bondOfferCount.toString());

    let offerPromises = [];

    for (let i = 1; i <= _bondOfferCount; i++) {
      offerPromises.push(parachainApi.query.bondedFinance.bondOffers(i));
    }

    return await Promise.all(offerPromises);
  } catch (ex) {
    console.error(ex);
    return [];
  }
}
