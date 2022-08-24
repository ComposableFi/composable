import useStore from "@/store/useStore";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { useMemo } from "react";
import { PoolStats } from "../poolStats/poolStats.types";

export const useLiquidityPoolStats = (pool: StableSwapPool | ConstantProductPool): PoolStats | undefined => {
  const { poolStats } = useStore();

  const stats = useMemo(() => {
    if (poolStats[pool.poolId]) {
        return poolStats[pool.poolId];
    }

    return undefined;
  }, [poolStats, pool]);

  return stats;
};