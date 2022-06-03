import { StoreSlice } from "../types";
import { setActiveBonds, setAllBonds } from "./reducers";
import { BondOffer, BondSlice } from "./types";

const createBondsSlice: StoreSlice<BondSlice> = (set) => ({
  allBonds: [],
  activeBonds: [],
  setActiveBonds: (bondOffer: BondOffer) =>
    set((prev: BondSlice) => ({
      activeBonds: setActiveBonds(prev.activeBonds, bondOffer),
    })),
  setAllBonds: (bondOffer: BondOffer) =>
    set((prev: BondSlice) => ({
      allBonds: setAllBonds(prev.allBonds, bondOffer),
    })),
});

export default createBondsSlice;
