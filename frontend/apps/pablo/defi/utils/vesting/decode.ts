import { VestingSchedule } from "@/defi/types";
import { fromChainUnits } from "@/defi/utils";
import { humanizedBnToBn, stringToNumber } from "shared";

export function decodeVestingSchedule(vestingSchedule: any): VestingSchedule {
  const type = vestingSchedule.window.blockNumberBased ? "block" : "moment";
  const window = {
    start: vestingSchedule.window.blockNumberBased
      ? humanizedBnToBn(vestingSchedule.window.blockNumberBased.start)
      : humanizedBnToBn(vestingSchedule.window.momentBased.start),
    period: vestingSchedule.window.blockNumberBased
      ? humanizedBnToBn(vestingSchedule.window.blockNumberBased.period)
      : humanizedBnToBn(vestingSchedule.window.momentBased.period),
  };
  return {
    perPeriod: fromChainUnits(vestingSchedule.perPeriod.replaceAll(",", "")),
    periodCount: Number(vestingSchedule.periodCount),
    window,
    type,
  };
}
