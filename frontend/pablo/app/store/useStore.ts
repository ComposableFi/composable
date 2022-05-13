import create from "zustand";
import { persist } from "zustand/middleware";
import createAssetsSlice from "./assets/assets.slice";
import createConstantProductPoolsSlice from "./pools/constantProduct/constantProduct.slice";
import createLiquidityBootstrappingPoolSlice from "./pools/liquidityBootstrapping/liquidityBootstrapping.slice";
import createAuctionsSlice from "./auctions/auctions.slice";
import createSwapsSlice from "./swaps/swaps.slice";
import createUiSlice from "./ui/ui.slice";

const useStore = create(
  // persist(
    (set, _get) => ({
      ...createUiSlice(set),
      ...createAssetsSlice(set),
      ...createAuctionsSlice(set),
      ...createLiquidityBootstrappingPoolSlice(set),
      ...createConstantProductPoolsSlice(set),
      ...createSwapsSlice(set),
    }),
    // {
    //   name: "ui-pablo",
    //   partialize: (state) => ({ assets: state.assets, auctions: state.auctions }),
    // }
  // )
);

export default useStore;