import { useCallback, useEffect, useMemo, useState } from "react";
import type { GetStatsReturn } from "../../utils/pablo/pools/stats";
import { getPoolVolume, getStats } from "../../utils/pablo/pools/stats";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { getOraclePrice } from "@/store/oracle/slice";
import { pipe } from "fp-ts/lib/function";
import * as O from "fp-ts/lib/Option";

export const usePoolRatio = (poolId: string) => {
  const userOwnedLiquidity = useStore((store) => store.ownedLiquidity.tokens);
  const pool = useStore(
    useCallback(
      (store) => {
        return pipe(store.pools.getPoolById(poolId), O.toNullable);
      },
      [poolId]
    )
  );
  const balance = useMemo(() => {
    if (!pool) {
      return {
        free: new BigNumber(0),
        locked: new BigNumber(0),
      };
    }
    if (pool && !userOwnedLiquidity[pool.config.lpToken]) {
      return {
        free: new BigNumber(0),
        locked: new BigNumber(0),
      };
    }
    return userOwnedLiquidity[pool.config.lpToken].balance;
  }, [pool, userOwnedLiquidity]);
  const poolAmount = useStore((store) => store.pools.poolAmount);
  const isTokensLoaded = useStore(
    (store) => store.substrateTokens.hasFetchedTokens
  );
  const poolTVL = useMemo(() => {
    if (!poolAmount || !isTokensLoaded || !pool) {
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
  }, [isTokensLoaded, pool, poolAmount]);
  const [stats, setStats] = useState<GetStatsReturn>(null);
  const totalIssued = useStore((store) => store.pools.totalIssued);
  const poolVolume = getPoolVolume(stats);
  const poolRatio = useMemo(() => {
    return pool
      ? balance.free
          .div(totalIssued[pool.poolId.toString()])
          .multipliedBy(100)
          .toNumber() || 0
      : 0;
  }, [balance.free, pool, totalIssued]);
  const setPoolStat = useCallback(() => {
    if (!pool || !stats) return;
    getStats(pool).then((result) => {
      setStats(result);
    });
  }, [stats, pool]);

  useEffect(() => {
    setPoolStat();
  }, [setPoolStat]);

  return {
    lpRatio: poolRatio,
    poolVolume: poolVolume,
    poolTVL: poolTVL,
    userVolume: poolVolume.multipliedBy(new BigNumber(poolRatio)).div(100),
    userTVL: poolTVL.multipliedBy(new BigNumber(poolRatio)).div(100),
  };
};
