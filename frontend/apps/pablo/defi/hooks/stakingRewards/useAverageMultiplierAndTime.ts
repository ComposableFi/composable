import { fetchSubsquid } from "@/defi/subsquid/stakingRewards/helpers";
import { PABLO_STAKING_OVERVIEW_QUERY } from "@/defi/subsquid/stakingRewards/queries";
import { fromPerbill } from "@/defi/utils";
import { useEffect, useState } from "react";

export function useAverageLockTimeAndMultiplier(): {
    averageLockMultiplier: number;
    averageLockTime: number
} {
  const [averageLockMultiplier, setAverageLockMultiplier] = useState(0);
  const [averageLockTime, setAverageLockTime] = useState(0);

  useEffect(() => {
    fetchSubsquid<{ pabloOverviewStats: {
        averageLockMultiplier: number;
        averageLockTime: number;
    }}>(PABLO_STAKING_OVERVIEW_QUERY).then((overviewStats) => {
        setAverageLockMultiplier(fromPerbill(overviewStats.pabloOverviewStats.averageLockMultiplier).toNumber());
        setAverageLockTime(overviewStats.pabloOverviewStats.averageLockTime);
    })
  }, []);

  return {
    averageLockMultiplier,
    averageLockTime
  };
}
