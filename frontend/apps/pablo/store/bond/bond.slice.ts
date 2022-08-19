import create from "zustand";
import { BondOffer, VestingSchedule } from "@/defi/types";
import BigNumber from "bignumber.js";

export interface BondSlice {
  bondOffers: BondOffer[];
  returnOnInvestmentRecord: Record<string, BigNumber>;
  bondOffersTotalPurchasedCount: Record<string, BigNumber>;
  bondedOfferVestingScheduleIds: Record<string, Set<string>>;
  bondedOfferVestingSchedules: Record<string, VestingSchedule[]>;
}

export const useBondOffersSlice = create<BondSlice>(() => ({
  bondOffers: [],
  returnOnInvestmentRecord: {},
  bondOffersTotalPurchasedCount: {},
  bondedOfferVestingScheduleIds: {},
  bondedOfferVestingSchedules: {},
}));

export const putBondOffers = (bondOffers: BondOffer[]) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondOffers,
  }));

export const putBondOffersReturnOnInvestmentRecord = (
  returnOnInvestmentRecord: Record<string, BigNumber>
) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    returnOnInvestmentRecord,
  }));

export const putBondedOfferBondedVestingScheduleIds = (
  bondedOfferVestingScheduleIds: Record<string, Set<string>>
) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondedOfferVestingScheduleIds,
  }));

export const putBondedOfferVestingSchedules = (
  bondedOfferVestingSchedules: Record<string, VestingSchedule[]>
) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondedOfferVestingSchedules,
  }));

export const putBondOffersTotalPurchasedCount = (
  bondOffersTotalPurchasedCount: Record<string, BigNumber>
) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondOffersTotalPurchasedCount,
  }));

export const updateExistingBondOffer = (bondOffer: BondOffer) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondOffers: state.bondOffers.map((offer) =>
      offer.offerId.eq(bondOffer.offerId) ? bondOffer : offer
    ),
  }));

export const useBondOfferPriceInAmountOfPrincipalTokens = (offerId: string) =>
  useBondOffersSlice().bondOffers.find(
    (offer) => offer.offerId.toString() === offerId
  )?.bondPrice ?? new BigNumber(0);

export const useBondOfferROI = (offerId: string) =>
  useBondOffersSlice().returnOnInvestmentRecord[offerId] || new BigNumber(0);

export const useBondOfferTotalPurchased = (offerId: string) =>
  useBondOffersSlice().bondOffersTotalPurchasedCount[offerId] ||
  new BigNumber(0);

export const useBondedOfferVestingSchedules = (offerId: string) =>
  useBondOffersSlice().bondedOfferVestingSchedules[offerId] || [];

export const useBondedOfferVestingScheduleIds = (offerId: string) =>
  useBondOffersSlice().bondedOfferVestingScheduleIds[offerId] || [];
