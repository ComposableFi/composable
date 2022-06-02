import BigNumber from "bignumber.js";
import produce from "immer";
import { LiquidityPoolsSlice } from "../pools.types";
import { CreatePoolSlice } from "./createPool.types";

export const putLiquidity = (
  createPoolSlice: LiquidityPoolsSlice["pools"],
  liquidity: Partial<CreatePoolSlice["liquidity"]>
) => {
  return produce(createPoolSlice, (draft) => {
    draft.createPool.liquidity.baseAmount =
      liquidity.baseAmount ?? createPoolSlice.createPool.liquidity.baseAmount;
    draft.createPool.liquidity.quoteAmount =
      liquidity.quoteAmount ?? createPoolSlice.createPool.liquidity.quoteAmount;
  });
};

export const putWeights = (
  createPoolSlice: LiquidityPoolsSlice["pools"],
  weights: Partial<CreatePoolSlice["weights"]>
) => {
  return produce(createPoolSlice, (draft) => {
    let otherWeight = new BigNumber(100);
    if (weights.baseWeight) {
      draft.createPool.weights.baseWeight = weights.baseWeight;
      draft.createPool.weights.quoteWeight = otherWeight
        .minus(weights.baseWeight)
        .toString();
    } else if (weights.quoteWeight) {
      draft.createPool.weights.quoteWeight = weights.quoteWeight;
      draft.createPool.weights.baseWeight = otherWeight
        .minus(weights.quoteWeight)
        .toString();
    }
  });
};

export const putSimilarPool = (
  createPoolSlice: LiquidityPoolsSlice["pools"],
  pool: Partial<CreatePoolSlice["similarPool"]>
) => {
  return produce(createPoolSlice, (draft) => {
    draft.createPool.similarPool.poolId =
      pool.poolId ?? createPoolSlice.createPool.similarPool.poolId;
    draft.createPool.similarPool.fee =
      pool.fee ?? createPoolSlice.createPool.similarPool.fee;
    draft.createPool.similarPool.volume =
      pool.volume ?? createPoolSlice.createPool.similarPool.volume;
    draft.createPool.similarPool.value =
      pool.value ?? createPoolSlice.createPool.similarPool.value;
  });
};

export const putSelectable = (
  createPoolSlice: LiquidityPoolsSlice["pools"],
  selectables: Partial<CreatePoolSlice>
) => {
  return produce(createPoolSlice, (draft) => {
    draft.createPool.ammId =
      selectables.ammId ?? createPoolSlice.createPool.ammId;
    draft.createPool.baseAsset =
      selectables.baseAsset ?? createPoolSlice.createPool.baseAsset;
    draft.createPool.quoteAsset =
      selectables.quoteAsset ?? createPoolSlice.createPool.quoteAsset;
    draft.createPool.swapFee =
      selectables.swapFee ?? createPoolSlice.createPool.swapFee;
    draft.createPool.currentStep =
      selectables.currentStep ?? createPoolSlice.createPool.currentStep;
  });
};

export const resetCreatePool = (
  createPoolSlice: LiquidityPoolsSlice["pools"]
) => {
  return produce(createPoolSlice, (draft) => {
    draft.createPool.currentStep= 1;
    draft.createPool.baseAsset= "none";
    draft.createPool.quoteAsset= "none";
    draft.createPool.ammId= "none";
    draft.createPool.swapFee= "0";
    draft.createPool.liquidity.baseAmount= "0";
    draft.createPool.liquidity.quoteAmount= "0";
    draft.createPool.weights.baseWeight= "0";
    draft.createPool.weights.  quoteWeight= "0";
    draft.createPool.similarPool.poolId= -1;
    draft.createPool.similarPool.value ="0";
    draft.createPool.similarPool.volume= "0";
    draft.createPool.similarPool.fee= "0";
  });
}