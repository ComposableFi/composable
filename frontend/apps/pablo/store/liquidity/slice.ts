import BigNumber from "bignumber.js";
import { StoreSlice } from "../types";
import { LiquiditySlice } from "./types";

const createLiquiditySlice: StoreSlice<LiquiditySlice> = (set) => ({
  liquidityInPool: {},
  userProvidedLiquidity: {},
  userLpBalances: {},
  putLiquidityInPoolRecord: (record) =>
    set((state) => {
      for (const poolId in record) {
        state.liquidityInPool[poolId] = {
          baseAmount: record[poolId].baseAmount,
          quoteAmount: record[poolId].quoteAmount
        }
      }
      return state;
    }),
  updatePoolLiquidity: (id, amounts) =>
    set((state) => {
      state.liquidityInPool[id].baseAmount = amounts.baseAmount;
      state.liquidityInPool[id].quoteAmount = amounts.quoteAmount;
      return state;
    }),
  setUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts) => set((state) => {
    if (amounts.baseAmount) state.userProvidedLiquidity[poolId].tokenAmounts.baseAmount = amounts.baseAmount;
    if (amounts.quoteAmount) state.userProvidedLiquidity[poolId].tokenAmounts.quoteAmount = amounts.quoteAmount;
    return state;
  }),
  updateUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts) =>
    set((state) => {
      
      if (amounts.baseAmount) {
        let prevBase = new BigNumber(0);
        if (state.userProvidedLiquidity[poolId]) {
          prevBase = new BigNumber(
            state.userProvidedLiquidity[poolId].tokenAmounts.baseAmount
          );
          state.userProvidedLiquidity[poolId].tokenAmounts.baseAmount = prevBase
            .plus(amounts.baseAmount)
            .toString();
        }
      }
  
      if (amounts.quoteAmount) {
        let prevQuote = new BigNumber(0);
        if (state.userProvidedLiquidity[poolId]) {
          prevQuote = new BigNumber(
            state.userProvidedLiquidity[poolId].tokenAmounts.quoteAmount
          );
          state.userProvidedLiquidity[poolId].tokenAmounts.quoteAmount = prevQuote
            .plus(amounts.quoteAmount)
            .toString();
        }
      }

      
      return state;
    }),
  setUserLpBalance: (poolId: number, amount) =>
    set((state) => {
      state.userLpBalances[poolId] = amount
      return state;
    }),
});

export default createLiquiditySlice;
