import { OrmlTokensAccountData } from "@acala-network/types/interfaces/types-lookup";
import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";

export const subscribePicassoBalanceByAssetId = async (
  api: ApiPromise,
  accountId: string,
  onChainId: BigNumber | undefined,
  decimals: number,
  callback: (accountData: {
    locked: BigNumber;
    free: BigNumber;
  }) => void
) => {
  const uAccount = api.createType("AccountId32", accountId);
  let unsubscribe: () => void = () => {};
  try {
    if (!onChainId) {
      return new Promise(unsubscribe);
    }

    console.log('Subscribing for ', onChainId.toString(), accountId)
    unsubscribe = await api.query.tokens.accounts(
      uAccount,
      api.createType("u128", onChainId.toString()),
      (acocuntData: OrmlTokensAccountData) => {
        callback(
          {
            locked: fromChainIdUnit(BigInt(acocuntData.reserved.toString()), decimals),
            free: fromChainIdUnit(BigInt(acocuntData.free.toString()), decimals)
          }
        );
      }
    );
  } catch (err) {
    console.log(err);
    callback({
      locked: fromChainIdUnit(0),
      free: fromChainIdUnit(0)
    });
  }

  return unsubscribe;
};
