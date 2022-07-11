import { BondOffer } from "@/defi/types";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import { useMemo } from "react";

export default function useTotalPurchasedBondOffer(bondOffer: BondOffer): BigNumber {
    const {
        bondOffers: {
            totalPurchased
        },
        apollo
    } = useStore();

    return useMemo(() => {
        if (apollo[bondOffer.asset] && totalPurchased[bondOffer.offerId.toString()]) {
            return totalPurchased[bondOffer.offerId.toString()].times(apollo[bondOffer.asset]).times(bondOffer.bondPrice)
        }
        return new BigNumber(0)
    }, [bondOffer, totalPurchased, apollo])
}