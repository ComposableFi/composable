import BigNumber from "bignumber.js";
import produce from "immer";
import { LiquiditySlice } from "./liquidity.types";

export const putTokenAmount = (
  liquiditySlice: LiquiditySlice["poolLiquidity"],
  poolId: number,
  amount: {
    baseAmount?: string;
    quoteAmount?: string;
  }
) => {
  return produce(liquiditySlice, (draft) => {
    if (!draft[poolId]) {
      draft[poolId] = {
        tokenAmounts: {
          baseAmount: "0",
          quoteAmount: "0",
        },
        value: {
          baseValue: "0",
          quoteValue: "0"
        },
      };
    }

    if (amount.baseAmount) {
      draft[poolId].tokenAmounts.baseAmount = amount.baseAmount;
    }

    if (amount.quoteAmount) {
      draft[poolId].tokenAmounts.quoteAmount = amount.quoteAmount;
    }
  });
};

export const putTokenValue = (
  liquiditySlice: LiquiditySlice["poolLiquidity"],
  poolId: number,
  value: {
    baseValue?: string;
    quoteValue?: string;
  }
) => {
  return produce(liquiditySlice, (draft) => {
    if (!liquiditySlice[poolId]) {
      draft[poolId] = {
        tokenAmounts: {
          baseAmount: "0",
          quoteAmount: "0",
        },
        value: {
          baseValue: "0",
          quoteValue: "0",
        },
      };
    }
    
    if (value.baseValue) {
      draft[poolId].value.baseValue = value.baseValue;
    }

    if (value.quoteValue) {
      draft[poolId].value.quoteValue = value.quoteValue;
    }

  });
};

export const putUserProvidedTokenAmount = (
  liquiditySlice: LiquiditySlice["userProvidedLiquidity"],
  poolId: number,
  amount: {
    baseAmount?: string;
    quoteAmount?: string;
  }
) => {
  return produce(liquiditySlice, (draft) => {
    if (!draft[poolId]) {
      draft[poolId] = {
        tokenAmounts: {
          baseAmount: "0",
          quoteAmount: "0",
        },
      };
    }

    if (amount.baseAmount) {
      draft[poolId].tokenAmounts.baseAmount = amount.baseAmount;
    }

    if (amount.quoteAmount) {
      draft[poolId].tokenAmounts.quoteAmount = amount.quoteAmount;
    }
  });
};