import { AssetId } from "@/defi/polkadot/types";
import { StoreSlice } from "../types";
import { SwapsSlice } from "./swaps.types";
import {
  putDexRoute,
  putPoolConstants,
  putUiAssetSelection,
  putUserAccountBalance,
  putPoolVariables,
  invertAssetSelection,
} from "./swaps.utils";

const createSwapsSlice: StoreSlice<SwapsSlice> = (set) => ({
  swaps: {
    dexRouter: {
      dexRoute: [],
    },
    poolVariables: {
      spotPrice: "0",
      quoteAssetReserve: "0",
      baseAssetReserve: "0",
    },
    poolConstants: {
      poolAccountId: "",
      poolIndex: -1,
      fee: "0",
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
    quoteAssetReserve: string | undefined;
    baseAssetReserve: string | undefined;
  }) =>
    set((prev: SwapsSlice) => ({
      swaps: putPoolVariables(prev.swaps, key),
    })),
  invertAssetSelectionSwaps: () =>
    set((prev: SwapsSlice) => ({
      swaps: invertAssetSelection(prev.swaps),
    })),
});

export default createSwapsSlice;
