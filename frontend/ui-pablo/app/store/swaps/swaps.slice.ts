import { DEFAULT_SWAP_BASE, DEFAULT_SWAP_QUOTE } from "@/defi/utils";
import BigNumber from "bignumber.js";
import { StoreSlice } from "../types";
import { SwapsSlice } from "./swaps.types";
import {
  putAssetId, putSelectedPool, putSpotPrice, resetSwapsSlice, flipAssetSelection, putTokenAmounts
} from "./swaps.utils";

const createSwapsSlice: StoreSlice<SwapsSlice> = (set) => ({
  swaps: {
    tokenAmounts: {
      assetOneAmount: new BigNumber(0),
      assetTwoAmount: new BigNumber(0)
    },
    spotPrice: new BigNumber(0),
    selectedAssets: {
      base: DEFAULT_SWAP_BASE,
      quote: DEFAULT_SWAP_QUOTE,
    },
    selectedPool: undefined,
    setSelectedAsset: (id, side) => set((prev: SwapsSlice) => ({
      swaps: putAssetId(prev.swaps, id, side)
    })),
    setSelectedPool: (pool) => set((prev: SwapsSlice) => ({
      swaps: putSelectedPool(prev.swaps, pool)
    })),
    setSpotPrice: (price) => set((prev: SwapsSlice) => ({
      swaps: putSpotPrice(prev.swaps, price)
    })),
    resetSwaps: () => set((prev: SwapsSlice) => ({
      swaps: resetSwapsSlice(prev.swaps)
    })),
    flipAssetSelection: () => set((prev: SwapsSlice) => ({
      swaps: flipAssetSelection(prev.swaps)
    })),
    setTokenAmounts: (amounts) => set((prev: SwapsSlice) => ({
      swaps: putTokenAmounts(prev.swaps, amounts)
    })),
  }
});

export default createSwapsSlice;
