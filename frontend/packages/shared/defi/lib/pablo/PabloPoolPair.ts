import { Asset, humanizedBnToBn } from "shared";
import BigNumber from "bignumber.js";

export class PabloPoolPair {
  base: BigNumber;
  quote: BigNumber;

  static fromJSON(pair: {
    base: string | number;
    quote: string | number;
  }): PabloPoolPair {
    try {
      return new PabloPoolPair(
        humanizedBnToBn(pair.base),
        humanizedBnToBn(pair.quote)
      );
    } catch (err: any) {
      console.error("[PabloPoolPair] ", err.message);
      throw new Error(err.message);
    }
  }

  constructor(base: BigNumber, quote: BigNumber) {
    this.base = base;
    this.quote = quote;
  }

  toJSON(): {
    base: string;
    quote: string;
  } {
    return {
      base: this.base.toString(),
      quote: this.quote.toString(),
    };
  }

  invertJSON(): {
    base: string;
    quote: string;
  } {
    return {
      base: this.quote.toString(),
      quote: this.base.toString(),
    };
  }

  getBaseAsset(): BigNumber {
    return this.base;
  }

  getQuoteAsset(): BigNumber {
    return this.quote;
  }

  intoAssets(assets: Asset[]): Asset[] {
    return assets.filter(
      (asset) =>
        (asset.getPicassoAssetId(true) as BigNumber).eq(this.base) ||
        (asset.getPicassoAssetId(true) as BigNumber).eq(this.quote)
    );
  }
}
