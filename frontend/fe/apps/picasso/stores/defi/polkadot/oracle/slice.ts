import BigNumber from "bignumber.js";
import { currencyIdToAssetMap } from "@/stores/defi/polkadot/bonds/constants";
import { createSlice } from "@reduxjs/toolkit";

type OraclePrice = {
  price: BigNumber;
  block: BigNumber;
};

type OracleState = {
  prices: Record<keyof typeof currencyIdToAssetMap, OraclePrice>;
};

const initialState: OracleState = {
  prices: getInitialPrices(),
};

const oracleSlice = createSlice({
  name: "Oracle",
  initialState,
  reducers: {
    updatePrice: (state, action) => {
      state.prices = {
        ...state.prices,
        ...action.payload,
      };
    },
  },
});

export const { updatePrice } = oracleSlice.actions;

function getInitialPrices(): Record<string, OraclePrice> {
  return Object.fromEntries(
    Object.entries(currencyIdToAssetMap).map(([id]) => {
      return [
        id,
        {
          price: new BigNumber(0),
          block: new BigNumber(0),
        },
      ];
    })
  );
}

export default oracleSlice.reducer;
