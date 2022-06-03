import create from "zustand";
import createAssetsSlice from "./assets/assets.slice";
import createAddLiquiditySlice from "./pools/addLiquidity/addLiquidity.slice";
import createAuctionsSlice from "./auctions/auctions.slice";
import createSwapsSlice from "./swaps/swaps.slice";
import createPoolsSlice from "./pools/pools.slice";
import createUiSlice from "./ui/ui.slice";
import createBondsSlice from "./bonds/slice";

const useStore = create(
  // persist(
  (set, _get) => ({
    ...createUiSlice(set),
    ...createAssetsSlice(set),
    ...createAuctionsSlice(set),
    ...createSwapsSlice(set),
    ...createBondsSlice(set),
    ...createAddLiquiditySlice(set),
    ...createPoolsSlice(set),
  })
  // {
  //   name: "ui-pablo",
  //   partialize: (state) => ({ assets: state.assets, auctions: state.auctions }),
  // }
  // )
);

export default useStore;
