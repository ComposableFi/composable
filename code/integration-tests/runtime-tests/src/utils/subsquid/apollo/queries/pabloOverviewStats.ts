import gql from "../gql";

export type PabloOverviewStats = {
  pabloOverviewStats: {
    totalValueLocked: { assetId: string; amount: bigint }[];
    averageLockMultiplier: number;
    averageLockTime: number;
  };
};

export const PABLO_OVERVIEW_STATS = gql`
  query pabloOverviewStats {
    pabloOverviewStats {
      totalValueLocked {
        amount
        assetId
      }
      averageLockMultiplier
      averageLockTime
    }
  }
`;
