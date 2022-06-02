import {
  ConstantProductPool,
  LiquidityBootstrappingPool,
  LiquidityPoolsSlice,
  StableSwapPool,
} from "./pools.types";
import produce from "immer";

export const putPoolsList = (
  liquidityPoolsSlice: LiquidityPoolsSlice["pools"],
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
        draft.constantProductPools.nonVerified =
          poolsList as ConstantProductPool[];
    } else if (poolType === "LiquidityBootstrapping") {
      if (verified)
        draft.liquidityBootstrappingPools.verified =
          poolsList as LiquidityBootstrappingPool[];
      if (!verified)
        draft.liquidityBootstrappingPools.nonVerified =
          poolsList as LiquidityBootstrappingPool[];
    } else if (poolType === "StableSwap") {
      if (verified)
        draft.stableSwapPools.verified = poolsList as StableSwapPool[];
      if (!verified)
        draft.stableSwapPools.nonVerified = poolsList as StableSwapPool[];
    }
  });
};

export const putLiquidityBootstrappingPoolSpotPrice = (
  liquidityPoolsSlice: LiquidityPoolsSlice["pools"],
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

export const putUserLpBalance = (
  liquidityPoolsSlice: LiquidityPoolsSlice["pools"],
  poolId: number,
  balance: string
) => {
  return produce(liquidityPoolsSlice, (draft) => {
    draft.user.lpBalances[poolId] = balance;
  });
};