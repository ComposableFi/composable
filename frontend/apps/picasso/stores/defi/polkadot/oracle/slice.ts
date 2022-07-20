import BigNumber from "bignumber.js";
import { currencyIdToAssetMap } from "@/stores/defi/polkadot/bonds/constants";
import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "@/stores/types";

type OraclePrice = {
  price: BigNumber;
  block: BigNumber;
};

type Price = Record<keyof typeof currencyIdToAssetMap, OraclePrice>;
export type OracleSlice = {
  oracle: {
    prices: Price;
  };
};

const initialState: OracleSlice = {
  oracle: {
    prices: getInitialPrices(),
  },
};

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

export const createOracleSlice: StoreSlice<OracleSlice> = (
  set: NamedSet<OracleSlice>
) => ({
  ...initialState,
  updatePrice: (prices: Price) =>
    set((state) => ({
      ...state,
      oracle: {
        ...state.oracle,
        prices,
      },
    })),
});
