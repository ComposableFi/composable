import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { RootState } from "@/stores/root";
import StatsDummyData from "./dummyData";

const CHART_INTERVAL = ["1h", "24h", "1w", "1m", "1y"];

export type TelemetryDataProps = {
  name: string;
  value: number;
};

interface TelemetryData {
  data: Array<TelemetryDataProps>;
}

interface ChartData {
  name: string;
  data: Array<[number, number][]>;
  interval: Array<string>;
  pickedInterval: number;
}

interface TelemetryChartData {
  data: Array<{ data: ChartData }>;
}

interface TelemetryState {
  telemetryData: TelemetryData;
  telemetryChartData: TelemetryChartData;
}

const initialState: TelemetryState = {
  telemetryData: {
    data: StatsDummyData.TELEMETRY.infoData,
  },
  telemetryChartData: {
    data: [
      {
        data: {
          name: "Mempool & fee growth",
          interval: CHART_INTERVAL,
          pickedInterval: 0,
          data: StatsDummyData.TELEMETRY.chartData.memPool,
        },
      },
    ],
  },
};

export const statsTelemetrySlice = createSlice({
  name: "telemetryData",
  initialState,
  reducers: {
    setFinalizedBlock: (
      state: TelemetryState,
      action: PayloadAction<TelemetryData["data"][0]>
    ) => {
      state.telemetryData.data[0] = action.payload;
    },
    setAverageTime: (
      state: TelemetryState,
      action: PayloadAction<TelemetryData["data"][1]>
    ) => {
      state.telemetryData.data[1] = action.payload;
    },
    setLastBlock: (
      state: TelemetryState,
      action: PayloadAction<TelemetryData["data"][2]>
    ) => {
      state.telemetryData.data[2] = action.payload;
    },
    setMemPoolInterval: (
      state: TelemetryState,
      action: PayloadAction<number>
    ) => {
      state.telemetryChartData.data[0].data.pickedInterval = action.payload;
    },
  },
});

export const {
  setFinalizedBlock,
  setAverageTime,
  setLastBlock,
  setMemPoolInterval,
} = statsTelemetrySlice.actions;

export const selectTelemetryData = (state: RootState) =>
  state.statsTelemetry.telemetryData;
export const selectTelemetryChartData = (state: RootState) =>
  state.statsTelemetry.telemetryChartData;

export default statsTelemetrySlice.reducer;
