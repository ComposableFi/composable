import { getAssetById } from "@/defi/polkadot/Assets";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { u128 } from "@polkadot/types-codec";

export const fetchBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<string> => {
  try {
    const uAssetId = api.createType("CurrencyId", assetId);
    const uAccount = api.createType("AccountId32", accountId);
    const balance = await api.rpc.assets.balanceOf(uAssetId, uAccount);

    console.log({ balance: balance.toString(), assetId });

    return new BigNumber(balance.toString()).div(10 ** 12).toFixed(4);
  } catch (err: any) {
    console.log(err);
    return "0";
  }
};
