import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { OrmlTokensAccountData } from "@acala-network/types/interfaces/types-lookup";
import { ApiPromise } from "@polkadot/api";
import { UnsubscribePromise } from "@polkadot/api-base/types/base";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";

export async function subscribeKaruraBalance(
  api: ApiPromise,
  accountId: string,
  tokenMetadata: TokenMetadata,
  callback: (balance: BigNumber) => void
): Promise<() => void> {
  let unsub: UnsubscribePromise = new Promise(() => {});
  try {
    if (!tokenMetadata.karuraId) throw new Error("Unsupported assets");
    const uAccount = api.createType("AccountId32", accountId);
    // @ts-ignore
    unsub = await api.query.tokens.accounts(
      uAccount,
      api.createType("AcalaPrimitivesCurrencyCurrencyId", {
        token: api.createType("AcalaPrimitivesCurrencyTokenSymbol", tokenMetadata.karuraId),
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