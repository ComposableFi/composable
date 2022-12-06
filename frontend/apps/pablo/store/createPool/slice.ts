import { AmmId } from "@/defi/types";
import { StoreSlice } from "../types";
import { CreatePoolSlice } from "./types";

const createPoolSlice: StoreSlice<CreatePoolSlice> = (set) => ({
  createPool: {
    currentStep: 1,
    baseAsset: "none",
    quoteAsset: "none",
    ammId: "none",
    swapFee: "0",
    liquidity: {
      baseAmount: "0",
      quoteAmount: "0",
    },
    weights: {
      baseWeight: "0",
      quoteWeight: "0",
    },
    similarPool: {
      poolId: -1,
      value: "0",
      volume: "0",
      fee: "0",
    },
    setLiquidity: (
      liquidity: Partial<CreatePoolSlice["createPool"]["liquidity"]>
    ) =>
      set((state) => {
        if (liquidity.baseAmount) {
          state.createPool.liquidity.baseAmount = liquidity.baseAmount;
        }
        if (liquidity.quoteAmount) {
          state.createPool.liquidity.quoteAmount = liquidity.quoteAmount;
        }
        return state;
      }),
    setWeights: (weights: Partial<CreatePoolSlice["createPool"]["weights"]>) =>
      set((state) => {
        if (weights.baseWeight) {
          state.createPool.weights.baseWeight = weights.baseWeight;
        }
        if (weights.quoteWeight) {
          state.createPool.weights.quoteWeight = weights.quoteWeight;
        }
        return state;
      }),
    setSimilarPool: (
      pool: Partial<CreatePoolSlice["createPool"]["similarPool"]>
    ) =>
      set((state) => {
        if (pool.fee) state.createPool.similarPool.fee = pool.fee;
        if (pool.poolId) state.createPool.similarPool.poolId = pool.poolId;
        if (pool.value) state.createPool.similarPool.value = pool.value;
        if (pool.volume) state.createPool.similarPool.volume = pool.volume;
        return state;
      }),
    setSelectable: (
      selectables: Partial<{
        baseAsset: string | "none";
        quoteAsset: string | "none";
        ammId: AmmId | "none";
        swapFee: string;
      }>
    ) =>
      set((state) => {
        if (selectables.baseAsset)
          state.createPool.baseAsset = selectables.baseAsset;
        if (selectables.quoteAsset)
          state.createPool.quoteAsset = selectables.quoteAsset;
        if (selectables.ammId) state.createPool.ammId = selectables.ammId;
        if (selectables.swapFee) state.createPool.swapFee = selectables.swapFee;

        return state;
      }),
    resetSlice: () =>
      set((state) => {
        state.createPool.currentStep = 1;
        state.createPool.baseAsset = "none";
        state.createPool.quoteAsset = "none";
        state.createPool.ammId = "none";
        state.createPool.swapFee = "0";
        state.createPool.liquidity.baseAmount = "0";
        state.createPool.liquidity.quoteAmount = "0";
        state.createPool.weights.baseWeight = "0";
        state.createPool.weights.quoteWeight = "0";
        state.createPool.similarPool.poolId = -1;
        state.createPool.similarPool.value = "0";
        state.createPool.similarPool.volume = "0";
        state.createPool.similarPool.fee = "0";

        return state;
      }),
  },
});

export default createPoolSlice;
