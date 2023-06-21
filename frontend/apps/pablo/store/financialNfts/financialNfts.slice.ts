import BigNumber from "bignumber.js";
import create from "zustand";
import { FinancialNft } from "shared";

export interface FinancialNftSlice {
  // Collection Id => [Instance Ids]
  ownedNfts: FinancialNft[];
  xTokenBalances: Record<string, Record<string, BigNumber>>;
}

export const useFinancialNftSlice = create<FinancialNftSlice>(() => ({
  ownedNfts: [],
  xTokenBalances: {},
}));

export const setOwnedFinancialNfts = (
  ownedNfts: FinancialNft[]
) => useFinancialNftSlice.setState((state) => ({ ...state, ownedNfts }));

export const resetOwnedFinancialNfts = () => useFinancialNftSlice.setState((state) => ({ ...state, ownedNfts: [] }))

export const useOwnedFinancialNfts = (): FinancialNft[] =>
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
