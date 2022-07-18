import { BondOffer } from "@/defi/types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useMemo } from "react";

export default function useBondPrice(bondOffer: BondOffer): BigNumber {
  const { apollo } = useStore();

  return useMemo(() => {
    if (apollo[bondOffer.asset]) {
      return bondOffer.bondPrice.times(apollo[bondOffer.asset]);
    }
    return new BigNumber(0);
  }, [bondOffer, apollo]);
}
