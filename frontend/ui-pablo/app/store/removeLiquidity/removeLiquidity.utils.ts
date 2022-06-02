import { AssetId } from "@/defi/polkadot/types";
import produce from "immer";
import { RemoveLiquiditySlice } from "./removeLiquidity.types";

export const putState = (
  removeLiquiditySlice: RemoveLiquiditySlice["removeLiquidity"],
  state: {
    poolId: number;
    baseAsset: AssetId;
    quoteAsset: AssetId;
    pooledAmountBase: string;
    pooledAmountQuote: string;
  }
) => {
  return produce(removeLiquiditySlice, (draft) => {
    draft.baseAsset = state.baseAsset;
    draft.quoteAsset = state.quoteAsset;
    draft.poolId = state.poolId;
    draft.pooledAmountBase = state.pooledAmountBase;
    draft.pooledAmountQuote = state.pooledAmountQuote;
  });
};

export const resetState = (
    removeLiquiditySlice: RemoveLiquiditySlice["removeLiquidity"],
) => {
    return produce(removeLiquiditySlice, (draft) => {
      draft.baseAsset = "none"
      draft.quoteAsset = "none"
      draft.poolId = -1;
      draft.pooledAmountBase = "0";
      draft.pooledAmountQuote = "0";
    });
  };
  
