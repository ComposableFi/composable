import BigNumber from "bignumber.js";
import produce from "immer";
import { LiquiditySlice } from "./liquidity.types";

export const putLiquidityRecord = (
  liquiditySlice: LiquiditySlice["liquidityInPool"],
  liquidityRecord: Record<string, { baseAmount: BigNumber; quoteAmount: BigNumber; }>
) => {
  return produce(liquiditySlice, (draft) => {
    for (const poolId in liquidityRecord) {
      draft[poolId] = {
        baseAmount: liquidityRecord[poolId].baseAmount,
        quoteAmount: liquidityRecord[poolId].quoteAmount
      }
    }
  });
};

export const updatePoolLiquidity = (
  liquiditySlice: LiquiditySlice["liquidityInPool"],
  poolId: string,
  amounts: { baseAmount: BigNumber; quoteAmount: BigNumber; }
) => {
  return produce(liquiditySlice, (draft) => {
    draft[poolId] = {
      baseAmount: amounts.baseAmount,
      quoteAmount: amounts.quoteAmount
    }
  });
};


export const addPoolTokenAmount = (
  liquiditySlice: LiquiditySlice["liquidityInPool"],
  poolId: number,
  amount: {
    baseAmount?: string;
    quoteAmount?: string;
  }
) => {
  return produce(liquiditySlice, (draft) => {
    draft[poolId].baseAmount = new BigNumber(liquiditySlice[poolId].baseAmount ?? 0);
    draft[poolId].baseAmount = draft[poolId].baseAmount.plus(amount.baseAmount ?? 0);

    draft[poolId].quoteAmount = new BigNumber(liquiditySlice[poolId].quoteAmount ?? 0);
    draft[poolId].quoteAmount = draft[poolId].quoteAmount.plus(amount.quoteAmount ?? 0);
  });
};

export const removePoolTokenAmount = (
  liquiditySlice: LiquiditySlice["liquidityInPool"],
  poolId: number,
  amount: {
    baseAmount?: string;
    quoteAmount?: string;
  }
) => {
  return produce(liquiditySlice, (draft) => {
    draft[poolId].baseAmount = new BigNumber(liquiditySlice[poolId].baseAmount ?? 0);
    draft[poolId].baseAmount = draft[poolId].baseAmount.minus(amount.baseAmount ?? 0);

    draft[poolId].quoteAmount = new BigNumber(liquiditySlice[poolId].quoteAmount ?? 0);
    draft[poolId].quoteAmount = draft[poolId].quoteAmount.minus(amount.quoteAmount ?? 0);
  });
};


export const putUserProvidedLiquidityTokenAmount = (
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

export const putUserLpBalance = (
  liquiditySlice: LiquiditySlice["userLpBalances"],
  poolId: number,
  amount: string
) => {
  return produce(liquiditySlice, (draft) => {
    draft[poolId] = amount;
  });
};

export const updateUserProvidedLiquidityTokenAmount = (
  liquiditySlice: LiquiditySlice["userProvidedLiquidity"],
  poolId: number,
  amount: {
    baseAmount?: string;
    quoteAmount?: string;
  }
) => {
  return produce(liquiditySlice, (draft) => {
    if (amount.baseAmount) {
      let prevBase = new BigNumber(0);
      if (liquiditySlice[poolId]) {
        prevBase = new BigNumber(
          liquiditySlice[poolId].tokenAmounts.baseAmount
        );
        draft[poolId].tokenAmounts.baseAmount = prevBase
          .plus(amount.baseAmount)
          .toString();
      }
    }

    if (amount.quoteAmount) {
      let prevQuote = new BigNumber(0);
      if (liquiditySlice[poolId]) {
        prevQuote = new BigNumber(
          liquiditySlice[poolId].tokenAmounts.quoteAmount
        );
        draft[poolId].tokenAmounts.quoteAmount = prevQuote
          .plus(amount.quoteAmount)
          .toString();
      }
    }
  });
};