import { BondOffer } from "@/defi/types";
import { useMemo } from "react";
import {
  AVERAGE_BLOCK_TIME,
  calculateVestingTime,
} from "@/defi/utils";
import { useBlockInterval } from "../useBlockInterval";
import BigNumber from "bignumber.js";

export default function useBondVestingSchedule(
  bondOffer: BondOffer | undefined
): string {
  const averageBlockTime = useBlockInterval();

  const vestingTime = useMemo(() => {
    if (bondOffer) {
      if (bondOffer.maturity === "Infinite") {
        return bondOffer.maturity;
      }

      return calculateVestingTime(
        bondOffer.maturity.Finite.returnIn,
        averageBlockTime
          ? new BigNumber(averageBlockTime.toString())
          : new BigNumber(AVERAGE_BLOCK_TIME)
      );
    }

    return "00:00:00";
  }, [averageBlockTime, bondOffer]);

  return vestingTime;
}
