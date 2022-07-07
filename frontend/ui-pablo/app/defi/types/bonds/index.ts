import { BondPrincipalAsset } from "@/defi/hooks/bonds/useBondOffers";
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
  principalAsset: BondPrincipalAsset;
  bondPrice: BigNumber;
  roi: BigNumber;
  totalPurchased: BigNumber;
}