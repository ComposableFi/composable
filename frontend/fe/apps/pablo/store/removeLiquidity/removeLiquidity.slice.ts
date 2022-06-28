import { StoreSlice } from "../types";
import { putState, resetState } from "./removeLiquidity.utils";
import { RemoveLiquiditySlice } from "./removeLiquidity.types";

const createRemoveLiquiditySlice: StoreSlice<RemoveLiquiditySlice> = (set) => ({
    removeLiquidity: {
      poolId: -1,
      setRemoveLiquidity: (state) =>
        set((prev: RemoveLiquiditySlice) => ({
            removeLiquidity: putState(prev.removeLiquidity, state),
        })),
      resetRemoveLiquidity: () =>
        set((prev: RemoveLiquiditySlice) => ({
            removeLiquidity: resetState(prev.removeLiquidity),
        })),
    },
});

export default createRemoveLiquiditySlice;
