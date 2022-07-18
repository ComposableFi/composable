import { BondOffer } from "@/defi/types";
import BigNumber from "bignumber.js";

export interface BondSlice {
  bondOffers: {
    list: BondOffer[];
    totalPurchased: Record<string, BigNumber>;
    roi: Record<string, BigNumber>
  };
  setBondOfferTotalPurchased: (totalPurchasedBonds: Record<string, BigNumber>) => void;
  putBondOfferROI: (totalPurchasedBonds: Record<string, BigNumber>) => void;
  setBondOffers: (bondsOffers: BondOffer[]) => void;
  putBondOffer: (bondsOffers: BondOffer) => void;
  reset: () => void;
}