import BigNumber from "bignumber.js";
import create from "zustand";

export interface FinancialNftSlice {
  // Collection Id => [Instance Ids]
  ownedNfts: Record<string, Array<string>>;
  xTokenBalances: Record<string, Record<string, BigNumber>>;
}

export const useFinancialNftSlice = create<FinancialNftSlice>(() => ({
  ownedNfts: {},
  xTokenBalances: {},
}));

export const setOwnedFinancialNfts = (
  ownedNfts: Record<string, Array<string>>
) => useFinancialNftSlice.setState((state) => ({ ...state, ownedNfts }));

export const resetOwnedFinancialNfts = () => useFinancialNftSlice.setState((state) => ({ ...state, ownedNfts: {} }))

export const useOwnedFinancialNfts = (): Record<string, Array<string>> =>
  useFinancialNftSlice().ownedNfts;

export const putXTokenBalances = (
  balances: Record<string, Record<string, BigNumber>>
) =>
  useFinancialNftSlice.setState((state) => ({
    ...state,
    xTokenBalances: {
      ...state.xTokenBalances,
      ...balances,
    },
  }));

export const useXTokenBalances = (
  collectionId: string
): Record<string, BigNumber> =>
  useFinancialNftSlice().xTokenBalances[collectionId] || {};
