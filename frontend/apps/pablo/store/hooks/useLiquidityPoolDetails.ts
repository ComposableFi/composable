import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import BigNumber from "bignumber.js";
import { useState, useEffect, useMemo } from "react";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import { useLiquidityByPool } from "./useLiquidityByPool";
import { DailyRewards } from "../poolStats/poolStats.types";
import { calculatePoolStats, fetchPoolStats } from "@/defi/utils/pablo";
import { MockedAsset } from "../assets/assets.types";
import { matchAssetByPicassoId } from "@/defi/utils";
import useStore from "../useStore";
import { useStakingRewardPool } from "../stakingRewards/stakingRewards.slice";

export const useLiquidityPoolDetails = (poolId: number) => {
  const { poolStats, poolStatsValue, userLpBalances, putPoolStats, supportedAssets } = useStore();

  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const [pool, setPool] =
    useState<StableSwapPool | ConstantProductPool | undefined>(undefined);

  const stakingRewardPool = useStakingRewardPool(pool ? pool.lpToken : "-");
  const tokensLocked = useLiquidityByPool(pool);

  const [baseAsset, setBaseAsset] =
    useState<MockedAsset | undefined>(undefined);
  const [quoteAsset, setQuoteAsset] =
    useState<MockedAsset | undefined>(undefined);

  useEffect(() => {
    let matchingPool: StableSwapPool | ConstantProductPool | undefined =
      allLpRewardingPools.find((p) => p.poolId === poolId);

    if (matchingPool) {
      let base = matchingPool.pair.base.toString();
      let quote = matchingPool.pair.quote.toString();
      const baseAsset = supportedAssets.find(asset => matchAssetByPicassoId(asset, base))
      const quoteAsset = supportedAssets.find(asset => matchAssetByPicassoId(asset, quote))
      setPool(matchingPool);
      setBaseAsset(baseAsset);
      setQuoteAsset(quoteAsset);
    } else {
      setPool(undefined);
      setBaseAsset(undefined);
      setQuoteAsset(undefined);
    }
  }, [poolId, allLpRewardingPools, supportedAssets]);

  useEffect(() => {
    if (pool) {
      fetchPoolStats(pool).then((poolStates) => {
        const poolStats = calculatePoolStats(poolStates);
        if (poolStats) {
          putPoolStats(pool.poolId, poolStats)
        }
      })
    }
  }, [pool, putPoolStats]);

  const _poolStats = useMemo(() => {
    let _poolValue = {
      _24HrFeeValue: "0",
      _24HrVolumeValue: "0",
      totalVolumeValue: "0",
    };

    let _poolStats = {
      _24HrTransactionCount: 0,
      dailyRewards: [] as DailyRewards[],
      apr: "0",
    };

    if (poolStatsValue[poolId]) {
      _poolValue = poolStatsValue[poolId];
    }

    if (poolStats[poolId]) {
      _poolStats = poolStats[poolId];
    }

    return {
      ..._poolValue,
      ..._poolStats,
    };
  }, [poolStats, poolStatsValue, poolId]);

  const lpBalance = useMemo(() => {
    if (pool) {
      if (userLpBalances[pool.poolId]) {
        return new BigNumber(userLpBalances[pool.poolId]);
      }
    }
    return new BigNumber(0);
  }, [pool, userLpBalances]);

  return {
    stakingRewardPool,
    baseAsset,
    quoteAsset,
    pool,
    lpBalance,
    tokensLocked,
    poolStats: _poolStats,
  };
};
