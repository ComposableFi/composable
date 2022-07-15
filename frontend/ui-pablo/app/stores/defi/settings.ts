import { TransactionSettings } from '@/defi/types';
import { createSlice } from '@reduxjs/toolkit';
import BigNumber from 'bignumber.js';

interface Settings {
  maxTolerance?: number,
  minTolerance?: number,
  maxDeadline?: number,
  minDeadline?: number,
  transactionSettings: TransactionSettings,
}

const initialState: Settings = {
  maxTolerance: 100,
  minTolerance: 0,
  maxDeadline: 9999,
  minDeadline: 0,
  transactionSettings: {
    tolerance: 0.1,
    deadline: 20,
  },
};

export const settingsSlice = createSlice({
  name: "Pool",
  initialState,
  reducers: {
    setTransactionSettings: (state, action) => {
      state.transactionSettings = {...state.transactionSettings, ...action.payload};
    },
  },
});

export const {
  setTransactionSettings,
} = settingsSlice.actions;


export default settingsSlice.reducer;