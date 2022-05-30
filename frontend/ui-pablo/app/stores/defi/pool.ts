import { Liquidity, PoolInfo, Supply, TokenId } from '@/defi/types';
import { createSlice } from '@reduxjs/toolkit';
import BigNumber from 'bignumber.js';
import { initPoolData, initSupplyData } from '../dummy/pool';
import { RootState } from "../root";

interface Pool {
  currentSupply: Supply;
  currentLiquidity: Liquidity;
  currentStep: number;
  currentPool: PoolInfo;
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
    amount: new BigNumber(1.57),
  },
  currentStep: 1,
  currentPool: initPoolData,
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
    setCurrentStep: (state, action) => {
      state.currentStep = action.payload;
    },
    initCurrentSupply: (state) => {
      state.currentSupply = {...initSupplyData};
    },
    initCreatePool: (state) => {
      state.currentStep = 1;
      state.currentPool = {...initPoolData};
      state.currentSupply = {...initSupplyData};
    },
  },
});

export const {
  setCurrentSupply,
  setCurrentLiquidity,
  setCurrentPool,
  setCurrentStep,
  initCurrentSupply,
  initCreatePool,
} = poolSlice.actions;

export const getTokenIdsFromPool = ({ pool: { currentPool } }: RootState) => ({
  tokenId1: currentPool.tokenId1 as TokenId,
  tokenId2: currentPool.tokenId2 as TokenId
});

export const getTokenIdsFromSupply = ({ pool: { currentSupply } }: RootState) => ({
  tokenId1: currentSupply.tokenId1 as TokenId,
  tokenId2: currentSupply.tokenId2 as TokenId
});

export default poolSlice.reducer;