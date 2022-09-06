import { StoreSlice } from "../types";
import { LiquiditySlice } from "./liquidity.types";
import {
  putLiquidityRecord,
  putUserLpBalance,
  putUserProvidedLiquidityTokenAmount,
  updateUserProvidedLiquidityTokenAmount,
  updatePoolLiquidity,
} from "./liquidity.utils";

const createLiquiditySlice: StoreSlice<LiquiditySlice> = (set) => ({
  liquidityInPool: {},
  userProvidedLiquidity: {},
  userLpBalances: {},
  putLiquidityInPoolRecord: (record) =>
    set((prev: LiquiditySlice) => ({
      liquidityInPool: putLiquidityRecord(prev.liquidityInPool, record),
    })),
  updatePoolLiquidity: (id, amounts) =>
    set((prev: LiquiditySlice) => ({
      liquidityInPool: updatePoolLiquidity(prev.liquidityInPool, id, amounts),
    })),
  setUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts) =>
    set((prev: LiquiditySlice) => ({
      userProvidedLiquidity: putUserProvidedLiquidityTokenAmount(
        prev.userProvidedLiquidity,
        poolId,
        amounts
      ),
    })),
  updateUserProvidedTokenAmountInLiquidityPool: (poolId: number, amounts) =>
    set((prev: LiquiditySlice) => ({
      userProvidedLiquidity: updateUserProvidedLiquidityTokenAmount(
        prev.userProvidedLiquidity,
        poolId,
        amounts
      ),
    })),
  setUserLpBalance: (poolId: number, amount) =>
    set((prev: LiquiditySlice) => ({
      userLpBalances: putUserLpBalance(prev.userLpBalances, poolId, amount),
    })),
});

export default createLiquiditySlice;
