import { VestingSchedule } from "@/defi/types";
import { fromChainUnits } from "@/defi/utils";
import { stringToNumber } from "shared";

export function decodeVestingSchedule(vestingSchedule: any): VestingSchedule {
  const type = vestingSchedule.window.blockNumberBased ? "block" : "moment";
  const window = {
    start: vestingSchedule.window.blockNumberBased
      ? stringToNumber(vestingSchedule.window.blockNumberBased.start)
      : stringToNumber(vestingSchedule.window.momentBased.start),
    period: vestingSchedule.window.blockNumberBased
      ? stringToNumber(vestingSchedule.window.blockNumberBased.period)
      : stringToNumber(vestingSchedule.window.momentBased.period),
  };
  return {
    perPeriod: fromChainUnits(vestingSchedule.perPeriod.replaceAll(",", "")),
    periodCount: Number(vestingSchedule.periodCount),
    window,
    type,
  };
}
