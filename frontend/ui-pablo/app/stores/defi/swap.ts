import { TOKENS } from "@/defi/Tokens";
import { Token } from "@/defi/types";
import { createSlice } from "@reduxjs/toolkit";
import BigNumber from "bignumber.js";

interface Swap {
  swap: {
    token1PriceInUSD: BigNumber;
    token2PriceInUSD: BigNumber;
  };
  percentageToSwap: number;
}

const initialState: Swap = {
  swap: {
    token1PriceInUSD: new BigNumber(1),
    token2PriceInUSD: new BigNumber(1),
  },
  percentageToSwap: 50,
};

export const swapSlice = createSlice({
  name: "Swap",
  initialState,
  reducers: {
    setSwap: (state, action) => {
      state.swap = { ...state.swap, ...action.payload };
    },
  },
});

export const { setSwap } = swapSlice.actions;

export default swapSlice.reducer;
