import { StoreSlice } from "../types";
import { LiquiditySlice } from "./liquidity.types";
import { putTokenAmount, putTokenValue, putUserProvidedTokenAmount } from "./liquidity.utils";

const createLiquiditySlice: StoreSlice<LiquiditySlice> = (set) => ({
  poolLiquidity: {},
  userProvidedLiquidity: {},
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
    userProvidedLiquidity: putUserProvidedTokenAmount(prev.userProvidedLiquidity, poolId, amounts),
  })),
});

export default createLiquiditySlice;
