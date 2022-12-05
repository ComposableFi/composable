import { PabloConstantProductPool } from "shared";
import BigNumber from "bignumber.js";

export type SwapsChartRange = "24h" | "1w" | "1m";
export type SwapSide = "base" | "quote";
export interface SwapsSlice {
  swaps: {
    tokenAmounts: {
      assetOneAmount: BigNumber;
      assetTwoAmount: BigNumber;
    },
    spotPrice: BigNumber;
    selectedAssets: {
      base: string | "none";
      quote: string | "none";
    },
    selectedPool: PabloConstantProductPool | undefined;
    setTokenAmounts: (tokeAmounts: {assetOneAmount: BigNumber; assetTwoAmount: BigNumber}) => void;
    setSelectedPool: (pool: PabloConstantProductPool | undefined) => void;
    flipAssetSelection: () => void;
    setSelectedAsset: (assetId: string | "none", side: SwapSide) => void;
    setSpotPrice: (spotPrice: BigNumber) => void;
    resetSwaps: () => void;
  },
}