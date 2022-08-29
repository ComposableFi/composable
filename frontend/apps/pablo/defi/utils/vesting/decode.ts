import { VestingSchedule } from "@/defi/types";
import { fromChainUnits } from "@/defi/utils";
import BigNumber from "bignumber.js";

export function decodeVestingSchedule(vestingSchedule: any): VestingSchedule {
  const type = vestingSchedule.window.blockNumberBased ? "block" : "moment";
  const window = {
    start: vestingSchedule.window.blockNumberBased
      ? new BigNumber(vestingSchedule.window.blockNumberBased.start)
      : new BigNumber(vestingSchedule.window.momentBased.start),
    period: vestingSchedule.window.blockNumberBased
      ? new BigNumber(vestingSchedule.window.blockNumberBased.period)
      : new BigNumber(vestingSchedule.window.momentBased.period),
  };
  return {
    perPeriod: fromChainUnits(vestingSchedule.perPeriod),
    periodCount: Number(vestingSchedule.periodCount),
    window,
    type,
    alreadyClaimed: fromChainUnits(vestingSchedule.alreadyClaimed),
    vestingScheduleId: new BigNumber(vestingSchedule.vestingScheduleId)
  };
}
