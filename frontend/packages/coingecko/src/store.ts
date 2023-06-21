import { TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";
import create from "zustand";
import { immer } from "zustand/middleware/immer";

export const currencies = ["usd"] as const;
export type CoingeckoCurrencies = typeof currencies[number];
export type CoingeckoCurrencyChange = `${CoingeckoCurrencies}_24h_change`;
type ChangeType = Record<CoingeckoCurrencyChange, number>;
type CurrencyPrice = Record<CoingeckoCurrencies, BigNumber>;

type Price = ChangeType & CurrencyPrice;
export type CoingeckoState = {
  prices: {
    [key in TokenId]: Price;
  };
};
export type CoingeckoActions = {
  setPrice: (
    token: TokenId,
    value: BigNumber,
    currency: CoingeckoCurrencies,
    change: number
  ) => void;
};

const initialState = Object.values(TOKENS).reduce((agg, token) => {
  agg[token.id] = {
    usd: new BigNumber(0),
    usd_24h_change: 0,
  };
  return agg;
}, {} as CoingeckoState["prices"]);

export type CoingeckoSlice = CoingeckoState & CoingeckoActions;
export const useCoingecko = create<CoingeckoSlice>()(
  immer((set) => ({
    prices: initialState,
    setPrice: (
      tokenId: TokenId,
      price: BigNumber,
      currency: CoingeckoCurrencies,
      change
    ) => {
      set((state) => {
        state.prices[tokenId] = {
          [currency]: price,
          [`${currency}_24h_change`]: change,
        } as Price;
      });
    },
  }))
);
