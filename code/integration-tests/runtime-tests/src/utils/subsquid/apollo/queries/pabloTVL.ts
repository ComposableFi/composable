import gql from "../gql";

export type PabloTVL = {
  pabloTVL: {
    totalValueLocked: bigint;
    date: string;
    assetId: string;
  }[];
};

export const PABLO_TOTAL_VALUE_LOCKED = gql`
  query getTotalValueLocked($range: String!, $poolId: String!) {
    pabloTVL(params: { range: $range, poolId: $poolId }) {
      date
      totalValueLocked
      assetId
    }
  }
`;
