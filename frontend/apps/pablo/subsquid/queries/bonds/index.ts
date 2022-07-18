import { makeClient } from "../../index";

export const queryTotalPurchasedBondsByBondOfferIds = () => makeClient().query(`query queryTotalPurchasedBondsByBondOfferIds {
    bondedFinanceBondOffers {
      id
      totalPurchased
    }
}`).toPromise();