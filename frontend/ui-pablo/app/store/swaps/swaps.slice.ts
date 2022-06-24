import { StoreSlice } from "../types";
import { SwapsSlice } from "./swaps.types";
import {
  putDexRoute,
  putPoolConstants,
  putUiAssetSelection,
  putPoolVariables,
  invertAssetSelection,
  resetSwaps,
} from "./swaps.utils";

const createSwapsSlice: StoreSlice<SwapsSlice> = (set) => ({
  swaps: {
    dexRouter: {
      dexRoute: [],
    },
    poolVariables: {
      spotPrice: "0",
    },
    poolConstants: {
      poolAccountId: "",
      poolIndex: -1,
      feeConfig: {
        feeRate: "0",
        ownerFeeRate: "0",
        protocolFeeRate: "0"
      },
      lbpConstants: undefined,
      poolType: "none",
      pair: {
        quote: -1,
        base: -1,
      },
    },
    ui: {
      quoteAssetSelected: "none",
      baseAssetSelected: "none",
    },
  },
  setDexRouteSwaps: (dexRoute: number[]) =>
    set((prev: SwapsSlice) => ({
      swaps: putDexRoute(prev.swaps, dexRoute),
    })),
  setUiAssetSelectionSwaps: (
    side: "base" | "quote",
    assetId: string | "none"
  ) =>
    set((prev: SwapsSlice) => ({
      swaps: putUiAssetSelection(prev.swaps, side, assetId),
    })),
  setPoolConstantsSwaps: (poolConstants) =>
    set((prev: SwapsSlice) => ({
      swaps: putPoolConstants(prev.swaps, poolConstants),
    })),
  setPoolVariablesSwaps: (key: {
    spotPrice: string;
  }) =>
    set((prev: SwapsSlice) => ({
      swaps: putPoolVariables(prev.swaps, key),
    })),
  invertAssetSelectionSwaps: () =>
    set((prev: SwapsSlice) => ({
      swaps: invertAssetSelection(prev.swaps),
    })),
  resetSwaps: () => set((prev: SwapsSlice) => ({ swaps: resetSwaps(prev.swaps) }))
});

export default createSwapsSlice;
