import BigNumber from "bignumber.js";
import { StoreSlice } from "../types";
import { SwapsSlice } from "./swaps.types";
import {
  putAssetId, putSelectedPool, putSpotPrice, resetSwapsSlice, flipAssetSelection
} from "./swaps.utils";

const createSwapsSlice: StoreSlice<SwapsSlice> = (set) => ({
  swaps: {
    spotPrice: new BigNumber(0),
    selectedAssets: {
      base: "1",
      quote: "4",
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
    }))
  }
});

export default createSwapsSlice;
