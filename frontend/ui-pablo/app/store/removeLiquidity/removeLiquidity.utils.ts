import { AssetId } from "@/defi/polkadot/types";
import produce from "immer";
import { ConstantProductPool, StableSwapPool } from "../pools/pools.types";
import { RemoveLiquiditySlice } from "./removeLiquidity.types";

export const putState = (
  removeLiquiditySlice: RemoveLiquiditySlice["removeLiquidity"],
  state: {
    poolId: number;
  }
) => {
  return produce(removeLiquiditySlice, (draft) => {
    draft.poolId = state.poolId;
  });
};

export const resetState = (
  removeLiquiditySlice: RemoveLiquiditySlice["removeLiquidity"]
) => {
  return produce(removeLiquiditySlice, (draft) => {
    draft.poolId = -1;
  });
};
