import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { Asset } from "./Asset";

export class Apollo {
  protected __api: ApiPromise;

  constructor(api: ApiPromise) {
    this.__api = api;
  }

  public async getPrice(...assets: Asset[]): Promise<Map<string, BigNumber>> {
    let priceMap: Map<string, BigNumber> = new Map();
    try {
      for (const asset of assets) {
        const assetId: string = asset.getPicassoAssetId() as string;
        let prices = await this.__api.query.oracle.prices(assetId);
        const decoded = prices.toJSON();
        // @ts-ignore
        priceMap.set(assetId, decoded.price);
      }
      return priceMap;
    } catch (err: any) {
      console.error("[Apollo.getPrice] ", err.message);
      return Promise.reject(err.message);
    }
  }
}
