import { BondOffer } from "@/defi/types";
import { StoreSlice } from "../types";
import { putBondOffers } from "./bonds.reducers";
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
  reset: () =>
    set(() => ({
      allBonds: [],
      activeBonds: [],
    })),
});

export default createBondsSlice;
