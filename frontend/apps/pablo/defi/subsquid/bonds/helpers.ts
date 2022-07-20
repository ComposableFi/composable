import { queryTotalPurchasedBondsByBondOfferIds } from "./queries";
import BigNumber from "bignumber.js";

export async function fetchTotalPurchasedBondsByOfferIds(): Promise<Record<string, BigNumber>> {
  let totalPurchasedMap: Record<string, BigNumber> = {};
  try {
    let { data, error } = await queryTotalPurchasedBondsByBondOfferIds();
    if (!data)
      throw new Error(
        `fetchTotalPurchasedBondsByOfferIds unable to fetch subsquid data`
      );
    if (error) throw new Error(error.message);

    let { bondedFinanceBondOffers } = data;

    totalPurchasedMap = bondedFinanceBondOffers.reduce(
      (
        p: Record<string, BigNumber>,
        c: { id: string; totalPurchased: string }
      ) => {
        return {
          ...p,
          [c.id]: new BigNumber(c.totalPurchased),
        };
      },
      {} as Record<string, BigNumber>
    );
  } catch (err) {
    console.error(err);
  } finally {
    return totalPurchasedMap;
  }
}