import { Liquidity, PoolDetails, PoolInfo, PoolLiquidityChartData, Supply, TokenId } from '@/defi/types';
import { createSlice } from '@reduxjs/toolkit';
import BigNumber from 'bignumber.js';
import { initPoolData, initSupplyData, selectedPoolData } from '../dummy/pool';

interface Pool {
  currentSupply: Supply;
  currentLiquidity: Liquidity;
  currentStep: number;
  currentPool: PoolInfo;
  selectedPool: PoolDetails;
  selectedPoolLiquidityChartData: PoolLiquidityChartData,
}

const initialState: Pool = {
  currentSupply: initSupplyData,
  currentLiquidity: {
    tokenId1: 'ksm',
    tokenId2: 'pica',
    pooledAmount1: new BigNumber(59.28),
    pooledAmount2: new BigNumber(592.8),
    price1: new BigNumber(10),
    price2: new BigNumber(0.1),
    share: new BigNumber(3.3),
    amount: new BigNumber(1200),
  },
  currentStep: 1,
  currentPool: initPoolData,
  selectedPool: selectedPoolData,
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
    setCurrentLiquidity: (state, action) => {
      state.currentLiquidity = {...action.payload};
    },
    setCurrentPool: (state, action) => {
      state.currentPool = {...state.currentPool, ...action.payload};
    },
  },
});

export const {
  setCurrentSupply,
  setCurrentLiquidity,
  setCurrentPool,
} = poolSlice.actions;

export default poolSlice.reducer;