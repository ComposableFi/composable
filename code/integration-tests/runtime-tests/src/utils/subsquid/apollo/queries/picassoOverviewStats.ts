import gql from "../gql";

export type OverviewStats = {
  overviewStats: {
    accountHoldersCount: number;
    activeUsersCount: number;
    totalValueLocked: {
      assetId: string;
      amount: string;
    }[];
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
