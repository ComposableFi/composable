import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";
import { OrmlTokensAccountData } from "@acala-network/types/interfaces/types-lookup";

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

export const fetchKaruraBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  assetId: string
): Promise<BigNumber> => {
  try {
    const uAccount = api.createType("AccountId32", accountId);
    // @ts-ignore
    const balance = await api.query.tokens.accounts(
      uAccount,
      api.createType("AcalaPrimitivesCurrencyCurrencyId", {
        token: api.createType("AcalaPrimitivesCurrencyTokenSymbol", assetId)
      })
    );

    const { free } = balance.toJSON() as any;

    return fromChainIdUnit(new BigNumber(free.toString()));
  } catch (err) {
    console.log(err);
    return new BigNumber(0);
  }
};
