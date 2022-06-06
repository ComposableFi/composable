import { StoreSlice } from "../types";
import { LiquiditySlice } from "./liquidity.types";
import {
  putTokenAmount,
  putTokenValue,
  putUserLpBalance,
  putUserProvidedTokenAmount,
} from "./liquidity.utils";

const createLiquiditySlice: StoreSlice<LiquiditySlice> = (set) => ({
  poolLiquidity: {},
  userProvidedLiquidity: {},
  userLpBalances: {},
  setTokenAmountInPool: (poolId: number, amounts) =>
    set((prev: LiquiditySlice) => ({
      liquidity: putTokenAmount(prev.poolLiquidity, poolId, amounts),
    })),
  setTokenValueInPool: (poolId: number, value) =>
    set((prev: LiquiditySlice) => ({
      liquidity: putTokenValue(prev.poolLiquidity, poolId, value),
    })),
  setUserProvidedTokenAmountInPool: (poolId: number, amounts) =>
    set((prev: LiquiditySlice) => ({
      userProvidedLiquidity: putUserProvidedTokenAmount(
        prev.userProvidedLiquidity,
        poolId,
        amounts
      ),
    })),
  setUserLpBalance: (poolId: number, amount) =>
    set((prev: LiquiditySlice) => {
      userLpBalances: putUserLpBalance(prev.userLpBalances, poolId, amount);
    }),
});

export default createLiquiditySlice;
