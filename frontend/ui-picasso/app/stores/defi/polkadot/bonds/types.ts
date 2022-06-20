import { Token } from "@/defi/Tokens";
import BigNumber from "bignumber.js";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";

interface OfferReward {
  asset: Token | Token[];
  amount: BigNumber;
  maturity: BigNumber;
  assetId: string;
}

export interface BondOffer {
  beneficiary: AccountId32;
  asset: Token | Token[];
  assetId: string;
  bondPrice: BigNumber;
  nbOfBonds: BigNumber;
  maturity: number | "Infinite";
  reward: OfferReward;
  rewardPrice: BigNumber; // from Oracle
  price: BigNumber; // from Oracle
  bondOfferId: string;
}
