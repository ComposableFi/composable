import { fromChainUnits } from "@/defi/utils";
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