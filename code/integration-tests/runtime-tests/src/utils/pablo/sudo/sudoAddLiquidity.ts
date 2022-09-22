import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { Bool, Null, Result, u128 } from "@polkadot/types-codec";
import { Balance } from "@polkadot/types/interfaces/runtime";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";
import { IEvent } from "@polkadot/types/types";
import BN from "bn.js";
import { expect } from "chai";

/**
 * Creates a constant product (Uniswap) dex pool.
 * @param api Connected API client.
 * @param sudoKey
 * @param poolId
 * @param baseAmount
 * @param quoteAmount
 * @param minMintAmount
 * @param keepAlive
 */
export default async function(
  api: ApiPromise,
  sudoKey: KeyringPair,
  poolId: number | u128 | BN,
  baseAmount: number | u128 | Balance | bigint,
  quoteAmount: number | u128 | Balance | bigint,
  minMintAmount: number | u128 | Balance,
  keepAlive: Bool | boolean
): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>> {
  const result = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.pablo.addLiquidity(poolId, baseAmount, quoteAmount, minMintAmount, keepAlive))
  );
  expect(result.data[0].isOk).to.be.true;
  return result;
}
