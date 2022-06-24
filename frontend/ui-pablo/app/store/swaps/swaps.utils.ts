import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import BigNumber from "bignumber.js";
import produce from "immer";
import { SwapSide, SwapsSlice } from "./swaps.types";

export const putAssetId = (
  swapState: SwapsSlice["swaps"],
  id: string | "none",
  side: SwapSide
) => {
  return produce(swapState, (draft) => {
    if (side === "base") {
      draft.selectedAssets.base = id
    } else {
      draft.selectedAssets.quote = id
    }
  });
};

export const putSelectedPool = (
  swapState: SwapsSlice["swaps"],
  pool: ConstantProductPool | StableSwapPool | undefined
) => {
  return produce(swapState, (draft) => {
    draft.selectedPool = pool
  });
};

export const putSpotPrice = (
  swapState: SwapsSlice["swaps"],
  spotPrice: BigNumber
) => {
  return produce(swapState, (draft) => {
    draft.spotPrice = spotPrice
  });
}

export const resetSwapsSlice = (
  swapState: SwapsSlice["swaps"]
) => {
  return produce(swapState, (draft) => {
    draft.spotPrice = new BigNumber(0);
    draft.selectedAssets = {
      base: "1",
      quote: "4"
    }
    draft.selectedPool = undefined;
  });
}

export const flipAssetSelection = (
  swapState: SwapsSlice["swaps"]
) => {
  return produce(swapState, (draft) => {
    draft.selectedAssets = {
      base: swapState.selectedAssets.quote,
      quote: swapState.selectedAssets.base
    }
    draft.selectedPool = undefined;
  });
}