import { MockedAsset } from "@/store/assets/assets.types";
import BigNumber from "bignumber.js";

export type BondPrincipalAsset = {
  lpPrincipalAsset:
    | {
        baseAsset: MockedAsset | undefined;
        quoteAsset: MockedAsset | undefined;
      };
  simplePrincipalAsset: MockedAsset | undefined;
};

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
  principalAsset: BondPrincipalAsset;
  bondPriceInUSD: BigNumber;
  roi: BigNumber;
  totalPurchasedInUSD: BigNumber;
}