import produce from "immer";
import { SwapsSlice } from "./swaps.types";

export const putDexRoute = (
  swapState: SwapsSlice["swaps"],
  dexRoute: number[]
) => {
  return produce(swapState, (draft) => {
    draft.dexRouter.dexRoute = dexRoute;
  });
};

export const putUiAssetSelection = (
  swapState: SwapsSlice["swaps"],
  side: "quote" | "base",
  assetId: string | "none"
) => {
  return produce(swapState, (draft) => {
    if (side === "quote") {
      draft.ui.quoteAssetSelected = assetId;
    } else {
      draft.ui.baseAssetSelected = assetId;
    }
  });
};

export const putPoolConstants = (
  swapState: SwapsSlice["swaps"],
  constants: SwapsSlice["swaps"]["poolConstants"]
) => {
  return produce(swapState, (draft) => {
    draft.poolConstants = {
      ... constants
    }
  });
}

export const putPoolVariables = (
  swapState: SwapsSlice["swaps"],
  key: {
    spotPrice: string;
  }
) => {
  return produce(swapState, (draft) => {
    if (key.spotPrice) {
      draft.poolVariables.spotPrice = key.spotPrice;
    }
  });
}

export const invertAssetSelection = (
  swapState: SwapsSlice["swaps"],
) => {
  return produce(swapState, (draft) => {
    if (swapState.ui.baseAssetSelected !== "none" && swapState.ui.quoteAssetSelected !== "none") {
      draft.ui.quoteAssetSelected = swapState.ui.baseAssetSelected;
      draft.ui.baseAssetSelected = swapState.ui.quoteAssetSelected;
    }
  });
}

export const resetSwaps = (
  swapState: SwapsSlice["swaps"],
) => {
  return produce(swapState, (draft) => {
    draft.poolConstants = {
      poolAccountId: "",
      poolIndex: -1,
      feeConfig: {
        feeRate: "0",
        ownerFeeRate: "0",
        protocolFeeRate: "0"
      },
      poolType: "none",
      pair: {
        base: -1,
        quote: -1,
      },
      lbpConstants: undefined
    }
    draft.dexRouter.dexRoute = [];
  });
}