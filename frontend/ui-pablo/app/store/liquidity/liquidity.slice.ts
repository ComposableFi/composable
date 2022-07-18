import { StoreSlice } from "../types";
import { LiquiditySlice } from "./liquidity.types";
import {
  putTokenAmount,
  putTokenValue,
  putUserLpBalance,
  putUserProvidedLiquidityTokenAmount,
  updateUserProvidedLiquidityTokenAmount,
} from "./liquidity.utils";

const createLiquiditySlice: StoreSlice<LiquiditySlice> = (set) => ({
  poolLiquidity: {},
  userProvidedLiquidity: {},
  userLpBalances: {},
  setTokenAmountInLiquidityPool: (poolId: number, amounts) =>
    set((prev: LiquiditySlice) => ({
      poolLiquidity: putTokenAmount(prev.poolLiquidity, poolId, amounts),
    })),
  setTokenValueInLiquidityPool: (poolId: number, value) =>
    set((prev: LiquiditySlice) => ({
      poolLiquidity: putTokenValue(prev.poolLiquidity, poolId, value),
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
