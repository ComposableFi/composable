import { gql } from "@apollo/client";

export const GET_BONDED_FINANCE = gql`
  query MyQuery {
    bondedFinanceBondOffers {
      id
      beneficiary
      totalPurchased
      offerId
    }
  }
`;
