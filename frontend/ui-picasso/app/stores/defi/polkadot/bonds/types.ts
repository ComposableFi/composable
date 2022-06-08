import { Token } from "@/defi/Tokens";
import BigNumber from "bignumber.js";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";

interface OfferReward {
  asset: Token;
  amount: BigNumber;
  maturity: BigNumber;
}

export interface BondOffer {
  beneficiary: AccountId32;
  asset: Token;
  bondPrice: BigNumber;
  nbOfBonds: BigNumber;
  maturity: number | "Infinite";
  reward: OfferReward;
  rewardPrice: BigNumber; // from Oracle
  price: BigNumber; // from Oracle
}
