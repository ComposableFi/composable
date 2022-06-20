import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { createSlice } from "@reduxjs/toolkit";
import BigNumber from "bignumber.js";

export interface BondsSlice {
  openPositions: Array<ActiveBond>;
  bonds: Array<BondOffer>;
  total: number;
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

const initialState = {
  bonds: [],
  bondOfferCount: 0,
  openPositions: [],
};

export const bondsSlice = createSlice({
  name: "Bonds",
  initialState,
  reducers: {
    setBonds: (state, action) => {
      state.bonds = action.payload;
    },
    setBondOfferCount: (state, action) => {
      state.bondOfferCount = action.payload;
    },
    updateOpenPositions: (state, action) => {
      state.openPositions = action.payload;
    },
  },
});

export const { setBonds, setBondOfferCount, updateOpenPositions } = bondsSlice.actions;

export default bondsSlice.reducer;
