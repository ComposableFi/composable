import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "@/stores/root";
import StatsDummyData from "./dummyData";

export type TreasuryDataProps = {
  name: string;
  value: Array<number>;
  tooltip: string;
};

interface TreasuryData {
  data: Array<TreasuryDataProps>;
}

interface ChartData {
  name: string;
  value: number;
  change: number;
  data: Array<[number, number][]>;
}

interface TreasuryChartData {
  data: Array<{ data: ChartData }>;
}

interface TreasuryBondingData {
  bond: Array<{ label: string; description: string }>;
  claim: Array<{ label: string; description: string }>;
}

interface TreasuryState {
  treasuryData: TreasuryData;
  treasuryChartData: TreasuryChartData;
  treasuryBonding: TreasuryBondingData;
}

const initialState: TreasuryState = {
  treasuryData: {
    data: StatsDummyData.TREASURY.infoData,
  },
  treasuryChartData: {
    data: StatsDummyData.TREASURY.chartData,
  },
  treasuryBonding: {
    bond: StatsDummyData.TREASURY.bonding.bond,
    claim: StatsDummyData.TREASURY.bonding.claim,
  },
};

export const statsTreasurySlice = createSlice({
  name: "statsTreasury",
  initialState,
  reducers: {
    setFeaturedMarketCap: (
      state: TreasuryState,
      action: PayloadAction<TreasuryData["data"][0]>
    ) => {
      state.treasuryData.data[0] = action.payload;
    },
    setFeaturedChaosPriceAndDiscount: (
      state: TreasuryState,
      action: PayloadAction<TreasuryData["data"][1]>
    ) => {
      state.treasuryData.data[1] = action.payload;
    },
    setFeaturedCirculatingSupply: (
      state: TreasuryState,
      action: PayloadAction<TreasuryData["data"][2]>
    ) => {
      state.treasuryData.data[2] = action.payload;
    },
    setFeaturedTreasuryBalance: (
      state: TreasuryState,
      action: PayloadAction<TreasuryData["data"][3]>
    ) => {
      state.treasuryData.data[3] = action.payload;
    },
    setFeaturedChaosApyAndRunway: (
      state: TreasuryState,
      action: PayloadAction<TreasuryData["data"][4]>
    ) => {
      state.treasuryData.data[4] = action.payload;
    },
    setFeaturedSchaos: (
      state: TreasuryState,
      action: PayloadAction<TreasuryData["data"][5]>
    ) => {
      state.treasuryData.data[5] = action.payload;
    },
    setChartMarketCap: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][0]>
    ) => {
      state.treasuryChartData.data[0] = action.payload;
    },
    setChartTreasuryAssetValue: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][1]>
    ) => {
      state.treasuryChartData.data[1] = action.payload;
    },
    setChaosStaked: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][2]>
    ) => {
      state.treasuryChartData.data[2] = action.payload;
    },
    setTreasuryProportions: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][3]>
    ) => {
      state.treasuryChartData.data[3] = action.payload;
    },
    setChartChaosApy: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][4]>
    ) => {
      state.treasuryChartData.data[4] = action.payload;
    },
    setChartRevenue: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][5]>
    ) => {
      state.treasuryChartData.data[5] = action.payload;
    },
    setChartBondProcess: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][6]>
    ) => {
      state.treasuryChartData.data[6] = action.payload;
    },
    setChartTotalLiquidityOwned: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData["data"][7]>
    ) => {
      state.treasuryChartData.data[7] = action.payload;
    },
    setTreasuryData: (
      state: TreasuryState,
      action: PayloadAction<TreasuryData>
    ) => {
      state.treasuryData = action.payload;
    },
    setTreasuryChartData: (
      state: TreasuryState,
      action: PayloadAction<TreasuryChartData>
    ) => {
      state.treasuryChartData = action.payload;
    },
  },
});

export const {
  setFeaturedChaosApyAndRunway,
  setFeaturedChaosPriceAndDiscount,
  setFeaturedCirculatingSupply,
  setFeaturedMarketCap,
  setFeaturedSchaos,
  setFeaturedTreasuryBalance,
  setChartBondProcess,
  setChartChaosApy,
  setChartMarketCap,
  setChartRevenue,
  setChartTotalLiquidityOwned,
  setChartTreasuryAssetValue,
  setTreasuryData,
  setTreasuryChartData,
} = statsTreasurySlice.actions;

export const selectTreasuryFeaturedData = (state: RootState) =>
  state.statsTreasury.treasuryData;
export const selectTreasuryChartData = (state: RootState) =>
  state.statsTreasury.treasuryChartData;

export default statsTreasurySlice.reducer;
