import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import BigNumber from "bignumber.js";
import { StoreSlice } from "@/stores/types";

export interface BondsSlice {
  bonds: {
    activeBonds: Array<ActiveBond>;
    bonds: Array<BondOffer>;
    bondOfferCount: number | BigNumber;
    total: number;
    setBonds: (bonds: BondOffer[]) => void;
    setBondOfferCount: (bondOfferCount: number | BigNumber) => void;
    setActiveBonds: (activeBonds: ActiveBond[]) => void;
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
  alreadyClaimed: number;
  periodCount: BigNumber;
  perPeriod: BigNumber;
  vestingScheduleId: number;
  window: {
    blockNumberBased: {
      start: BigNumber;
      period: BigNumber;
    };
  };
}

export const createBondsSlice: StoreSlice<BondsSlice> = (set) => ({
  bonds: {
    bonds: [],
    bondOfferCount: 0,
    activeBonds: [],
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
    setActiveBonds: (activeBonds: ActiveBond[]) => {
      set((state) => ({
        bonds: {
          ...state.bonds,
          activeBonds,
        },
      }));
    },
  },
});
