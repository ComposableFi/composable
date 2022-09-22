import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { Bool, u128 } from "@polkadot/types-codec";
import { Balance } from "@polkadot/types/interfaces/runtime";
import { IEvent } from "@polkadot/types/types";
import { AccountId32 } from "@polkadot/types/interfaces";
import BN from "bn.js";
import { ComposableTraitsDexFee } from "@composable/types/interfaces";

/**
 * Creates a constant product (Uniswap) dex pool.
 * @param api Connected API client.
 * @param senderWallet The wallet to send the transaction from.
 * @param poolId
 * @param lpAmount
 * @param minBaseAmount
 * @param minQuoteAmount
 */
export default async function(
  api: ApiPromise,
  senderWallet: KeyringPair,
  poolId: number | u128 | BN,
  assetId: number | u128 | Balance | BN | bigint,
  amount: number | u128 | Balance | BN | bigint,
  minReceive: number | u128 | Balance | BN | bigint,
  keepAlive: boolean | Bool
): Promise<IEvent<[u128, AccountId32, u128, u128, u128, u128, ComposableTraitsDexFee]>> {
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.pablo.Swapped.is,
    api.tx.pablo.buy(poolId, assetId, amount, minReceive, keepAlive)
  );
}
