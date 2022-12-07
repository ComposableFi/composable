import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "../unit";

type OfferMaturity = { finite: { returnIn: BigNumber } } | "Infinite";

export class BondOfferReward {
  protected readonly __asset: BigNumber;
  protected readonly __maturity: BigNumber;
  protected readonly __amount: BigNumber;

  static fromJSON(reward: any): BondOfferReward {
    try {
      const amount = new BigNumber(reward.amount);
      const asset = new BigNumber(reward.asset);
      const maturity = fromChainIdUnit(reward.maturity);

      return new BondOfferReward(amount, asset, maturity);
    } catch (err: any) {
      console.error("[BondOffer] ", err);
      throw new Error(err.message);
    }
  }

  constructor(amount: BigNumber, asset: BigNumber, maturity: BigNumber) {
    this.__amount = amount;
    this.__asset = asset;
    this.__maturity = maturity;
  }

  getAssetId(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__asset : this.__asset.toString();
  }

  getAmount(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__amount : this.__amount.toString();
  }

  getMaturity(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__maturity : this.__maturity.toString();
  }
}

export class BondOffer {
  protected readonly __offerId: BigNumber;
  protected readonly __asset: BigNumber;
  protected readonly __beneficiary: string;
  protected readonly __bondPrice: BigNumber;
  protected readonly __nbOfBonds: BigNumber;
  protected readonly __maturity: OfferMaturity;
  protected readonly __reward: BondOfferReward;

  static async fetchBondOffer(
    api: ApiPromise,
    index: number
  ): Promise<BondOffer | null> {
      let bondOffer: BondOffer | null = null;
      try {
        let offer = await api.query.bondedFinance.bondOffers(index);
        const [beneficiary, _offer] = offer.toJSON() as any;
        bondOffer = BondOffer.fromJSON(
          index,
          beneficiary,
          _offer
        );
      } catch (err) {
        console.error(err);
      } finally {
        return bondOffer;
      }
  }

  static async fetchBondOffers(parachainApi: ApiPromise): Promise<BondOffer[]> {
    try {
      const bondOfferCount = await parachainApi.query.bondedFinance.bondOfferCount();
      let offerPromises = [];
      for (let i = 1; i <= bondOfferCount.toNumber(); i++) {
        offerPromises.push(BondOffer.fetchBondOffer(parachainApi, i));
      }
      let bonds = await Promise.all(offerPromises);
      return bonds.filter(bond => !!bond) as BondOffer[];
    } catch (ex) {
      console.error(ex);
      return [];
    }
  }

  static fromJSON(index: number, beneficiary: string, offer: any): BondOffer {
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

      return new BondOffer(
        offerId,
        asset,
        beneficiary,
        bondPrice,
        nbOfBonds,
        maturity,
        reward
      );
    } catch (err: any) {
      console.error("[BondOffer] ", err);
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

  getBondPrice(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__bondPrice : this.__bondPrice.toString();
  }

  getBondOfferId(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__offerId : this.__offerId.toString();
  }

  getBondedAssetId(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__asset : this.__asset.toString();
  }

  getRewardAssetId(inBn: boolean = false): BigNumber | string {
    return this.__reward.getAssetId(inBn);
  }

  getRewardAssetAmount(inBn: boolean = false): BigNumber | string {
    return this.__reward.getAmount(inBn);
  }
  
  getRewardAssetMaturity(inBn: boolean = false): BigNumber | string {
    return this.__reward.getMaturity(inBn);
  }

  getNumberOfBonds(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__nbOfBonds : this.__nbOfBonds.toString();
  }
}
