import { AssetMetadata, getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { useState, useEffect, useMemo } from "react";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import { useLiquidityByPool } from "./useLiquidityByPool";
import { DailyRewards } from "../poolStats/poolStats.types";
import { calculatePoolStats, fetchPoolStats } from "@/defi/utils/pablo";
import BigNumber from "bignumber.js";
import useStore from "@/store/useStore";

export const useLiquidityPoolDetails = (poolId: number) => {
  const { poolStats, poolStatsValue, userLpBalances, putPoolStats } = useStore();

  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const [pool, setPool] =
    useState<StableSwapPool | ConstantProductPool | undefined>(undefined);

  const tokensLocked = useLiquidityByPool(pool);

  const [baseAsset, setBaseAsset] =
    useState<AssetMetadata | undefined>(undefined);
  const [quoteAsset, setQuoteAsset] =
    useState<AssetMetadata | undefined>(undefined);

  useEffect(() => {
    let pool: StableSwapPool | ConstantProductPool | undefined =
      allLpRewardingPools.find((p) => p.poolId === poolId);

    if (pool) {
      setPool(pool);
      const base = getAssetByOnChainId("picasso", pool.pair.base);
      const quote = getAssetByOnChainId("picasso", pool.pair.quote);

      if (base) {
        setBaseAsset(base);
      }
      if (quote) {
        setQuoteAsset(quote);
      }
    } else {
      setPool(undefined);
      setBaseAsset(undefined);
      setQuoteAsset(undefined);
    }
  }, [poolId, allLpRewardingPools]);

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
    baseAsset,
    quoteAsset,
    pool,
    lpBalance,
    tokensLocked,
    poolStats: _poolStats,
  };
};
