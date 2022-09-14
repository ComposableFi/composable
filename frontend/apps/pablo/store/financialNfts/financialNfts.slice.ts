import create from "zustand";

export interface FinancialNftSlice {
  // Collection Id => [Instance Ids]
  ownedNfts: Record<string, Array<string>>;
}

export const useFinancialNftSlice = create<FinancialNftSlice>(() => ({
  ownedNfts: {},
}));

export const setOwnedFinancialNfts = (
  ownedNfts: Record<string, Array<string>>
) => useFinancialNftSlice.setState((state) => ({ ownedNfts }));

export const useOwnedFinancialNfts = (): Record<string, Array<string>> =>
  useFinancialNftSlice().ownedNfts;
