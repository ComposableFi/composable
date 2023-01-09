import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";
import { OrmlTokensAccountData } from "@acala-network/types/interfaces/types-lookup";
import { ApiPromise } from "@polkadot/api";
import { UnsubscribePromise } from "@polkadot/api-base/types/base";
import BigNumber from "bignumber.js";
import { fromChainIdUnit } from "shared";
import { TokenBalance } from "@/stores/defi/polkadot/balances/slice";

export async function subscribeKaruraBalance(
  api: ApiPromise,
  accountId: string,
  tokenMetadata: TokenMetadata,
  callback: (balance: TokenBalance) => void
): Promise<() => void> {
  let unsub: UnsubscribePromise = new Promise(() => {});
  try {
    if (!tokenMetadata.chainId.karura) {
      return new Promise(() => {});
    }

    const uAccount = api.createType("AccountId32", accountId);
    // @ts-ignore
    unsub = await api.query.tokens.accounts(
      uAccount,
      api.createType("AcalaPrimitivesCurrencyCurrencyId", {
        token: api.createType(
          "AcalaPrimitivesCurrencyTokenSymbol",
          tokenMetadata.chainId.karura
        ),
      }),
      (result: OrmlTokensAccountData) => {
        const { free, reserved } = result.toJSON() as any;
        const bnFree = fromChainIdUnit(new BigNumber(free.toString()));
        const bnLocked = fromChainIdUnit(new BigNumber(reserved.toString()));
        callback({
          free: bnFree,
          locked: bnLocked,
        });
      }
    );
  } catch (error) {
    callback({
      free: new BigNumber(0),
      locked: new BigNumber(0),
    });
    console.error(error);
  }
  return unsub;
}
