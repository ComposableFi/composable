import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { maturityToSeconds } from "shared";
import { useBlockInterval } from "substrate-react";

export function useBondVestingInDays(bondOffer: BondOffer) {
  const interval = useBlockInterval();

  return maturityToSeconds(bondOffer.maturity, interval);
}
