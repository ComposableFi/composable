import { useEffect, useMemo, useState } from "react";
import type { GetStatsReturn } from "../../utils/pablo/pools/stats";
import { getPoolVolume, getStats } from "../../utils/pablo/pools/stats";
import useStore from "@/store/useStore";
import { PoolConfig } from "@/store/pools/types";
import BigNumber from "bignumber.js";
import { getOraclePrice } from "@/store/oracle/slice";

export const usePoolRatio = (pool: PoolConfig) => {
  const userOwnedLiquidity = useStore((store) => store.ownedLiquidity.tokens);
  const balance = userOwnedLiquidity[pool.config.lpToken]?.balance ?? {
    free: new BigNumber(0),
    locked: new BigNumber(0),
  };
  const poolAmount = useStore((store) => store.pools.poolAmount);
  const isTokensLoaded = useStore(
    (store) => store.substrateTokens.hasFetchedTokens
  );
  const poolTVL = useMemo(() => {
    if (!poolAmount || !isTokensLoaded) {
      return new BigNumber(0);
    }
    const [assetOne, assetTwo] = pool.config.assets;
    const priceOne = getOraclePrice(assetOne.getSymbol(), "coingecko", "usd");
    const priceTwo = getOraclePrice(assetTwo.getSymbol(), "coingecko", "usd");
    const amountOne =
      poolAmount[pool.poolId.toString()]?.[
        assetOne.getPicassoAssetId()?.toString() ?? ""
      ];
    const amountTwo =
      poolAmount[pool.poolId.toString()]?.[
        assetTwo.getPicassoAssetId()?.toString() ?? ""
      ];

    if (amountOne?.length === 0 || amountTwo?.length === 0) {
      return new BigNumber(0);
    }
    if (priceOne.isZero()) {
      return new BigNumber(amountTwo).multipliedBy(priceTwo.multipliedBy(2));
    }

    return new BigNumber(amountOne).multipliedBy(priceOne.multipliedBy(2));
  }, [pool.config.assets, pool.poolId, poolAmount, isTokensLoaded]);

  const [stats, setStats] = useState<GetStatsReturn>(null);
  const totalIssued = useStore((store) => store.pools.totalIssued);
  const poolVolume = getPoolVolume(stats);
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
