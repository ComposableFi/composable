import { BondOffer } from "@/defi/types";
import BigNumber from "bignumber.js";
import { Token } from "../../defi/types";

export interface BondSlice {
  bondOffers: {
    list: BondOffer[];
    totalPurchased: Record<string, BigNumber>;
  };
  puttotalPurchased: (totalPurchasedBonds: Record<string, BigNumber>) => void;
  putBondOffers: (bondsOffers: BondOffer[]) => void;
  putBondOffer: (bondsOffers: BondOffer) => void;
  reset: () => void;
}


interface OfferReward {
  currencyId: number;
  asset: Token;
  amount: BigNumber;
  maturity: number;
}

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
  nbOfBonds: (amount: number) => number;
  userBalance: () => Promise<string>;
  purchasableTokens: () => Promise<string>;
  rewardableTokens: (amount: number) => string;
  roi: BigNumber;
  vestingPeriod: string;
}
