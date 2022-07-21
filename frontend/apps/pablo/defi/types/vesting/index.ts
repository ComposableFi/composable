import BigNumber from "bignumber.js";

type Window = { start: BigNumber; period: BigNumber };
export interface VestingSchedule {
  perPeriod: BigNumber;
  periodCount: number;
  window: Window;
  type: "block" | "moment";
}