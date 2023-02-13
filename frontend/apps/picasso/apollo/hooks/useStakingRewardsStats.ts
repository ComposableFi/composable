import { useQuery } from "@apollo/client";
import {
  GET_STAKING_REWARDS_STATS,
  StakingRewardsStats,
} from "@/apollo/queries/stakingRewardsStats";

export function useStakingRewardsStats() {
  const { data, loading } = useQuery<StakingRewardsStats>(
    GET_STAKING_REWARDS_STATS,
    {
      pollInterval: 30000,
      fetchPolicy: "cache-first",
    }
  );

  return {
    data,
    loading,
  };
}
