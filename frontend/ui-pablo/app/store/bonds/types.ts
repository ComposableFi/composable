import BigNumber from "bignumber.js";
import { Token } from "../../defi/types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";

export interface BondSlice {
  allBonds: AllBond[];
  activeBonds: ActiveBond[];
  setActiveBonds: (bondOffer: BondOffer) => void;
  setAllBonds: (bondOffer: BondOffer) => void;
}

export interface BondOffer {
  beneficiary: AccountId32;
  asset: Token;
  bondPrice: BigNumber;
  nbOfBonds: BigNumber;
  maturity: number | "Infinite";
  reward: OfferReward;
}

interface OfferReward {
  asset: Token;
  amount: BigNumber;
  maturity: BigNumber;
}

type Asset = { token1: Token; token2: Token };

type ActiveBond = {
  assetPair: Asset;
  pending_amount: BigNumber;
  claimable_amount: BigNumber;
  vesting_time: number;
};

type AllBond = {
  assetPair: Asset;
  price: BigNumber;
  roi: BigNumber;
  totalPurchased: BigNumber;
};
