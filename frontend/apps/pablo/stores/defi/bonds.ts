import { TOKENS } from "@/defi/Tokens";
import { BondDetails, Token, TokenId } from "@/defi/types";
import { createSlice } from "@reduxjs/toolkit";
import BigNumber from "bignumber.js";

interface Bond {
  selectedBond: BondDetails;
}

const initialState: Bond = {
  selectedBond: {
    tokenId1: "ksm",
    tokenId2: "pica",
    roi: 26,
    vesting_term: 7,
    tvl: new BigNumber(1500000),
    volumne: new BigNumber(132500000),
    discount_price: new BigNumber(2.3),
    market_price: new BigNumber(2.9),
    balance: new BigNumber(435),
    rewardable_amount: new BigNumber(0),
    buyable_amount: new BigNumber(500),
    pending_amount: new BigNumber(0),
    claimable_amount: new BigNumber(0),
    remaining_term: 7,
    vested_term: 0,
  },
};

export const bondSlice = createSlice({
  name: "Swap",
  initialState,
  reducers: {
    setSelectedBond: (state, action) => {
      state.selectedBond = { ...state.selectedBond, ...action.payload };
    },
  },
});

export const { setSelectedBond } = bondSlice.actions;

export default bondSlice.reducer;
