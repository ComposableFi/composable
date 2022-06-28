import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import BigNumber from "bignumber.js";
import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "@/stores/types";

export interface BondsSlice {
  bonds: {
    openPositions: Array<ActiveBond>;
    bonds: Array<BondOffer>;
    bondOfferCount: number | BigNumber;
    total: number;
    setBonds: (bonds: BondOffer[]) => void;
    setBondOfferCount: (bondOfferCount: number | BigNumber) => void;
    updateOpenPositions: (openPositions: ActiveBond[]) => void;
  };
}

export interface VestingSchedule {
  window: {
    blockNumberBased: {
      start: BigNumber;
      period: BigNumber;
    };
  };
  periodCount: BigNumber;
  perPeriod: BigNumber;
}

export interface ActiveBond {
  bond: BondOffer;
  periodCount: BigNumber;
  perPeriod: BigNumber;
  window: {
    blockNumberBased: {
      start: BigNumber;
      period: BigNumber;
    };
  };
}

export const createBondsSlice: StoreSlice<BondsSlice> = (
  set: NamedSet<BondsSlice>
) => ({
  bonds: {
    bonds: [],
    bondOfferCount: 0,
    openPositions: [],
    total: 0,
    setBonds: (bonds: BondOffer[]) =>
      set((state) => ({
        bonds: {
          ...state.bonds,
          bonds,
        },
      })),
    setBondOfferCount: (bondOfferCount: number | BigNumber) =>
      set((state) => ({
        bonds: {
          ...state.bonds,
          bondOfferCount,
        },
      })),
    updateOpenPositions: (openPositions: ActiveBond[]) =>
      set((state) => ({
        bonds: {
          ...state.bonds,
          openPositions,
        },
      })),
  },
});
