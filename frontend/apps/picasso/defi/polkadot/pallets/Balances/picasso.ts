import { OrmlTokensAccountData } from "@acala-network/types/interfaces/types-lookup";
import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "shared";
import BigNumber from "bignumber.js";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { ParachainNetworks } from "substrate-react";
import { TokenBalance } from "@/stores/defi/polkadot/balances/slice";

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
  callback: (balance: TokenBalance) => void
) => {
  const uAccount = api.createType("AccountId32", accountId);
  let unsubscribe: () => void = () => {};
  try {
    if (!tokenMetadata.chainId.picasso) {
      return new Promise(unsubscribe);
    }

    unsubscribe = await api.query.tokens.accounts(
      uAccount,
      api.createType("u128", tokenMetadata.chainId.picasso.toString()),
      (balance: OrmlTokensAccountData) => {
        callback({
          free: fromChainIdUnit(
            new BigNumber(balance.free.toString()),
            tokenMetadata.decimals.picasso ?? ParachainNetworks.picasso.decimals
          ),
          locked: fromChainIdUnit(
            new BigNumber(balance.reserved.toString()),
            tokenMetadata.decimals.picasso ?? ParachainNetworks.picasso.decimals
          ),
        });
      }
    );
  } catch (err) {
    console.log(err);
    callback({
      free: new BigNumber(0),
      locked: new BigNumber(0),
    });
  }

  return unsubscribe;
};
