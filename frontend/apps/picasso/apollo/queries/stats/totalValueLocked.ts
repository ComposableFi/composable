import { gql } from "@apollo/client";
import { AssetAmountPair } from "@/apollo/queries/types";

export type TVL = {
  lockedValues: Array<AssetAmountPair>;
  date: string;
};

export type TotalValueLocked = {
  totalValueLocked: TVL[];
};

export const GET_TOTAL_VALUE_LOCKED = gql`
  query getTotalValueLocked($range: String!, $source: String!) {
    totalValueLocked(params: { range: $range, source: $source }) {
      date
      lockedValues {
        assetId
        amount
      }
    }
  }
`;
