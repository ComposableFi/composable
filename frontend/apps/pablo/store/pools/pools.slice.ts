import { StoreSlice } from "../types";
import {
  PoolsSlice,
  AnyPoolArray
} from "./pools.types";
import {
  putLiquidityBootstrappingPoolSpotPrice,
  setPoolsListVerified,
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
      pools: AnyPoolArray
    ) =>
      set((prev: PoolsSlice) => ({
        pools: setPoolsListVerified(prev.pools, pools),
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
