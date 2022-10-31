import { OrmlTokensAccountData } from "@acala-network/types/interfaces/types-lookup";
import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";

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
  tokenMetadata: TokenMetadata,
  callback: (balance: BigNumber) => void
) => {
  const uAccount = api.createType("AccountId32", accountId);
  let unsubscribe = () => {};
  try {
    if (!tokenMetadata.picassoId) throw new Error('Unsupported Token');
    unsubscribe = await api.query.tokens.accounts(
      uAccount,
      api.createType("u128", tokenMetadata.picassoId.toString()),
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
