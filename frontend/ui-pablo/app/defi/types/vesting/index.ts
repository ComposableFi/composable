import BigNumber from "bignumber.js";

type Window = { start: number; period: number };
export interface VestingSchedule {
  perPeriod: BigNumber;
  periodCount: number;
  window: Window;
  type: "block" | "moment";
}