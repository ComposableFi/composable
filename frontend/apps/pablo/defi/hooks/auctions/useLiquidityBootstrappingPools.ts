import { matchAssetByPicassoId } from "@/defi/utils";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { useMemo, useState } from "react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

type LBPWithAssets = LiquidityBootstrappingPool & { spotPrice: BigNumber; } & { baseAsset: MockedAsset | undefined; quoteAsset: MockedAsset | undefined }

export const useLiquidityBootstrappingPools =
  (): {
    auctionsTableLimit: number,
    liquidityBootstrappingPools: LBPWithAssets[];
    setActiveAuctionsPool: (lbPool: LiquidityBootstrappingPool) => void;
  } => {
    const [auctionsTableLimit, setAuctionsTableLimit] = useState(4);

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
        let p: LBPWithAssets = { ...pool, baseAsset: undefined as MockedAsset | undefined, quoteAsset: undefined as MockedAsset | undefined, spotPrice: new BigNumber(0) };
        let baseAsset = supportedAssets.find(a => matchAssetByPicassoId(a, p.pair.base.toString()));
        let quoteAsset = supportedAssets.find(a => matchAssetByPicassoId(a, p.pair.quote.toString()));

        p.spotPrice = spotPrices.reduce((acc, [auctionId, price]) => {
          if (auctionId === pool.poolId) {
            return new BigNumber(price)
          }
          return acc;
        }, new BigNumber(0))
        p.baseAsset = baseAsset;
        p.quoteAsset = quoteAsset;

        pools.push(p);
      });

      return pools;
    }, [verified, spotPrices, supportedAssets]);

    return { liquidityBootstrappingPools: lbPools, setActiveAuctionsPool, auctionsTableLimit };
  };