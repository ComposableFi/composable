import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  StableSwapPool,
} from "@/defi/types";
import produce from "immer";
import { PoolsSlice } from "./pools.types";

export const setPoolsListVerified = (
  liquidityPoolsSlice: PoolsSlice["pools"],
  poolsList: Array<
    ConstantProductPool | LiquidityBootstrappingPool | StableSwapPool
  >
) => {
  return produce(liquidityPoolsSlice, (draft) => {
    draft.constantProductPools.verified = [];
    draft.stableSwapPools.verified = [];
    draft.liquidityBootstrappingPools.verified = [];

    poolsList.forEach((pool) => {
      if ((pool as LiquidityBootstrappingPool).sale) {
        draft.liquidityBootstrappingPools.verified.push(pool as LiquidityBootstrappingPool);
      } else if ((pool as ConstantProductPool).baseWeight) {
        draft.constantProductPools.verified.push(pool as ConstantProductPool);
      } else if ((pool as StableSwapPool).amplificationCoefficient) {
        draft.stableSwapPools.verified.push(pool as StableSwapPool);
      }
    });
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
