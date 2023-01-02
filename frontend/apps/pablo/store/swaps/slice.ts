import { StoreSlice } from "../types";
import { SwapsSlice } from "./types";
import BigNumber from "bignumber.js";

const createSwapsSlice: StoreSlice<SwapsSlice> = (set) => ({
  swaps: {
    tokenAmounts: {
      assetOneAmount: new BigNumber(0),
      assetTwoAmount: new BigNumber(0),
    },
    spotPrice: new BigNumber(0),
    selectedAssets: {
      base: "none",
      quote: "none",
    },
    selectedPool: undefined,
    setSelectedAsset: (id, side) =>
      set((state) => {
        if (side === "base") {
          state.swaps.selectedAssets.base = id;
        } else {
          state.swaps.selectedAssets.quote = id;
        }

        return state;
      }),
    setSelectedPool: (pool) =>
      set((state) => {
        state.swaps.selectedPool = pool;
        return state;
      }),
    setSpotPrice: (price) =>
      set((state) => {
        state.swaps.spotPrice = price;
        return state;
      }),
    resetSwaps: () =>
      set((state) => {
        state.swaps.spotPrice = new BigNumber(0);
        state.swaps.selectedAssets = {
          base: "1",
          quote: "4",
        };
        state.swaps.selectedPool = undefined;
        state.swaps.tokenAmounts.assetOneAmount = new BigNumber(0);
        state.swaps.tokenAmounts.assetTwoAmount = new BigNumber(0);
        return state;
      }),
    flipAssetSelection: () =>
      set((state) => {
        state.swaps.selectedAssets = {
          base: state.swaps.selectedAssets.quote,
          quote: state.swaps.selectedAssets.base,
        };
        state.swaps.tokenAmounts.assetOneAmount = new BigNumber(0);
        state.swaps.tokenAmounts.assetTwoAmount = new BigNumber(0);
        return state;
      }),
    setTokenAmounts: (amounts) =>
      set((state) => {
        if (amounts.assetOneAmount) {
          state.swaps.tokenAmounts.assetOneAmount = amounts.assetOneAmount;
        }
        if (amounts.assetTwoAmount) {
          state.swaps.tokenAmounts.assetTwoAmount = amounts.assetTwoAmount;
        }
        return state;
      }),
  },
});

export default createSwapsSlice;
