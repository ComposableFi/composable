import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "./BondedFinance";

export const fetchBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<BigNumber> => {
  try {
    const uAccount = api.createType("AccountId32", accountId);
    const balance = await api.rpc.assets.balanceOf(assetId, uAccount);

    return fromChainIdUnit(new BigNumber(balance.toString()));
  } catch (err: any) {
    console.log(err);
    return new BigNumber(0);
  }
};
