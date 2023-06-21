import { BondOffer } from "@/stores/defi/polkadot/bonds/types";
import { useBlockInterval } from "@/defi/polkadot/hooks/useBlockInterval";
import { maturityToSeconds } from "shared";

export function useBondVestingInDays(bondOffer: BondOffer) {
  const interval = useBlockInterval();

  return maturityToSeconds(bondOffer.maturity, interval);
}
