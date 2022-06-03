import { AssetId } from "@/defi/polkadot/types";
import { StoreSlice } from "../types";
import { SwapsChartRange, SwapsSlice } from "./swaps.types";
import {
  putDexRoute,
  putPoolConstants,
  putUiAssetSelection,
  putUserAccountBalance,
  putPoolVariables,
  putSwapsChartSeries,
  putSwapsChartSelectedRange,
  put24HourOldPrice,
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
    userAccount: {
      baseAssetBalance: "0",
      quoteAssetBalance: "0",
    },
    ui: {
      quoteAssetSelected: "none",
      baseAssetSelected: "none",
    },
  },
  swapsChart: {
    series: [],
    _24hourOldPrice: "0",
    selectedRange: "24h",
  },
  setDexRouteSwaps: (dexRoute: number[]) =>
    set((prev: SwapsSlice) => ({
      swaps: putDexRoute(prev.swaps, dexRoute),
    })),
  setUiAssetSelectionSwaps: (
    side: "base" | "quote",
    assetId: AssetId | "none"
  ) =>
    set((prev: SwapsSlice) => ({
      swaps: putUiAssetSelection(prev.swaps, side, assetId),
    })),
  setPoolConstantsSwaps: (poolConstants) =>
    set((prev: SwapsSlice) => ({
      swaps: putPoolConstants(prev.swaps, poolConstants),
    })),
  setUserAccountBalanceSwaps: (side: "base" | "quote", balance: string) =>
    set((prev: SwapsSlice) => ({
      swaps: putUserAccountBalance(prev.swaps, side, balance),
    })),
  setPoolVariablesSwaps: (key: {
    spotPrice: string;
    quoteAssetReserve: string | undefined;
    baseAssetReserve: string | undefined;
  }) =>
    set((prev: SwapsSlice) => ({
      swaps: putPoolVariables(prev.swaps, key),
    })),
  putSwapsChartSeries: (series: [number, number][]) =>
    set((prev: SwapsSlice) => ({
      swapsChart: putSwapsChartSeries(prev.swapsChart, series),
    })),
  putSwapsChartSelectedRange: (range: SwapsChartRange) =>
    set((prev: SwapsSlice) => ({
      swapsChart: putSwapsChartSelectedRange(prev.swapsChart, range),
    })),
  put24HourOldPrice: (price: string) =>
    set((prev: SwapsSlice) => ({
      swapsChart: put24HourOldPrice(prev.swapsChart, price),
    })),
});

export default createSwapsSlice;
