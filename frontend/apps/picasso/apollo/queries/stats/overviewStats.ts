import { gql } from "@apollo/client";
import { AssetAmountPair } from "@/apollo/queries/types";

export type OverviewStats = {
  overviewStats: {
    accountHoldersCount: number;
    activeUsersCount: number;
    totalValueLocked: Array<AssetAmountPair>;
    transactionsCount: number;
  };
};
export const OVERVIEW_STATS = gql`
  query overviewStats {
    overviewStats {
      accountHoldersCount
      activeUsersCount
      totalValueLocked {
        amount
        assetId
      }
      transactionsCount
    }
  }
`;
