import create from "zustand";
import BigNumber from "bignumber.js";
import { PabloConstantProductPool } from "shared";

export interface AddLiquiditySlice {
  pool: PabloConstantProductPool | undefined;
  ui: {
    assetOne: string | "none";
    assetTwo: string | "none";
    assetOneAmount: BigNumber;
    assetTwoAmount: BigNumber;
  };
  findPoolManually: boolean;
}

export const useAddLiquiditySlice = create<AddLiquiditySlice>(() => ({
  pool: undefined,
  ui: {
    assetOne: "none",
    assetTwo: "none",
    assetOneAmount: new BigNumber(0),
    assetTwoAmount: new BigNumber(0),
  },
  findPoolManually: true,
}));

export const setPool = (
  pool: PabloConstantProductPool | undefined
) =>
  useAddLiquiditySlice.setState((state) => ({
    ...state,
    pool,
  }));

export const setSelection = (selections: Partial<AddLiquiditySlice["ui"]>) =>
  useAddLiquiditySlice.setState((state) => ({
    ...state,
    ui: {
      assetOne: selections.assetOne ? selections.assetOne : state.ui.assetOne,
      assetTwo: selections.assetTwo ? selections.assetTwo : state.ui.assetTwo,
      assetOneAmount: selections.assetOneAmount
        ? selections.assetOneAmount
        : state.ui.assetOneAmount,
      assetTwoAmount: selections.assetTwoAmount
        ? selections.assetTwoAmount
        : state.ui.assetTwoAmount,
    },
  }));

export const setManualPoolSearch = (searchManually: boolean) =>
  useAddLiquiditySlice.setState((state) => ({
    ...state,
    findPoolManually: searchManually,
  }));

export const resetAddLiquiditySlice = () =>
  useAddLiquiditySlice.setState((state) => ({
    pool: undefined,
    findPoolManually: true,
    ui: {
      assetOne: "none",
      assetTwo: "none",
      assetOneAmount: new BigNumber(0),
      assetTwoAmount: new BigNumber(0),
    },
  }));
