import { BondOffer } from "@/defi/types";
import BigNumber from "bignumber.js";
import { StoreSlice } from "../types";
import {
  putBondOffer,
  putBondOffers,
  putTotalPurchased,
} from "./bonds.reducers";
import { BondSlice } from "./bonds.types";

const createBondsSlice: StoreSlice<BondSlice> = (set) => ({
  bondOffers: {
    list: [],
    totalPurchased: {},
  },
  putBondOffers: (bondOffers: BondOffer[]) =>
    set((prev: BondSlice) => ({
      bondOffers: putBondOffers(prev.bondOffers, bondOffers),
    })),
  putBondOffer: (bondOffer: BondOffer) =>
    set((prev: BondSlice) => ({
      bondOffers: putBondOffer(prev.bondOffers, bondOffer),
    })),
  puttotalPurchased: (totalPurchasedBonds: Record<string, BigNumber>) =>
    set((prev: BondSlice) => ({
      bondOffers: putTotalPurchased(prev.bondOffers, totalPurchasedBonds),
    })),
  reset: () =>
    set(() => ({
      bondOffers: {
        list: [],
        totalPurchased: {},
      },
    })),
});

export default createBondsSlice;
