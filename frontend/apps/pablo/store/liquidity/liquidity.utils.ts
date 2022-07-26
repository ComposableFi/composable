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
          quoteValue: "0",
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

export const addPoolTokenAmount = (
  liquiditySlice: LiquiditySlice["poolLiquidity"],
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

    if (amount.baseAmount) {
      let prevQuote = new BigNumber(0);
      if (liquiditySlice[poolId]) {
        prevQuote = new BigNumber(
          liquiditySlice[poolId].tokenAmounts.quoteAmount
        );
        draft[poolId].tokenAmounts.baseAmount = prevQuote
          .plus(amount.baseAmount)
          .toString();
      }
    }
  });
};

export const removePoolTokenAmount = (
  liquiditySlice: LiquiditySlice["poolLiquidity"],
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
          .minus(amount.baseAmount)
          .toString();
      }
    }

    if (amount.baseAmount) {
      let prevQuote = new BigNumber(0);
      if (liquiditySlice[poolId]) {
        prevQuote = new BigNumber(
          liquiditySlice[poolId].tokenAmounts.quoteAmount
        );
        draft[poolId].tokenAmounts.baseAmount = prevQuote
          .minus(amount.baseAmount)
          .toString();
      }
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