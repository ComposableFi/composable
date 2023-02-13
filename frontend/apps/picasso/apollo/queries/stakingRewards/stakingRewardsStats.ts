import { gql } from "@apollo/client";

export interface StakingRewardsStats {
  stakingRewardsStats: {
    averageLockDuration: string;
    totalValueLocked: string;
  };
}

export const GET_STAKING_REWARDS_STATS = gql`
  query stakingRewardsStats {
    stakingRewardsStats(params: { poolId: "1" }) {
      averageLockDuration
      totalValueLocked
    }
  }
`;
