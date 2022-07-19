import { createSlice } from '@reduxjs/toolkit';

interface AuctionsState {
  auctionsTableLimit: number,
  histiriesTableLimit: number,
};

const initialState: AuctionsState = {
  auctionsTableLimit: 4,
  histiriesTableLimit: 10
};

export const auctionsSlice = createSlice({
  name: "Auctions",
  initialState,
  reducers: {
  },
});


export default auctionsSlice.reducer;