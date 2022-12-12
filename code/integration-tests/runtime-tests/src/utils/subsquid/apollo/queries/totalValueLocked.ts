import gql from "../gql";

export type TVL = {
  totalValueLocked: number;
  date: string;
  source: string;
};

export type TotalValueLocked = {
  totalValueLocked: TVL[];
};

export const GET_TOTAL_VALUE_LOCKED = gql`
  query getTotalValueLocked($range: String!, $source: String) {
    totalValueLocked(params: { range: $range, source: $source }) {
      date
      source
      totalValueLocked
    }
  }
`;
