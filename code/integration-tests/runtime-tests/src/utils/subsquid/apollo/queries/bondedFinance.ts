import { gql as gqlCore, gql as gqlReact } from "@apollo/client/core";

const isFrontend = false;
const gql = isFrontend ? gqlCore : gqlReact;

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
