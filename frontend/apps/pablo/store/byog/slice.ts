import { TokenId } from "tokens";
import BigNumber from "bignumber.js";
import { StoreSlice } from "@/store/types";

type BYOGState = {
  feeItem: TokenId;
  feeItemEd: BigNumber;
};

type BYOGActions = {
  setFeeItem: (data: TokenId) => void;
  setFeeItemEd: (value: BigNumber) => void;
};

export type BYOGSlice = {
  byog: BYOGState & BYOGActions;
};

const initialState: BYOGState = {
  feeItem: "pica",
  feeItemEd: new BigNumber(0),
};

export const createBYOGSlice: StoreSlice<BYOGSlice> = (set) => ({
  byog: {
    setFeeItemEd: (value: BigNumber) => {
      set((state) => {
        state.byog.feeItemEd = value;
      });
    },
    setFeeItem: (data: TokenId) =>
      set((state) => {
        state.byog.feeItem = data;
      }),
    ...initialState,
  },
});
