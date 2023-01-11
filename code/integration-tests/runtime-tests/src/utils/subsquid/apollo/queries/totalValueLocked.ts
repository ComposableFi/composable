import gql from "../gql";

export type TVL = {
  totalValueLocked: {
    date: string;
    lockedValues: {
      amount: string;
      assetId: string;
    }[];
  }[];
};

export const GET_TOTAL_VALUE_LOCKED = gql`
  query getTotalValueLocked($range: String!, $source: String) {
    totalValueLocked(params: { range: $range, source: $source }) {
      date
      lockedValues {
        amount
        assetId
      }
    }
  }
`;
