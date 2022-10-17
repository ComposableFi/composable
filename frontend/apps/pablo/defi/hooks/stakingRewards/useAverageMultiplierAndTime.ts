import { fetchSubsquid } from "@/defi/subsquid/stakingRewards/helpers";
import { PABLO_STAKING_OVERVIEW_QUERY } from "@/defi/subsquid/stakingRewards/queries";
import { fromChainUnits, fromPerbill } from "@/defi/utils";
import { useEffect, useState } from "react";
import BigNumber from "bignumber.js";

export function useAverageLockTimeAndMultiplier(): {
    averageLockMultiplier: number;
    averageLockTime: number;
    totalValueLocked: BigNumber;
} {
  const [averageLockMultiplier, setAverageLockMultiplier] = useState(0);
  const [averageLockTime, setAverageLockTime] = useState(0);
  const [totalValueLocked, setTotalValueLocked] = useState(new BigNumber(0));

  useEffect(() => {
    fetchSubsquid<{ pabloOverviewStats: {
        averageLockMultiplier: number;
        averageLockTime: number;
        totalValueLocked: string;
    }}>(PABLO_STAKING_OVERVIEW_QUERY).then((overviewStats) => {
        setAverageLockMultiplier(fromPerbill(overviewStats.pabloOverviewStats.averageLockMultiplier).toNumber());
        setAverageLockTime(overviewStats.pabloOverviewStats.averageLockTime);
        setTotalValueLocked(fromChainUnits(overviewStats.pabloOverviewStats.totalValueLocked));
    });
  }, []);

  return {
    averageLockMultiplier,
    averageLockTime,
    totalValueLocked
  };
}
