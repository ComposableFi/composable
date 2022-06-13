import { AssetId } from "@/defi/polkadot/types";
import produce from "immer";
import { SwapsChartRange, SwapsSlice } from "./swaps.types";

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
  assetId: AssetId | "none"
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

export const putUserAccountBalance = (
  swapState: SwapsSlice["swaps"],
  side: "quote" | "base",
  balance: string,
) => {
  return produce(swapState, (draft) => {
    if (side === "quote") {
      draft.userAccount.quoteAssetBalance = balance;
    } else {
      draft.userAccount.baseAssetBalance = balance;
    }
  });
}

export const putPoolVariables = (
  swapState: SwapsSlice["swaps"],
  key: {
    spotPrice: string;
    quoteAssetReserve: string | undefined;
    baseAssetReserve: string | undefined;
  }
) => {
  return produce(swapState, (draft) => {
    if (key.baseAssetReserve) {
      draft.poolVariables.baseAssetReserve = key.baseAssetReserve;
    }
    if (key.quoteAssetReserve) {
      draft.poolVariables.quoteAssetReserve = key.quoteAssetReserve;
    }
    if (key.spotPrice) {
      draft.poolVariables.spotPrice = key.spotPrice;
    }
  });
}