import BigNumber from "bignumber.js";
import create from "zustand";

const oracleProviders = ["apollo", "coingecko", "binance"] as const;
const oracleCurrencies = ["USD"] as const;
export type OracleProvider = typeof oracleProviders[number];
export type OracleCurrency = typeof oracleCurrencies[number];

export interface OracleSlice {
  prices: Record<
    string,
    Record<OracleProvider, Record<OracleCurrency, BigNumber>>
  >;
}

export const oracleSlice = create<OracleSlice>(() => ({ prices: {} }));
export const setOraclePrice = (
  currencySymbol: string,
  provider: OracleProvider,
  baseCurrency: OracleCurrency,
  price: BigNumber
) => {
  oracleSlice.setState((state) => {
    let existsSymbol = currencySymbol in state.prices;
    let existsProvider = provider in state.prices[currencySymbol];
    let existsBaseCurrency = baseCurrency in state.prices[currencySymbol][provider];
    let exists = existsSymbol && existsProvider && existsBaseCurrency;
    if (exists) {
      state.prices[currencySymbol][provider][baseCurrency] = new BigNumber(
        price
      );
    } else {
      for (const _provider of oracleProviders) {
        state.prices[currencySymbol][_provider] = oracleCurrencies.reduce(
          (agg, curr) => {
            agg[curr] = new BigNumber(0);
            return agg;
          },
          {} as Record<OracleCurrency, BigNumber>
        );

        state.prices[currencySymbol][provider][baseCurrency] = price;
      }
    }

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
    let exists = existsSymbol && existsProvider && existsBaseCurrency
    if (!exists) return new BigNumber(0);

    return prices[currencySymbol][provider][baseCurrency]
}