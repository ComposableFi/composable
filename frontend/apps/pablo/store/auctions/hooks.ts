import { useMemo } from "react";
import { LiquidityBootstrappingPool } from "../pools/pools.types";
import useStore from "../useStore";

export const useVerifiedLiquidityBootstrappingPools =
  (): {
    liquidityBootstrappingPools: LiquidityBootstrappingPool[];
    setActiveAuctionsPool: (lbPool: LiquidityBootstrappingPool) => void;
  } => {
    const {
      pools: {
        liquidityBootstrappingPools: { verified, spotPrices },
      },
      setActiveAuctionsPool
    } = useStore();

    let lbPools = useMemo(() => {
      const pools: LiquidityBootstrappingPool[] = [];

      verified.forEach((pool) => {
        let p = { ...pool };
        let price = spotPrices.find((p) => p[0] === pool.poolId);
        p.spotPrice = price ? price[1] : "0";

        pools.push(p);
      });

      return pools;
    }, [verified.length, spotPrices.length]);

    return { liquidityBootstrappingPools: lbPools, setActiveAuctionsPool };
  };


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