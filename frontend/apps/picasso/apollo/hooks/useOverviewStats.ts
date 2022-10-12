import { useQuery } from "@apollo/client";
import { OVERVIEW_STATS, OverviewStats } from "@/apollo/queries/overviewStats";

export const useOverviewStats = () => {
  const { data, error, loading } = useQuery<OverviewStats>(OVERVIEW_STATS);

  return { data, error, loading };
};
