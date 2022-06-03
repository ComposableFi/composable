import { useMemo } from "react";
import useStore from "../useStore";
import { LiquidityBootstrappingPool, LiquidityPoolType } from "./pools.types";

export const useAuctionSpotPrice = (auctionId: number): string => {
  const {
    pools: {
      liquidityBootstrappingPools: { spotPrices },
    },
  } = useStore();

  const spotPrice = useMemo(() => {
    let e = spotPrices.find((s) => s[0] === auctionId);
    return e ? e[1] : "0";
  }, [spotPrices]);

  return spotPrice;
};

export const useVerifiedLiquidityBootstrappingPools =
  (): LiquidityBootstrappingPool[] => {
    const {
      pools: {
        liquidityBootstrappingPools: { verified, spotPrices },
      },
    } = useStore();

    let lbPools = useMemo(() => {
      const pools: LiquidityBootstrappingPool[] = [];

      verified.forEach((pool) => {
        let p = { ... pool };
        let price = spotPrices.find((p) => p[0] === pool.poolId);
        p.spotPrice = price ? price[1] : "0";

        pools.push(p)
      });

      return pools;
    }, [verified.length, spotPrices.length]);

    return lbPools;
  };
