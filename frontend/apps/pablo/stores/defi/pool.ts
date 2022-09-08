import { PoolLiquidityChartData } from '@/defi/types';
import { createSlice } from '@reduxjs/toolkit';
import { initSupplyData } from '../dummy/pool';

interface Pool {
  currentSupply: {
    confirmed: boolean;
  },
  selectedPoolLiquidityChartData: PoolLiquidityChartData,
}

const initialState: Pool = {
  currentSupply: initSupplyData,
  selectedPoolLiquidityChartData: {
    series: [80, 20],
    labels: ["My Position", "Total Value Locked"],
  }
};

export const poolSlice = createSlice({
  name: "Pool",
  initialState,
  reducers: {
    setCurrentSupply: (state, action) => {
      state.currentSupply = {...state.currentSupply, ...action.payload};
    },
  },
});

export const {
  setCurrentSupply,
} = poolSlice.actions;

export default poolSlice.reducer;