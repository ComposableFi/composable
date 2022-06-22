import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/store/pools/pools.types";
import { useMemo } from "react";
import useStore from "@/store/useStore";

type LBPWithAssets = LiquidityBootstrappingPool & { baseAsset: MockedAsset | undefined; quoteAsset: MockedAsset | undefined }

export const useLiquidityBootstrappingPools =
  (): {
    liquidityBootstrappingPools: LBPWithAssets[];
    setActiveAuctionsPool: (lbPool: LiquidityBootstrappingPool) => void;
  } => {
    const {
      pools: {
        liquidityBootstrappingPools: { verified, spotPrices },
      },
      supportedAssets,
      setActiveAuctionsPool
    } = useStore();

    let lbPools = useMemo(() => {
      const pools: LBPWithAssets[] = [];

      verified.forEach((pool) => {
        let p = { ...pool, baseAsset: undefined as MockedAsset | undefined, quoteAsset: undefined as MockedAsset | undefined };
        let baseAsset = supportedAssets.find(a => a.network[DEFAULT_NETWORK_ID] === p.pair.base.toString());
        let quoteAsset = supportedAssets.find(a => a.network[DEFAULT_NETWORK_ID] === p.pair.base.toString());

        let price = spotPrices.find((p) => p[0] === pool.poolId);
        p.spotPrice = price ? price[1] : "0";
        p.baseAsset = baseAsset;
        p.quoteAsset = quoteAsset;

        pools.push(p);
      });

      return pools;
    }, [verified, spotPrices, supportedAssets]);

    return { liquidityBootstrappingPools: lbPools, setActiveAuctionsPool };
  };