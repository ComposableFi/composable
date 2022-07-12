import create from "zustand";
import createAssetsSlice from "./assets/assets.slice";
import createAuctionsSlice from "./auctions/auctions.slice";
import createSwapsSlice from "./swaps/swaps.slice";
import createPoolsSlice from "./pools/pools.slice";
import createUiSlice from "./ui/ui.slice";
import createBondsSlice from "./bonds/bonds.slice";

import createLiquiditySlice from "./liquidity/liquidity.slice";
import createRemoveLiquiditySlice from "./removeLiquidity/removeLiquidity.slice";
import createPoolSlice from "./createPool/createPool.slice";
import createPoolStatsSlice from "./poolStats/poolStats.slice";

const useStore = create(
  // persist(
  (set, _get) => ({
    ...createUiSlice(set),
    ...createAssetsSlice(set),
    ...createAuctionsSlice(set),
    ...createSwapsSlice(set),
    ...createPoolsSlice(set),
    ...createLiquiditySlice(set),
    ...createRemoveLiquiditySlice(set),
    ...createPoolSlice(set),
    ...createPoolStatsSlice(set),
    ...createBondsSlice(set),
  })
  // {
  //   name: "ui-pablo",
  //   partialize: (state) => ({ assets: state.assets, auctions: state.auctions }),
  // }
  // )
);

export default useStore;
