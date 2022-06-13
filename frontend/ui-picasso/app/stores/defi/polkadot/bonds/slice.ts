import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { createSlice } from "@reduxjs/toolkit";

export interface BondsSlice {
  bonds: Array<BondOffer>;
  total: number;
}

const initialState = {
  bonds: [],
  bondOfferCount: 0,
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
  },
});

export const { setBonds, setBondOfferCount } = bondsSlice.actions;

export default bondsSlice.reducer;
