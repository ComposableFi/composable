import { useEffect, useState } from "react";
import type { GetStatsReturn } from "../../utils/pablo/pools/stats";
import {
  getPoolTVL,
  getPoolVolume,
  getStats,
} from "../../utils/pablo/pools/stats";
import useStore from "@/store/useStore";
import { PoolConfig } from "@/store/pools/types";
import BigNumber from "bignumber.js";

export const usePoolRatio = (pool: PoolConfig) => {
  const userOwnedLiquidity = useStore((store) => store.ownedLiquidity.tokens);
  const balance = userOwnedLiquidity[pool.config.lpToken]?.balance ?? {
    free: new BigNumber(0),
    locked: new BigNumber(0),
  };
  const [stats, setStats] = useState<GetStatsReturn>(null);
  const totalIssued = useStore((store) => store.pools.totalIssued);
  const poolVolume = getPoolVolume(stats);
  const poolTVL = getPoolTVL(stats);
  const poolRatio =
    balance.free
      .div(totalIssued[pool.poolId.toString()])
      .multipliedBy(100)
      .toNumber() || 0;

  useEffect(() => {
    getStats(pool).then((result) => {
      setStats(result);
    });
  }, [pool]);

  return {
    lpRatio: poolRatio,
    poolVolume: poolVolume,
    poolTVL: poolTVL,
    userVolume: poolVolume.multipliedBy(new BigNumber(poolRatio)).div(100),
    userTVL: poolTVL.multipliedBy(new BigNumber(poolRatio)).div(100),
  };
};
