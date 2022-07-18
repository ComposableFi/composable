import { BondOffer } from "@/defi/types";
import BigNumber from "bignumber.js";
import produce from "immer";
import { BondSlice } from "./bonds.types";

export const putBondOffer = (
  bondOffersState: BondSlice["bondOffers"],
  bondOffer: BondOffer,
) => {
  return produce(bondOffersState, (draft) => {
    draft.list = bondOffersState.list.map(offer => {
      if (offer.offerId === bondOffer.offerId) {
        return bondOffer
      }
      return offer;
    })
  })
}

export const putBondOffers = (
  bondOffersState: BondSlice["bondOffers"],
  bondOffers: BondOffer[],
) => {
  return produce(bondOffersState, (draft) => {
    draft.list = [...bondOffers]
  })
}

export const putBondOfferTotalPurchased = (
  bondOffersState: BondSlice["bondOffers"],
  totalPurchasedBonds: Record<string, BigNumber>
) => {
  return produce(bondOffersState, (draft) => {
    draft.totalPurchased = totalPurchasedBonds;
  })
}

export const putBondOfferROI = (
  bondOffersState: BondSlice["bondOffers"],
  roi: Record<string, BigNumber>
) => {
  return produce(bondOffersState, (draft) => {
    draft.roi = roi;
  })
}