import { useQuery } from "@apollo/client";
import { OVERVIEW_STATS, OverviewStats } from "@/apollo/queries/overviewStats";

export const useOverviewStats = () => {
  const { data, error, loading } = useQuery<OverviewStats>(OVERVIEW_STATS, {
    pollInterval: 60000, // Refreshing every 1 minute.
  });

  return { data, error, loading };
};
