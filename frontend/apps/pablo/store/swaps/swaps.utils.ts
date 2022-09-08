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
    draft.tokenAmounts.assetOneAmount = new BigNumber(0);
    draft.tokenAmounts.assetTwoAmount = new BigNumber(0);
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
    draft.tokenAmounts.assetOneAmount = new BigNumber(0);
    draft.tokenAmounts.assetTwoAmount = new BigNumber(0);
  });
}

export const putTokenAmounts = (
  swapState: SwapsSlice["swaps"],
  amounts: {
    assetOneAmount: BigNumber | undefined,
    assetTwoAmount: BigNumber | undefined
  }
) => {
  return produce(swapState, (draft) => {
    if (amounts.assetOneAmount) {
      draft.tokenAmounts.assetOneAmount = amounts.assetOneAmount
    }
    if (amounts.assetTwoAmount) {
      draft.tokenAmounts.assetTwoAmount = amounts.assetTwoAmount
    }
  });
}