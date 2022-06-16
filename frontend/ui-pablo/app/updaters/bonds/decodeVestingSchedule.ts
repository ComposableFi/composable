import { VestingSchedule } from "../../store/bonds/bonds.types";
import { stringToBigNumber } from "../../utils/stringToBigNumber";
import { stringToNumber } from "../../utils/stringToNumber";

export function decodeVestingSchedule(vestingSchedule: any): VestingSchedule {
  const type = vestingSchedule.window.BlockNumberBased ? "block" : "moment";
  const window = {
    start: vestingSchedule.window.BlockNumberBased
      ? stringToNumber(vestingSchedule.window.BlockNumberBased.start)
      : stringToNumber(vestingSchedule.window.MomentBased.start),
    period: vestingSchedule.window.BlockNumberBased
      ? stringToNumber(vestingSchedule.window.BlockNumberBased.period)
      : stringToNumber(vestingSchedule.window.MomentBased.period),
  };
  return {
    perPeriod: stringToBigNumber(vestingSchedule.perPeriod),
    periodCount: Number(vestingSchedule.periodCount),
    window,
    type,
  };
}
