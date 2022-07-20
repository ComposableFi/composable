import { AssetId } from "@/defi/polkadot/types";
import { StoreSlice } from "../types";
import {
  PoolsSlice,
  AnyPoolArray
} from "./pools.types";
import {
  putLiquidityBootstrappingPoolSpotPrice,
  putPoolsList,
} from "./pools.utils";

const createPoolsSlice: StoreSlice<PoolsSlice> = (set) => ({
  pools: {
    constantProductPools: {
      verified: [],
      unVerified: [],
    },
    liquidityBootstrappingPools: {
      verified: [],
      unVerified: [],
      spotPrices: [],
    },
    stableSwapPools: {
      verified: [],
      unVerified: [],
    },
    setPoolsList: (
      pool: AnyPoolArray,
      poolType: "StableSwap" | "ConstantProduct" | "LiquidityBootstrapping",
      verified: boolean
    ) =>
      set((prev: PoolsSlice) => ({
        pools: putPoolsList(prev.pools, pool, poolType, verified),
      })),
    setLiquidityBootstrappingPoolSpotPrice: (
      poolId: number,
      spotPrice: string
    ) =>
      set((prev: PoolsSlice) => ({
        pools: putLiquidityBootstrappingPoolSpotPrice(
          prev.pools,
          poolId,
          spotPrice
        ),
      })),
  },
});

export default createPoolsSlice;
