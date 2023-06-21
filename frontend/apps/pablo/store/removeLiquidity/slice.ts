import { StoreSlice } from "../types";
import { RemoveLiquiditySlice } from "./types";

const createRemoveLiquiditySlice: StoreSlice<RemoveLiquiditySlice> = (set) => ({
  removeLiquidity: {
    poolId: -1,
    setRemoveLiquidity: (stat) =>
      set((state) => {
        state.removeLiquidity.poolId = stat.poolId;
        return state;
      }),
    resetRemoveLiquidity: () => () =>
      set((state) => {
        state.removeLiquidity.poolId = -1;
        return state;
      }),
  },
});

export default createRemoveLiquiditySlice;
