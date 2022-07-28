import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { Null, Result, u128 } from "@polkadot/types-codec";
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
 * @param lpAmount
 * @param minBaseAmount
 * @param minQuoteAmount
 */
export default async function(
  api: ApiPromise,
  sudoKey: KeyringPair,
  poolId: number | u128 | BN,
  lpAmount: number | u128 | Balance | BN | bigint,
  minBaseAmount: number | u128 | Balance | BN | bigint,
  minQuoteAmount: number | u128 | Balance | BN | bigint
): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>> {
  const result = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.pablo.removeLiquidity(poolId, lpAmount, minBaseAmount, minQuoteAmount))
  );
  expect(result.data[0].isOk).to.be.true;
  return result;
}
