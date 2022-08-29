import { makeClient } from "@/defi/subsquid/makeClient";

export const queryTotalPurchasedBondsByBondOfferIds = () => makeClient().query(`query queryTotalPurchasedBondsByBondOfferIds {
    bondedFinanceBondOffers {
      id
      totalPurchased
    }
}`).toPromise(); 
