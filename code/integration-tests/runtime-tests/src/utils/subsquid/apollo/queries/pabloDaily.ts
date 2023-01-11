import gql from "../gql";

export type PabloDaily = {
  pabloDaily: {
    volume: bigint;
    transactions: bigint;
    fees: bigint;
    poolId?: string;
    assetId: string;
  };
};

export const PABLO_DAILY = gql`
  query pabloDaily($poolId: String) {
    pabloDaily(params: { poolId: $poolId }) {
      volume
      transactions
      fees
      poolId
      assetId
    }
  }
`;
