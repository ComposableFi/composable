import { AssetId } from "@/defi/polkadot/types";
import { AmmId } from "@/defi/types";
import { StoreSlice } from "../types";
import { CreatePoolSlice } from "./createPool.types";
import {
  putLiquidity,
  putSelectable,
  putSimilarPool,
  putWeights,
  resetCreatePool,
} from "./createPool.utils";

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
        setLiquidity: (liquidity: Partial<CreatePoolSlice["createPool"]["liquidity"]>) =>
          set((prev: CreatePoolSlice) => ({
            createPool: putLiquidity(prev.createPool, liquidity),
          })),
        setWeights: (weights: Partial<CreatePoolSlice["createPool"]["weights"]>) =>
          set((prev: CreatePoolSlice) => ({
            createPool: putWeights(prev.createPool, weights),
          })),
        setSimilarPool: (pool: Partial<CreatePoolSlice["createPool"]["similarPool"]>) =>
          set((prev: CreatePoolSlice) => ({
            pools: putSimilarPool(prev.createPool, pool),
          })),
        setSelectable: (
          selectables: Partial<{
            baseAsset: string | "none";
            quoteAsset: string | "none";
            ammId: AmmId | "none";
            swapFee: string;
          }>
        ) =>
          set((prev: CreatePoolSlice) => ({
            createPool: putSelectable(prev.createPool, selectables),
          })),
        resetSlice: () =>
          set((prev: CreatePoolSlice) => ({
            createPool: resetCreatePool(prev.createPool),
          })),
      },
});

export default createPoolSlice;
