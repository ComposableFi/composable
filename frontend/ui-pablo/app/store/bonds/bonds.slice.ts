import { BondOffer } from "@/defi/types";
import { StoreSlice } from "../types";
import { putBondOffer, putBondOffers } from "./bonds.reducers";
import { BondSlice } from "./bonds.types";

const createBondsSlice: StoreSlice<BondSlice> = (set) => ({
  bondOffers: {
    list: []
  },
  putBondOffers: (
    bondOffers: BondOffer[]
  ) => set((prev: BondSlice) => ({
    bondOffers: putBondOffers(prev.bondOffers, bondOffers)
  })),
  putBondOffer: (
    bondOffer: BondOffer
  ) => set((prev: BondSlice) => ({
    bondOffers: putBondOffer(prev.bondOffers, bondOffer)
  })),
  reset: () =>
    set(() => ({
      allBonds: [],
      activeBonds: [],
    })),
});

export default createBondsSlice;
