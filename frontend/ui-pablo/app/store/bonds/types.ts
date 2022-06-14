import BigNumber from "bignumber.js";
import { Token } from "../../defi/types";
import type { AccountId32 } from "@polkadot/types/interfaces/runtime";

export interface BondSlice {
  allBonds: AllBond[];
  activeBonds: ActiveBond[];
  addActiveBond: (
    bondOffer: BondOffer,
    vestingSchedule: VestingSchedule,
    currentBlock: BigNumber,
    currentTime: BigNumber
  ) => void;
  addBond: (
    bondOffer: BondOffer,
    assetPrice: number,
    rewardPrice: number
  ) => void;
  reset: () => void;
}

export interface BondOffer {
  beneficiary: AccountId32;
  asset: Token;
  bondPrice: BigNumber;
  nbOfBonds: BigNumber;
  maturity: number | "Infinite";
  reward: OfferReward;
}

export interface VestingSchedule {
  perPeriod: BigNumber;
  periodCount: BigNumber;
  window: Window;
  type: "block" | "moment";
}

type Window = { start: BigNumber; period: BigNumber };

interface OfferReward {
  asset: Token;
  amount: BigNumber;
  maturity: BigNumber;
}

type ActiveBond = {
  asset: Token;
  pendingAmount: BigNumber;
  claimableAmount: BigNumber;
  vestingTime: string;
};

type AllBond = {
  asset: Token;
  price: BigNumber;
  roi: BigNumber;
  totalPurchased: BigNumber;
};
