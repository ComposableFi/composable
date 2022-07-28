import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { Bool, u128 } from "@polkadot/types-codec";
import { Balance } from "@polkadot/types/interfaces/runtime";
import { IEvent } from "@polkadot/types/types";
import { AccountId32 } from "@polkadot/types/interfaces";
import BN from "bn.js";

/**
 * Creates a constant product (Uniswap) dex pool.
 * @param api Connected API client.
 * @param senderWallet The wallet to send the transaction from.
 * @param poolId
 * @param baseAmount
 * @param quoteAmount
 * @param minMintAmount
 * @param keepAlive
 */
export default async function (
  api: ApiPromise,
  senderWallet: KeyringPair,
  poolId: number | u128 | BN,
  baseAmount: number | u128 | Balance | bigint | BN,
  quoteAmount: number | u128 | Balance | bigint | BN,
  minMintAmount: number | u128 | Balance | bigint | BN,
  keepAlive: Bool | boolean
): Promise<IEvent<[AccountId32, u128, u128, u128, u128]>> {
  return await sendAndWaitForSuccess(
    api,
    senderWallet,
    api.events.pablo.LiquidityAdded.is,
    api.tx.pablo.addLiquidity(poolId, baseAmount, quoteAmount, minMintAmount, keepAlive)
  );
}
