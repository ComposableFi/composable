import { TOKENS } from "@/../../packages/tokens";
import BigNumber from "bignumber.js";
import create from "zustand";

export const oracleProviders = ["apollo", "coingecko", "binance"] as const;
export const oracleCurrencies = ["usd"] as const;
export type OracleProvider = typeof oracleProviders[number];
export type OracleCurrency = typeof oracleCurrencies[number];

export interface OracleSlice {
  prices: Record<
    string,
    Record<OracleProvider, Record<OracleCurrency, BigNumber>>
  >;
}

export const oracleSlice = create<OracleSlice>(() => ({
  prices: Object.values(TOKENS).reduce((agg, token) => {
    agg[token.symbol] = {
      coingecko: {
        usd: new BigNumber(0),
      },
      binance: {
        usd: new BigNumber(0),
      },
      apollo: {
        usd: new BigNumber(0),
      },
    };
    return agg;
  }, {} as Record<string, Record<OracleProvider, Record<OracleCurrency, BigNumber>>>),
}));
export const setOraclePrice = (
  currencySymbol: string,
  provider: OracleProvider,
  baseCurrency: OracleCurrency,
  price: BigNumber
) => {
  oracleSlice.setState((state) => {
    state.prices[currencySymbol][provider][baseCurrency] = price;
    return state;
  });
};

export const getOraclePrice = (
  currencySymbol: string,
  provider: OracleProvider,
  baseCurrency: OracleCurrency
) => {
  const prices = oracleSlice.getState().prices;
  let existsSymbol = currencySymbol in prices;
  let existsProvider = provider in prices[currencySymbol];
  let existsBaseCurrency = baseCurrency in prices[currencySymbol][provider];
  let exists = existsSymbol && existsProvider && existsBaseCurrency;
  if (!exists) return new BigNumber(0);

  return prices[currencySymbol][provider][baseCurrency];
};

export const useOracleSlice = () => oracleSlice.getState();
