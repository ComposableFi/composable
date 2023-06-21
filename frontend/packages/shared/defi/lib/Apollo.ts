import { ApiPromise } from "@polkadot/api";
import { Asset } from "./assets/Asset";
import BigNumber from "bignumber.js";

export class Apollo {
  protected __api: ApiPromise;

  constructor(api: ApiPromise) {
    this.__api = api;
  }

  /**
   * TODO: Asses whether
   * this class is needed or not
   * @param assets
   * @returns {Promise<Record<string, BigNumber>>}
   */
  public async getPrice(assets: Asset[]): Promise<Record<string, BigNumber>> {
    let priceMap: Record<string, BigNumber> = {};
    try {
      for (const asset of assets) {
        const assetId: string = asset.getPicassoAssetId() as string;
        let prices = await this.__api.query.oracle.prices(assetId);
        // @ts-ignore
        const price = new BigNumber(prices.price.toString());
        // @ts-ignore
        priceMap[assetId] = price;
      }
      return priceMap;
    } catch (err: any) {
      console.error("[Apollo.getPrice] ", err.message);
      return Promise.reject(err.message);
    }
  }
}
