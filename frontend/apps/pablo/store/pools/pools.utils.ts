import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  StableSwapPool,
} from "@/defi/types";
import produce from "immer";
import { PoolsSlice } from "./pools.types";

export const putPoolsList = (
  liquidityPoolsSlice: PoolsSlice["pools"],
  poolsList:
    | ConstantProductPool[]
    | LiquidityBootstrappingPool[]
    | StableSwapPool[],
  poolType: "StableSwap" | "ConstantProduct" | "LiquidityBootstrapping",
  verified: boolean
) => {
  return produce(liquidityPoolsSlice, (draft) => {
    if (poolType === "ConstantProduct") {
      if (verified)
        draft.constantProductPools.verified =
          poolsList as ConstantProductPool[];
      if (!verified)
        draft.constantProductPools.unVerified =
          poolsList as ConstantProductPool[];
    } else if (poolType === "LiquidityBootstrapping") {
      if (verified)
        draft.liquidityBootstrappingPools.verified =
          poolsList as LiquidityBootstrappingPool[];
      if (!verified)
        draft.liquidityBootstrappingPools.unVerified =
          poolsList as LiquidityBootstrappingPool[];
    } else if (poolType === "StableSwap") {
      if (verified)
        draft.stableSwapPools.verified = poolsList as StableSwapPool[];
      if (!verified)
        draft.stableSwapPools.unVerified = poolsList as StableSwapPool[];
    }
  });
};

export const putLiquidityBootstrappingPoolSpotPrice = (
  liquidityPoolsSlice: PoolsSlice["pools"],
  poolId: number,
  spotPrice: string
) => {
  return produce(liquidityPoolsSlice, (draft) => {
    let exists = draft.liquidityBootstrappingPools.spotPrices.find(
      (serie) => serie[0] === poolId
    );
    if (exists) {
      draft.liquidityBootstrappingPools.spotPrices =
        draft.liquidityBootstrappingPools.spotPrices.map((i) => {
          if (i[0] === poolId) {
            i[1] = spotPrice;
          }
          return i;
        });
    } else {
      draft.liquidityBootstrappingPools.spotPrices.push([poolId, spotPrice]);
    }
  });
};