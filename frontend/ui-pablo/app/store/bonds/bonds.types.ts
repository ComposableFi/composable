import BigNumber from "bignumber.js";
import { Token } from "../../defi/types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";

export interface BondSlice {
  allBonds: AllBond[];
  activeBonds: ActiveBond[];
  addActiveBond: (
    bondOffer: BondOffer,
    vestingSchedule: VestingSchedule,
    currentBlock: number,
    currentTime: number
  ) => void;
  addBond: (
    bondOffer: BondOffer,
    principalAppoloPriceInUSD: number,
    rewardAppoloPriceInUSD: number
  ) => void;
  reset: () => void;
}

export interface BondOffer {
  offerId: number;
  beneficiary: AccountId32;
  currencyId: number;
  asset: Token | { base: Token; quote: Token };
  bondPrice: BigNumber;
  nbOfBonds: number;
  maturity: number | "Infinite";
  reward: OfferReward;
}

export interface VestingSchedule {
  perPeriod: BigNumber;
  periodCount: number;
  window: Window;
  type: "block" | "moment";
}

type Window = { start: number; period: number };

interface OfferReward {
  currencyId: number;
  asset: Token;
  amount: BigNumber;
  maturity: number;
}

type ActiveBond = {
  offerId: number;
  asset: BondOffer["asset"];
  pendingAmount: BigNumber;
  claimableAmount: BigNumber;
  vestingTime: string;
  bondOffer: BondOffer;
};

type AllBond = {
  offerId: number;
  asset: BondOffer["asset"];
  price: BigNumber;
  roi: BigNumber;
  totalPurchased: BigNumber;
  bondOffer: BondOffer;
};

export interface ISupplySummary {
  principalAsset: BondOffer["asset"];
  rewardAsset: OfferReward["asset"];
  marketPriceInUSD: () => Promise<number>;
  bondPriceInUSD: () => Promise<number>;
  roi: number;
  vestingPeriod: string;
}

export interface IDepositSummary {
  principalAsset: BondOffer["asset"];
  userBalance: () => Promise<string>;
  purchasableTokens: () => Promise<string>;
  rewardableTokens: (amount: number) => string;
  roi: BigNumber;
  vestingPeriod: string;
}
