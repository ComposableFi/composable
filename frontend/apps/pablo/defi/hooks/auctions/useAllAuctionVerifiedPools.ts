import { DEFAULT_NETWORK_ID } from "@/defi/utils";
import { MockedAsset } from "@/store/assets/assets.types";
import { LiquidityBootstrappingPool } from "@/defi/types";
import { useMemo, useState } from "react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useAuctionsSlice } from "@/store/auctions/auctions.slice";

type LiquidityBootstrappingPoolWithPriceAndAsset =
  LiquidityBootstrappingPool & { spotPrice: BigNumber } & {
    baseAsset?: MockedAsset;
    quoteAsset?: MockedAsset;
  };

export const useAllAuctionVerifiedPools = (): {
  tableLimit: number;
  setTableLimit: (limit: number) => void;
  liquidityBootstrappingPools: Array<LiquidityBootstrappingPoolWithPriceAndAsset>;
} => {
  const {
    supportedAssets,
    pools: { liquidityBootstrappingPools },
  } = useStore();
  const { spotPrices } = useAuctionsSlice();
  const [tableLimit, setTableLimit] = useState(5);

  return useMemo(() => {
    const pools: Array<LiquidityBootstrappingPoolWithPriceAndAsset> =
      liquidityBootstrappingPools.verified.map(
        (pool: LiquidityBootstrappingPool) => {
          let baseAsset = supportedAssets.find(
            (asset) =>
              asset.network[DEFAULT_NETWORK_ID] === pool.pair.base.toString()
          );
          let quoteAsset = supportedAssets.find(
            (asset) =>
              asset.network[DEFAULT_NETWORK_ID] === pool.pair.quote.toString()
          );
          const poolId = pool.poolId.toString();
          const spotPrice = spotPrices[poolId] ?? new BigNumber(0);

          return {
            ...pool,
            spotPrice,
            baseAsset,
            quoteAsset,
          };
        }
      );

    return {
      liquidityBootstrappingPools: pools,
      tableLimit,
      setTableLimit,
    };
  }, [
    liquidityBootstrappingPools.verified,
    tableLimit,
    supportedAssets,
    spotPrices,
  ]);
};
