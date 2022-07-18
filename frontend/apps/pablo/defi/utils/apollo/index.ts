import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export const fetchApolloPriceByAssetId = async (
  api: ApiPromise,
  assetId: string
): Promise<string> => {
  try {
    let data = await api.query.oracle.prices(assetId);
    const decoded: any = data.toJSON();
    return decoded.price;
  } catch (err: any) {
    return "0";
  }
};

export const fetchApolloPriceByAssetIds = async (
  api: ApiPromise,
  assetIds: string[]
): Promise<Record<string, BigNumber>> => {
  return assetIds.reduce(async (recordPromise, currentAssetId) => {
    const record = await recordPromise;
    let price = new BigNumber(0);
    try {
      price = new BigNumber(
        await fetchApolloPriceByAssetId(api, currentAssetId)
      );
    } catch (err) {
      console.error(
        `Error fetching price assetId: ${currentAssetId}, Error: ${err}`
      );
    } finally {
      record[currentAssetId] = price;
    }
    return record;
  }, Promise.resolve({}) as Promise<Record<string, BigNumber>>);
};
