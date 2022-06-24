import { PoolFeeConfig } from "@/defi/types";
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
      feeConfig: PoolFeeConfig;
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
    };
    ui: {
      quoteAssetSelected: string | "none";
      baseAssetSelected: string | "none";
    };
  };
  setDexRouteSwaps: (dexRoute: number[]) => void;
  setUiAssetSelectionSwaps: (
    side: "base" | "quote",
    assetId: string | "none"
  ) => void;
  invertAssetSelectionSwaps: () => void;
  setPoolConstantsSwaps: (
    poolConstants: {
      poolAccountId: string;
      poolIndex: number;
      feeConfig: PoolFeeConfig;
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
  setPoolVariablesSwaps: (key: {
    spotPrice: string;
  }) => void;
  resetSwaps: () => void;
}