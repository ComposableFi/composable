import { AssetMetadata, getAssetByOnChainId } from "@/defi/polkadot/Assets";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import BigNumber from "bignumber.js";
import { useState, useEffect, useMemo } from "react";
import useStore from "@/store/useStore";
import { useSelectedAccount, useParachainApi } from "substrate-react";
import { fetchBalanceByAssetId } from "../../updaters/balances/utils";
import { DEFAULT_NETWORK_ID } from "../../updaters/constants";
import { useUserProvidedLiquidityByPool } from "./useUserProvidedLiquidityByPool";
import { useAllLpTokenRewardingPools } from "./useAllLpTokenRewardingPools";
import { useLiquidityByPool } from "./useLiquidityByPool";

export const usePoolDetails = (poolId: number) => {
  const { poolStats, userLpBalances } = useStore();

  const allLpRewardingPools = useAllLpTokenRewardingPools();
  const [pool, setPool] =
    useState<StableSwapPool | ConstantProductPool | undefined>(undefined);

  const tokensLocked = useLiquidityByPool(pool);
  const liquidityProvided = useUserProvidedLiquidityByPool(pool);

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
  }, [poolId]);


  const _poolStats = useMemo(() => {
    if (poolStats[poolId]) {
      return poolStats[poolId];
    } else {
      return {
        totalVolume: "0",
        totalValueLocked: "0",
        apr: "0",
        _24HrFee: "0",
        _24HrVolume: "0",
        _24HrTransactionCount: 0,
        dailyRewards: [],
        _24HrFeeValue: "0",
        _24HrVolumeValue: "0",
        totalVolumeValue: "0"
      };
    }
  }, [poolStats, poolId]);

  const lpBalance = useMemo(() => {
    if (pool) {
      if (userLpBalances[pool.poolId]) {
        return new BigNumber(userLpBalances[pool.poolId])
      }
    }
    return new BigNumber(0)
  }, [pool, userLpBalances])

  return {
    baseAsset,
    quoteAsset,
    pool,
    lpBalance,
    liquidityProvided,
    tokensLocked,
    poolStats: _poolStats,
  };
};
