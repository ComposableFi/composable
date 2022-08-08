import { fromChainUnits } from "@/defi/utils";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export const fetchBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<string> => {
  try {
    // @ts-ignore
    const balance = await api.rpc.assets.balanceOf(
      api.createType("CurrencyId", assetId),
      api.createType("AccountId32", accountId)
    );
    return fromChainUnits(balance).toString();
  } catch (err: any) {
    return "0";
  }
};

export const fetchAssetBalance = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<BigNumber> => {
  try {
    // @ts-ignore
    const balance = await api.rpc.assets.balanceOf(
      api.createType("CurrencyId", assetId),
      api.createType("AccountId32", accountId)
    );
    return fromChainUnits(balance);
  } catch (err: any) {
    console.error(err);
    return new BigNumber(0);
  }
};