import { AssetId } from "@/defi/polkadot/types";
import { LiquidityPoolType } from "../pools/pools.types";

export type SwapsChartRange = "24h" | "1w" | "1m";
export type SwapSide = "base" | "quote";
export interface SwapsSlice {
  swaps: {
    dexRouter: {
      dexRoute: number[];
    };
    poolConstants: {
      poolAccountId: string;
      poolIndex: number;
      fee: string;
      poolType: LiquidityPoolType | "none";
      pair: {
        base: number;
        quote: number;
      }
      lbpConstants:
        | {
            start: number;
            end: number;
            initialWeight: string;
            finalWeight: string;
          }
        | undefined;
    },
    poolVariables: {
      spotPrice: string;
      quoteAssetReserve: string;
      baseAssetReserve: string;
    };
    userAccount: {
      baseAssetBalance: string;
      quoteAssetBalance: string;
    };
    ui: {
      quoteAssetSelected: AssetId | "none";
      baseAssetSelected: AssetId | "none";
    };
  };
  swapsChart: {
    _24hourOldPrice: string;
    selectedRange: SwapsChartRange;
    series: [number, number][]
  };
  setDexRouteSwaps: (dexRoute: number[]) => void;
  setUiAssetSelectionSwaps: (
    side: "base" | "quote",
    assetId: AssetId | "none"
  ) => void;
  setPoolConstantsSwaps: (
    poolConstants: {
      poolAccountId: string;
      poolIndex: number;
      fee: string;
      poolType: LiquidityPoolType | "none";
      pair: { base: number; quote: number; }
      lbpConstants:
        | {
            start: number;
            end: number;
            initialWeight: string;
            finalWeight: string;
          }
        | undefined;
    }
  ) => void;
  setUserAccountBalanceSwaps: (
    side: "base" | "quote",
    balance: string
  ) => void;
  setPoolVariablesSwaps: (key: {
    spotPrice: string;
    quoteAssetReserve: string | undefined;
    baseAssetReserve: string | undefined;
  }) => void;
  putSwapsChartSeries: (
    series: [number, number][]
  ) => void;
  putSwapsChartSelectedRange: (
    range: SwapsChartRange
  ) => void;
  put24HourOldPrice: (
    price: string
  ) => void;
}