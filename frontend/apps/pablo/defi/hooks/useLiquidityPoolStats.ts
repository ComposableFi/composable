import { useMemo } from "react";
import { DualAssetConstantProduct } from "shared";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";
import { PoolStats } from "@/store/poolStats/types";

export const useLiquidityPoolStats = (pool: DualAssetConstantProduct): PoolStats | undefined => {
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