import { VestingSchedule } from "../../store/bonds/types";
import BigNumber from "bignumber.js";

export const stringToBigNumber = (value: string): BigNumber =>
  new BigNumber(value.replaceAll(",", ""));

export function decodeVestingSchedule(vestingSchedule: any): VestingSchedule {
  const type = vestingSchedule.window.BlockNumberBased ? "block" : "moment";
  const window = {
    start: vestingSchedule.window.BlockNumberBased
      ? stringToBigNumber(vestingSchedule.window.BlockNumberBased.start)
      : stringToBigNumber(vestingSchedule.window.MomentBased.start),
    period: vestingSchedule.window.BlockNumberBased
      ? stringToBigNumber(vestingSchedule.window.BlockNumberBased.period)
      : stringToBigNumber(vestingSchedule.window.MomentBased.period),
  };

  return {
    perPeriod: stringToBigNumber(vestingSchedule.perPeriod),
    periodCount: stringToBigNumber(vestingSchedule.periodCount),
    window,
    type,
  };
}
