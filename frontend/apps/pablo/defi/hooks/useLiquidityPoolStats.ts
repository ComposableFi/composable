import { useMemo } from "react";
import { PabloConstantProductPool } from "@/../../packages/shared";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { PoolStats } from "@/store/poolStats/types";

export const useLiquidityPoolStats = (pool: PabloConstantProductPool): PoolStats | undefined => {
  const { poolStats } = useStore();

  const stats = useMemo(() => {
    const poolId = (pool.getPoolId(true) as BigNumber).toNumber();
    if (poolStats[poolId]) {
        return poolStats[poolId];
    }

    return undefined;
  }, [poolStats, pool]);

  return stats;
};