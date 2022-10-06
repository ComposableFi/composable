import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "../../unit";

type OfferMaturity = { finite: { returnIn: BigNumber } } | "Infinite";

export class BondOfferReward {
  protected __asset: BigNumber;
  protected __maturity: BigNumber;
  protected __amount: BigNumber;

  static fromJSON(reward: any): BondOfferReward {
    try {
      const amount = new BigNumber(reward.amount);
      const asset = new BigNumber(reward.asset);
      const maturity = fromChainIdUnit(reward.maturity);

      return new BondOfferReward(amount, asset, maturity);
    } catch (err: any) {
      console.error("[BondOffer]", err);
      throw new Error(err.message);
    }
  }

  constructor(amount: BigNumber, asset: BigNumber, maturity: BigNumber) {
    this.__amount = amount;
    this.__asset = asset;
    this.__maturity = maturity;
  }

  // get rewardAssetId(): BigNumber {
  //   return this.__asset;
  // }

  // get rewardAmount(): BigNumber {
  //   return this.__amount;
  // }

  // get maturity(): BigNumber {
  //   return this.__maturity;
  // }
}

export class BondOfferV1 {
  protected __offerId: BigNumber;
  protected __asset: BigNumber;
  protected __beneficiary: string;
  protected __bondPrice: BigNumber;
  protected __nbOfBonds: BigNumber;
  protected __maturity: OfferMaturity;
  protected __reward: BondOfferReward;

  static fromJSON(index: number, beneficiary: string, offer: any): BondOfferV1 {
    try {
      const offerId = new BigNumber(index);
      const asset = new BigNumber(offer.asset);
      const bondPrice = fromChainIdUnit(offer.bondPrice);
      const nbOfBonds = new BigNumber(offer.nbOfBonds);
      const reward = BondOfferReward.fromJSON(offer.reward);

      const maturity = offer.maturity.finite
        ? {
            finite: {
              returnIn: new BigNumber(offer.maturity.finite.returnIn),
            },
          }
        : "Infinite";

      return new BondOfferV1(
        offerId,
        asset,
        beneficiary,
        bondPrice,
        nbOfBonds,
        maturity,
        reward
      );
    } catch (err: any) {
      console.error("[BondOffer]", err);
      throw new Error(err.message);
    }
  }

  constructor(
    offerId: BigNumber,
    asset: BigNumber,
    beneficiary: string,
    bondPrice: BigNumber,
    nbOfBonds: BigNumber,
    maturity: OfferMaturity,
    reward: BondOfferReward
  ) {
    this.__offerId = offerId;
    this.__asset = asset;
    this.__beneficiary = beneficiary;
    this.__bondPrice = bondPrice;
    this.__nbOfBonds = nbOfBonds;
    this.__maturity = maturity;
    this.__reward = reward;
  }

  // get principalAssetId() {
  //   return this.__asset.toString();
  // }

  // get rewardAssetId() {
  //   return this.__reward.rewardAssetId;
  // }

  // get nbOfBonds(): BigNumber {
  //   return this.__nbOfBonds;
  // }
}
