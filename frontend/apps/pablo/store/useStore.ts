import create from "zustand";
import { AllSlices } from "./types";
import { immer } from "zustand/middleware/immer";
import { devtools, subscribeWithSelector } from "zustand/middleware";
import createLiquiditySlice from "./liquidity/slice";
import createRemoveLiquiditySlice from "./removeLiquidity/slice";
import createSwapsSlice from "./swaps/slice";
import createTokensSlice from "./tokens/slice";
import createTokenBalancesSlice from "./tokenBalances/slice";
import { createBYOGSlice } from "@/store/byog/slice";
import createPoolStatsSlice from "@/store/poolStats/slice";
import createPoolSlice from "@/store/createPool/slice";

const useStore = create<AllSlices>()(
  subscribeWithSelector(
    immer(
      devtools((...a) => ({
        ...createTokensSlice(...a),
        ...createTokenBalancesSlice(...a),
        ...createSwapsSlice(...a),
        ...createLiquiditySlice(...a),
        ...createRemoveLiquiditySlice(...a),
        ...createPoolSlice(...a),
        ...createPoolStatsSlice(...a),
        ...createBYOGSlice(...a),
      }))
    )
  )
);

export default useStore;
