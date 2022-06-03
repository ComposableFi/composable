import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "@/stores/root";
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

export const statsApolloSlice = createSlice({
  name: "statsApollo",
  initialState,
  reducers: {
    setApolloAssets: (
      state: ApolloState,
      action: PayloadAction<ApolloTableData>
    ) => {
      state.assets = { ...state.assets, ...action.payload };
    },
  },
});

export const { setApolloAssets } = statsApolloSlice.actions;

export const selectApolloData = (state: RootState) => state.statsApollo.assets;

export default statsApolloSlice.reducer;
