import { BondOffer } from "@/defi/types";
import BigNumber from "bignumber.js";
import { StoreSlice } from "../types";
import {
  putBondOffer,
  putBondOfferROI,
  putBondOffers,
  putBondOfferTotalPurchased,
} from "./bonds.utils";
import { BondSlice } from "./bonds.types";

const createBondsSlice: StoreSlice<BondSlice> = (set) => ({
  bondOffers: {
    list: [],
    totalPurchased: {},
    roi: {},
  },
  putBondOffers: (bondOffers: BondOffer[]) =>
    set((prev: BondSlice) => ({
      bondOffers: putBondOffers(prev.bondOffers, bondOffers),
    })),
  putBondOffer: (bondOffer: BondOffer) =>
    set((prev: BondSlice) => ({
      bondOffers: putBondOffer(prev.bondOffers, bondOffer),
    })),
  putBondOfferROI: (roi: Record<string, BigNumber>) =>
    set((prev: BondSlice) => ({
      bondOffers: putBondOfferROI(prev.bondOffers, roi),
    })),
  putBondOfferTotalPurchased: (
    totalPurchasedBonds: Record<string, BigNumber>
  ) =>
    set((prev: BondSlice) => ({
      bondOffers: putBondOfferTotalPurchased(
        prev.bondOffers,
        totalPurchasedBonds
      ),
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
