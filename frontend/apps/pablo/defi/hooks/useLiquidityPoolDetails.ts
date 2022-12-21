import BigNumber from "bignumber.js";
import { useEffect, useMemo, useState } from "react";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import { calculatePoolStats, fetchPoolStats } from "@/defi/utils/pablo";
import { Asset, DualAssetConstantProduct } from "shared";
import { DailyRewards } from "@/store/poolStats/types";
import { useStakingRewardPool } from "@/store/stakingRewards/stakingRewards.slice";
import useStore from "@/store/useStore";

export const useLiquidityPoolDetails = (poolId: number) => {
  const { poolStats, poolStatsValue, putPoolStats, substrateTokens } = useStore();
  const { tokens, hasFetchedTokens } = substrateTokens;

  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const [pool, setPool] =
    useState<DualAssetConstantProduct | undefined>(undefined);

  const stakingRewardPool = useStakingRewardPool(pool ? pool.getLiquidityProviderToken().getPicassoAssetId() as string : "-");
  const [baseAsset, setBaseAsset] =
    useState<Asset | undefined>(undefined);
  const [quoteAsset, setQuoteAsset] =
    useState<Asset | undefined>(undefined);

  useEffect(() => {
    let matchingPool: DualAssetConstantProduct | undefined =
      allLpRewardingPools.find((p) => {
        return (p.getPoolId(true) as BigNumber).eq(new BigNumber(poolId));
      });

    if (matchingPool && hasFetchedTokens) {
      const assets = Object.values(tokens);
      const underlyingAssets = matchingPool.getLiquidityProviderToken().getUnderlyingAssets();

      if (underlyingAssets.length > 0) {
        let base = underlyingAssets[0];
        let quote = underlyingAssets[1];
        const baseAsset = assets.find(asset => ((base.getPicassoAssetId(true) as BigNumber).eq(asset.getPicassoAssetId(
          true))));
        const quoteAsset = assets.find(asset => ((quote.getPicassoAssetId(true) as BigNumber).eq(asset.getPicassoAssetId(
          true))));
        setPool(matchingPool);
        setBaseAsset(baseAsset);
        setQuoteAsset(quoteAsset);
      }
    } else {
      setPool(undefined);
      setBaseAsset(undefined);
      setQuoteAsset(undefined);
    }
  }, [poolId, allLpRewardingPools, tokens, hasFetchedTokens]);

  useEffect(() => {
    if (pool) {
      fetchPoolStats(pool).then((poolStates) => {
        const poolStats = calculatePoolStats(poolStates);
        if (poolStats) {
          putPoolStats((pool.getPoolId(true) as BigNumber).toNumber(), poolStats);
        }
      });
    }
  }, [pool, putPoolStats]);

  const _poolStats = useMemo(() => {
    let _poolValue = {
      _24HrFeeValue: "0",
      _24HrVolumeValue: "0",
      totalVolumeValue: "0"
    };

    let _poolStats = {
      _24HrTransactionCount: 0,
      dailyRewards: [] as DailyRewards[],
      apr: "0"
    };

    if (poolStatsValue[poolId]) {
      _poolValue = poolStatsValue[poolId];
    }

    if (poolStats[poolId]) {
      _poolStats = poolStats[poolId];
    }

    return {
      ..._poolValue,
      ..._poolStats
    };
  }, [poolStats, poolStatsValue, poolId]);

  return {
    stakingRewardPool,
    baseAsset,
    quoteAsset,
    pool,
    poolStats: _poolStats
  };
};
