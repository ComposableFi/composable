import { createSlice } from "@reduxjs/toolkit";

interface Swap {
  percentageToSwap: number;
}

const initialState: Swap = {
  percentageToSwap: 50,
};

export const swapSlice = createSlice({
  name: "Swap",
  initialState,
  reducers: {
  },
});

export default swapSlice.reducer;
