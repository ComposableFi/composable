import { TokenId } from "tokens";
import BigNumber from "bignumber.js";
import { StoreSlice } from "@/store/types";

type BYOGState = {
  feeItem: TokenId;
  feeItemEd: BigNumber;
  isLoaded: boolean;
};

type BYOGActions = {
  setFeeItem: (data: TokenId) => void;
  setFeeItemEd: (value: BigNumber) => void;
  setLoaded: (value: boolean) => void;
};

export type BYOGSlice = {
  byog: BYOGState & BYOGActions;
};

const initialState: BYOGState = {
  feeItem: "pica",
  feeItemEd: new BigNumber(0),
  isLoaded: false,
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
    setLoaded: (value: boolean) => {
      set((state) => {
        state.byog.isLoaded = value;
      });
    },
    ...initialState,
  },
});
