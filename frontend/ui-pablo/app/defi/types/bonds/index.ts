import { MockedAsset } from "@/store/assets/assets.types";
import BigNumber from "bignumber.js";

export interface BondOffer {
  offerId: BigNumber;
  asset: string;
  beneficiary: string;
  bondPrice: BigNumber;
  maturity: { Finite: { returnIn: BigNumber } } | "Infinite";
  nbOfBonds: BigNumber;
  reward: {
    amount: BigNumber;
    asset: string;
    maturity: BigNumber;
  };
}

export interface OfferRow {
  offerId: BigNumber;
  principalAsset: {
    baseAsset: MockedAsset | undefined;
    quoteAsset: MockedAsset | undefined;
  } | MockedAsset | undefined;
  bondPrice: BigNumber;
  roi: BigNumber;
  totalPurchased: BigNumber;
}