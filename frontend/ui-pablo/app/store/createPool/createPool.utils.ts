import BigNumber from "bignumber.js";
import produce from "immer";
import { CreatePoolSlice } from "./createPool.types";

export const putLiquidity = (
  createPoolSlice: CreatePoolSlice["createPool"],
  liquidity: Partial<CreatePoolSlice["createPool"]["liquidity"]>
) => {
  return produce(createPoolSlice, (draft) => {
    draft.liquidity.baseAmount =
      liquidity.baseAmount ?? createPoolSlice.liquidity.baseAmount;
    draft.liquidity.quoteAmount =
      liquidity.quoteAmount ?? createPoolSlice.liquidity.quoteAmount;
  });
};

export const putWeights = (
  createPoolSlice: CreatePoolSlice["createPool"],
  weights: Partial<CreatePoolSlice["createPool"]["weights"]>
) => {
  return produce(createPoolSlice, (draft) => {
    let otherWeight = new BigNumber(100);
    if (weights.baseWeight) {
      draft.weights.baseWeight = weights.baseWeight;
      draft.weights.quoteWeight = otherWeight
        .minus(weights.baseWeight)
        .toString();
    } else if (weights.quoteWeight) {
      draft.weights.quoteWeight = weights.quoteWeight;
      draft.weights.baseWeight = otherWeight
        .minus(weights.quoteWeight)
        .toString();
    }
  });
};

export const putSimilarPool = (
  createPoolSlice: CreatePoolSlice["createPool"],
  pool: Partial<CreatePoolSlice["createPool"]["similarPool"]>
) => {
  return produce(createPoolSlice, (draft) => {
    draft.similarPool.poolId =
      pool.poolId ?? createPoolSlice.similarPool.poolId;
    draft.similarPool.fee =
      pool.fee ?? createPoolSlice.similarPool.fee;
    draft.similarPool.volume =
      pool.volume ?? createPoolSlice.similarPool.volume;
    draft.similarPool.value =
      pool.value ?? createPoolSlice.similarPool.value;
  });
};

export const putSelectable = (
  createPoolSlice: CreatePoolSlice["createPool"],
  selectables: Partial<CreatePoolSlice["createPool"]>
) => {
  return produce(createPoolSlice, (draft) => {
    draft.ammId =
      selectables.ammId ?? createPoolSlice.ammId;

    if (draft.ammId === "uniswap") {
      draft.weights.baseWeight = "50"
      draft.weights.quoteWeight = "50"
    }

    draft.baseAsset =
      selectables.baseAsset ?? createPoolSlice.baseAsset;
    draft.quoteAsset =
      selectables.quoteAsset ?? createPoolSlice.quoteAsset;
    draft.swapFee =
      selectables.swapFee ?? createPoolSlice.swapFee;
    draft.currentStep =
      selectables.currentStep ?? createPoolSlice.currentStep;
  });
};

export const resetCreatePool = (
  createPoolSlice: CreatePoolSlice["createPool"]
) => {
  return produce(createPoolSlice, (draft) => {
    draft.currentStep= 1;
    draft.baseAsset= "none";
    draft.quoteAsset= "none";
    draft.ammId= "none";
    draft.swapFee= "0";
    draft.liquidity.baseAmount= "0";
    draft.liquidity.quoteAmount= "0";
    draft.weights.baseWeight= "0";
    draft.weights.  quoteWeight= "0";
    draft.similarPool.poolId= -1;
    draft.similarPool.value ="0";
    draft.similarPool.volume= "0";
    draft.similarPool.fee= "0";
  });
}