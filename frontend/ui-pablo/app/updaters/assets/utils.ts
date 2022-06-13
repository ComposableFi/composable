import { fromChainUnits } from "@/utils/bignumber";
import { ApiPromise } from "@polkadot/api";

export const fetchBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<string> => {
  try {
    const balance = await (api.rpc as any).assets.balanceOf(
      api.createType("CurrencyId", assetId),
      api.createType("AccountId32", accountId)
    );
    return fromChainUnits(balance).toString();
  } catch (err: any) {
    return "0";
  }
};

export const fetchApolloPriceByAssetId = async (
  api: ApiPromise,
  assetId: string
): Promise<string> => {
  try {
    let data = await api.query.oracle.prices(assetId);
    const decoded: any = data.toJSON();
    console.log('Oracle Price: ', decoded)
    return decoded.price;
  } catch (err: any) {
    return "0";
  }
};
