import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "../../types";
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

export interface StatsTelemetrySlice {
  statsTelemetry: TelemetryState & {
    setFinalizedBlock: (data: TelemetryData["data"][0]) => void;
    setAverageTime: (data: TelemetryData["data"][1]) => void;
    setLastBlock: (data: TelemetryData["data"][2]) => void;
    setMemPoolInterval: (data: number) => void;
  };
}

export const createStatsTelemetrySlice: StoreSlice<StatsTelemetrySlice> = (
  set: NamedSet<StatsTelemetrySlice>
) => ({
  statsTelemetry: {
    ...initialState,
    setFinalizedBlock: (data: TelemetryData["data"][0]) => {
      set((state) => {
        state.statsTelemetry.telemetryData.data[0] = data;

        return state;
      });
    },
    setAverageTime: (data: TelemetryData["data"][1]) => {
      set((state) => {
        state.statsTelemetry.telemetryData.data[1] = data;

        return state;
      });
    },
    setLastBlock: (data: TelemetryData["data"][2]) => {
      set((state) => {
        state.statsTelemetry.telemetryData.data[2] = data;

        return state;
      });
    },
    setMemPoolInterval: (data: number) => {
      set((state) => {
        state.statsTelemetry.telemetryChartData.data[0].data.pickedInterval =
          data;

        return state;
      });
    },
  },
});
