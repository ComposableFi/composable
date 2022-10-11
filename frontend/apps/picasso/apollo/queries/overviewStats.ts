import { gql } from "@apollo/client";

export type OverviewStats = {
  overviewStats: {
    accountHoldersCount: number
    activeUsersCount: number
    totalValueLocked: string
    transactionsCount: number
  }
}
export const OVERVIEW_STATS = gql`
    query overviewStats {
        overviewStats {
            accountHoldersCount
            activeUsersCount
            totalValueLocked
            transactionsCount
        }
    }
`;
