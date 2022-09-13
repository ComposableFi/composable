import { StoreSlice } from "../../types";
import BigNumber from "bignumber.js";

const MAX_AVERAGE_TIME_LIST_LENGTH = 200;

interface TelemetryData {
  finalizedBlock: BigNumber;
  lastBlock: BigNumber;
  averageTime: Array<BigNumber>;
}

interface TelemetryState {
  telemetryData: TelemetryData;
}

const initialState: TelemetryState = {
  telemetryData: {
    finalizedBlock: new BigNumber(0),
    lastBlock: new BigNumber(0),
    averageTime: [],
  },
};

export interface StatsTelemetrySlice {
  statsTelemetry: TelemetryState & {
    setFinalizedBlock: (data: BigNumber) => void;
    pushAverageTime: (data: BigNumber) => void;
    setLastBlock: (data: BigNumber) => void;
    getBlockAverage: () => BigNumber;
  };
}

export const createStatsTelemetrySlice: StoreSlice<StatsTelemetrySlice> = (
  set,
  get
) => ({
  statsTelemetry: {
    ...initialState,
    setFinalizedBlock: (data: BigNumber) => {
      set((state) => {
        state.statsTelemetry.telemetryData.finalizedBlock = data;

        return state;
      });
    },
    pushAverageTime: (data: BigNumber) => {
      set((state) => {
        state.statsTelemetry.telemetryData.averageTime.push(data);
        if (
          state.statsTelemetry.telemetryData.averageTime.length >
          MAX_AVERAGE_TIME_LIST_LENGTH
        ) {
          state.statsTelemetry.telemetryData.averageTime.shift();
        }

        return state;
      });
    },
    setLastBlock: (data: BigNumber) => {
      set((state) => {
        state.statsTelemetry.telemetryData.lastBlock = data;

        return state;
      });
    },
    getBlockAverage: () => {
      const { averageTime } = get().statsTelemetry.telemetryData;
      if (averageTime.length === 0) {
        return new BigNumber(0);
      }
      return averageTime
        .reduce((a, b) => a.plus(b), new BigNumber(0))
        .div(averageTime.length);
    },
  },
});
