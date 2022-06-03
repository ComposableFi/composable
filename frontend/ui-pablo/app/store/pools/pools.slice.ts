import { AssetId } from "@/defi/polkadot/types";
import { AmmId } from "@/defi/types";
import { StoreSlice } from "../types";
import { CreatePoolSlice } from "./createPool/createPool.types";
import {
  putLiquidity,
  putSelectable,
  putSimilarPool,
  putWeights,
  resetCreatePool,
} from "./createPool/createPool.utils";
import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  LiquidityPoolsSlice,
  StableSwapPool,
} from "./pools.types";
import {
  putLiquidityBootstrappingPoolSpotPrice,
  putPoolsList,
  putUserLpBalance,
} from "./pools.utils";

const createPoolsSlice: StoreSlice<LiquidityPoolsSlice> = (set) => ({
  pools: {
    constantProductPools: {
      verified: [],
      nonVerified: [],
    },
    liquidityBootstrappingPools: {
      verified: [],
      nonVerified: [],
      spotPrices: [],
    },
    stableSwapPools: {
      verified: [],
      nonVerified: [],
    },
    createPool: {
      currentStep: 1,
      baseAsset: "none",
      quoteAsset: "none",
      ammId: "none",
      swapFee: "0",
      liquidity: {
        baseAmount: "0",
        quoteAmount: "0",
      },
      weights: {
        baseWeight: "0",
        quoteWeight: "0",
      },
      similarPool: {
        poolId: -1,
        value: "0",
        volume: "0",
        fee: "0",
      },
      setLiquidity: (liquidity: Partial<CreatePoolSlice["liquidity"]>) =>
        set((prev: LiquidityPoolsSlice) => ({
          pools: putLiquidity(prev.pools, liquidity),
        })),
      setWeights: (weights: Partial<CreatePoolSlice["weights"]>) =>
        set((prev: LiquidityPoolsSlice) => ({
          pools: putWeights(prev.pools, weights),
        })),
      setSimilarPool: (pool: Partial<CreatePoolSlice["similarPool"]>) =>
        set((prev: LiquidityPoolsSlice) => ({
          pools: putSimilarPool(prev.pools, pool),
        })),
      setSelectable: (
        selectables: Partial<{
          baseAsset: AssetId | "none";
          quoteAsset: AssetId | "none";
          ammId: AmmId | "none";
          swapFee: string;
        }>
      ) =>
        set((prev: LiquidityPoolsSlice) => ({
          pools: putSelectable(prev.pools, selectables),
        })),
      resetSlice: () =>
        set((prev: LiquidityPoolsSlice) => ({
          pools: resetCreatePool(prev.pools),
        })),
    },
    user: {
      lpBalances: {},
      setUserLpBalance: (poolId: number, balance: string) =>
        set((prev: LiquidityPoolsSlice) => ({
          pools: putUserLpBalance(prev.pools, poolId, balance),
        })),
    },
    setPoolsList: (
      pool:
        | ConstantProductPool[]
        | LiquidityBootstrappingPool[]
        | StableSwapPool[],
      poolType: "StableSwap" | "ConstantProduct" | "LiquidityBootstrapping",
      verified: boolean
    ) =>
      set((prev: LiquidityPoolsSlice) => ({
        pools: putPoolsList(prev.pools, pool, poolType, verified),
      })),
    setLiquidityBootstrappingPoolSpotPrice: (
      poolId: number,
      spotPrice: string
    ) =>
      set((prev: LiquidityPoolsSlice) => ({
        pools: putLiquidityBootstrappingPoolSpotPrice(
          prev.pools,
          poolId,
          spotPrice
        ),
      })),
  },
});

export default createPoolsSlice;
