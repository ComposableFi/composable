import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export async function fetchApolloPriceByAssetId (
  api: ApiPromise,
  assetId: string
): Promise<string> {
  try {
    let data = await api.query.oracle.prices(assetId);
    const decoded: any = data.toJSON();
    return decoded.price;
  } catch (err: any) {
    return "0";
  }
};

export async function fetchApolloPriceByAssetIds (
  api: ApiPromise,
  assetIds: string[]
): Promise<Record<string, BigNumber>> {
  let usdPricesRecord: Record<string, BigNumber> = {};

  for (const assetId of assetIds) {
    let price = new BigNumber(0);
    try {
      const p = await fetchApolloPriceByAssetId(api, assetId);
      price = new BigNumber(p);
    } catch (err) {
      console.error(`Error fetching price assetId: ${assetId}, Error: ${err}`)
    } finally {
      usdPricesRecord[assetId] = price;
    }
  }

  return usdPricesRecord;
}