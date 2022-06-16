import { getAssetById } from "@/defi/polkadot/Assets";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { u128 } from "@polkadot/types-codec";
import { fromPica } from "./BondedFinance";

export const fetchBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<BigNumber> => {
  try {
    const uAccount = api.createType("AccountId32", accountId);
    const balance = await api.rpc.assets.balanceOf(assetId, uAccount);

    return fromPica(new BigNumber(balance.toString()));
  } catch (err: any) {
    console.log(err);
    return new BigNumber(0);
  }
};
