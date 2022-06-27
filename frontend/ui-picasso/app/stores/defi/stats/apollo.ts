import { NamedSet } from "zustand/middleware";
import { AllSlices, StoreSlice } from "../../types";
import StatsDummyData from "./dummyData";

interface ApolloTableData {
  symbol: string;
  binanceValue: number;
  pabloValue: number;
  aggregatedValue: number;
  apolloValue: number;
  changeValue: number;
}

interface ApolloState {
  assets: Array<ApolloTableData>;
}

const initialState: ApolloState = {
  assets: StatsDummyData.APOLLO.assets,
};

export interface StatsApolloSlice {
  statsApollo: ApolloState & {
    setApolloAssets: (data: ApolloTableData) => void;
  };
}

export const createStatsApolloSlice: StoreSlice<StatsApolloSlice> = (
  set: NamedSet<StatsApolloSlice>
) => ({
  statsApollo: {
    ...initialState,
    setApolloAssets: (data: ApolloTableData) => {
      set((state: AllSlices) => {
        state.statsApollo.assets = { ...state.statsApollo.assets, ...data };
      });
    },
  },
});
