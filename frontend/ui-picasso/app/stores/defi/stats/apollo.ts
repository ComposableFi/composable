import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "@/stores/root";
import StatsDummyData from "./dummyData";

export interface ApolloTableData {
  symbol: string;
  binanceValue: number | undefined;
  // pabloValue: number;
  // aggregatedValue: number;
  apolloValue: number | undefined;
  changeValue: number | undefined;
}

interface ApolloState {
  assets: Array<ApolloTableData>;
}

const initialState: ApolloState = {
  assets: StatsDummyData.APOLLO.assets,
};

export const statsApolloSlice = createSlice({
  name: "statsApollo",
  initialState,
  reducers: {
    setApolloAssets: (
      state: ApolloState,
      action: PayloadAction<Array<ApolloTableData>>
    ) => {
      state.assets = action.payload;
    },
  },
});

export const { setApolloAssets } = statsApolloSlice.actions;

export const selectApolloData = (state: RootState) => state.statsApollo.assets;

export default statsApolloSlice.reducer;
