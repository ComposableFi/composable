import create from "zustand";
import { VestingSchedule } from "@/defi/types";
import { BondOffer } from "shared";
import BigNumber from "bignumber.js";

export interface BondedOfferVestingState {
  netRewards: BigNumber;
  claimable: BigNumber;
  pendingRewards: BigNumber;
  alreadyClaimed: BigNumber;
  milliSecondsSinceVestingStart: BigNumber;
}

export interface BondSlice {
  bondOffers: BondOffer[];
  returnOnInvestmentRecord: Record<string, BigNumber>;
  bondOffersTotalPurchasedCount: Record<string, BigNumber>;
  bondedOfferVestingScheduleIds: Record<string, Set<string>>;
  bondedOfferVestingSchedules: Record<string, VestingSchedule[]>;
  bondedOffersVestingState: Record<string, BondedOfferVestingState>;
}

export const useBondOffersSlice = create<BondSlice>(() => ({
  bondOffers: [],
  returnOnInvestmentRecord: {},
  bondOffersTotalPurchasedCount: {},
  bondedOfferVestingScheduleIds: {},
  bondedOfferVestingSchedules: {},
  bondedOffersVestingState: {},
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

export const putBondedOfferVestingState = (
  vestingState: Record<string, BondedOfferVestingState>
) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondedOffersVestingState: vestingState,
  }));

export const putBondedOfferVestingStateByOfferId = (
  offerId: string,
  vestingState: BondedOfferVestingState
) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondedOffersVestingState: {
      ...state.bondedOffersVestingState,
      [offerId]: vestingState,
    },
  }));

export const resetBondedOfferVestingState = () =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondedOffersVestingState: {},
  }));

export const updateExistingBondOffer = (bondOffer: BondOffer) =>
  useBondOffersSlice.setState((state) => ({
    ...state,
    bondOffers: state.bondOffers.map((offer) =>
      (offer.getBondOfferId(true) as BigNumber).eq(
        bondOffer.getBondOfferId(true) as BigNumber
      ) ? bondOffer : offer
    ),
  }));

export const useBondOfferPriceInAmountOfPrincipalTokens = (offerId: string): BigNumber =>
  useBondOffersSlice().bondOffers.find(
    (offer) => offer.getBondOfferId() as string === offerId
  )?.getBondPrice(true) as BigNumber ?? new BigNumber(0);

export const useBondOfferROI = (offerId: string) =>
  useBondOffersSlice().returnOnInvestmentRecord[offerId] || new BigNumber(0);

export const useBondOfferTotalPurchased = (offerId: string) =>
  useBondOffersSlice().bondOffersTotalPurchasedCount[offerId] ||
  new BigNumber(0);

export const useBondedOfferVestingSchedules = (offerId: string) =>
  useBondOffersSlice().bondedOfferVestingSchedules[offerId] || [];

export const useBondedOfferVestingScheduleIds = (offerId: string) =>
  useBondOffersSlice().bondedOfferVestingScheduleIds[offerId] || [];

export const useBondedOfferVestingState = (offerId: string) =>
  useBondOffersSlice().bondedOffersVestingState[offerId] || {
    alreadyClaimed: new BigNumber(0),
    netRewards: new BigNumber(0),
    claimable: new BigNumber(0),
    pendingRewards: new BigNumber(0),
    milliSecondsSinceVestingStart: new BigNumber(0),
  };
