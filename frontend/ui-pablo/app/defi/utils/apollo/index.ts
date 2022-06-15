import { ApiPromise } from "@polkadot/api";

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
