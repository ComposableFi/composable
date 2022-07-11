import { BondOffer } from "@/defi/types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useMemo } from "react";

export default function useBondOfferROI(bondOffer: BondOffer): BigNumber {
  const {
    bondOffers: { roi },
  } = useStore();

  return useMemo(() => {
    const offerId = bondOffer.offerId.toString();
    return roi[offerId] || new BigNumber(0);
  }, [bondOffer, roi]);
}
