import { OrmlTokensAccountData } from "@acala-network/types/interfaces/types-lookup";
import { ApiPromise } from "@polkadot/api";
import { UnsubscribePromise } from "@polkadot/api-base/types/base";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";

export const fetchBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<BigNumber> => {
  try {
    const uAccount = api.createType("AccountId32", accountId);
    // @ts-ignore
    const balance = await api.rpc.assets.balanceOf(assetId, uAccount);

    return fromChainIdUnit(new BigNumber(balance.toString()));
  } catch (err) {
    console.log(err);
    return new BigNumber(0);
  }
};

export const subscribePicassoBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string,
  callback: (balance: BigNumber) => void
) => {
  const uAccount = api.createType("AccountId32", accountId);
  let unsubscribe = () => {};
  try {
    unsubscribe = await api.query.tokens.accounts(
      uAccount,
      api.createType("u128", assetId),
      (balance: OrmlTokensAccountData) => {
        callback(fromChainIdUnit(new BigNumber(balance.free.toString())));
      }
    );
  } catch (err) {
    console.log(err);
    callback(new BigNumber(0));
  }

  return unsubscribe;
};

export async function subscribeKaruraBalance(
  api: ApiPromise,
  accountId: string,
  assetId: string,
  callback: (balance: BigNumber) => void
): Promise<() => void> {
  let unsub: UnsubscribePromise = new Promise(() => {});
  try {
    const uAccount = api.createType("AccountId32", accountId);
    // @ts-ignore
    unsub = await api.query.tokens.accounts(
      uAccount,
      api.createType("AcalaPrimitivesCurrencyCurrencyId", {
        token: api.createType("AcalaPrimitivesCurrencyTokenSymbol", assetId),
      }),
      (result: OrmlTokensAccountData) => {
        const { free } = result.toJSON() as any;
        const balance = fromChainIdUnit(new BigNumber(free.toString()));
        callback(balance);
      }
    );
  } catch (error) {
    callback(new BigNumber(0));
    console.error(error);
  }
  return unsub;
}
