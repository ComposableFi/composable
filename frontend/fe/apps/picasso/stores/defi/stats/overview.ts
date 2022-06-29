import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "@/stores/root";
import StatsDummyData from "./dummyData";

const CHART_INTERVAL = ["1h", "24h", "1w", "1m", "1y"];

export type OverviewDataProps = {
  name: string;
  value: number;
};

interface OverviewData {
  data: Array<OverviewDataProps>;
}

interface ChartData {
  name: string;
  value: number;
  change: number;
  data: Array<[number, number][]>;
  interval: Array<string>;
  pickedInterval: number;
}

interface OverviewChartData {
  data: Array<{ data: ChartData }>;
}

interface OverviewState {
  overviewData: OverviewData;
  overviewChartData: OverviewChartData;
}

const initialState: OverviewState = {
  overviewData: {
    data: StatsDummyData.OVERVIEW.infoData,
  },
  overviewChartData: {
    data: [
      {
        data: {
          name: "Total value locked",
          value: 54653784,
          change: 34,
          data: StatsDummyData.OVERVIEW.chartData.tvl,
          interval: CHART_INTERVAL,
          pickedInterval: 0,
        },
      },
      {
        data: {
          name: "Daily active users",
          value: 12567,
          change: -1.31,
          data: StatsDummyData.OVERVIEW.chartData.users,
          interval: CHART_INTERVAL,
          pickedInterval: 0,
        },
      },
    ],
  },
};

export const statsOverviewSlice = createSlice({
  name: "statsData",
  initialState,
  reducers: {
    setTVL: (
      state: OverviewState,
      action: PayloadAction<OverviewData["data"][0]>
    ) => {
      state.overviewData.data[0] = action.payload;
    },
    setAccountHolders: (
      state: OverviewState,
      action: PayloadAction<OverviewData["data"][1]>
    ) => {
      state.overviewData.data[1] = action.payload;
    },
    setTotalTx: (
      state: OverviewState,
      action: PayloadAction<OverviewData["data"][2]>
    ) => {
      state.overviewData.data[2] = action.payload;
    },
    setRewardDistribution: (
      state: OverviewState,
      action: PayloadAction<OverviewData["data"][3]>
    ) => {
      state.overviewData.data[3] = action.payload;
    },
    setTotalFees: (
      state: OverviewState,
      action: PayloadAction<OverviewData["data"][4]>
    ) => {
      state.overviewData.data[4] = action.payload;
    },
    setEarnedStakingTvl: (
      state: OverviewState,
      action: PayloadAction<OverviewData["data"][5]>
    ) => {
      state.overviewData.data[5] = action.payload;
    },
    setTvlInterval: (state: OverviewState, action: PayloadAction<number>) => {
      state.overviewChartData.data[0].data.pickedInterval = action.payload;
    },
    setDailyActiveUsersInterval: (
      state: OverviewState,
      action: PayloadAction<number>
    ) => {
      state.overviewChartData.data[1].data.pickedInterval = action.payload;
    },
  },
});

export const {
  setTVL,
  setAccountHolders,
  setTotalTx,
  setRewardDistribution,
  setTotalFees,
  setEarnedStakingTvl,
  setTvlInterval,
  setDailyActiveUsersInterval,
} = statsOverviewSlice.actions;

export const selectOverviewData = (state: RootState) =>
  state.statsOverview.overviewData;
export const selectOverviewChartData = (state: RootState) =>
  state.statsOverview.overviewChartData;

export default statsOverviewSlice.reducer;
