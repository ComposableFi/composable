import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { Null, Result, u128, u16 } from "@polkadot/types-codec";
import { Permill } from "@polkadot/types/interfaces/runtime";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";
import { IEvent } from "@polkadot/types/types";
import { AccountId32 } from "@polkadot/types/interfaces";
import { ComposableTraitsDefiCurrencyPairCurrencyId } from "@composable/types/interfaces/index";

/**
 * Creates a constant product (Uniswap) dex pool.
 * @param api Connected API client.
 * @param sudoKey
 * @param managerWallet The wallet managing the pool.
 * @param baseAssetId
 * @param quoteAssetId
 * @param fee
 * @param baseWeight
 */
export default async function(
  api: ApiPromise,
  sudoKey: KeyringPair,
  managerWallet: Uint8Array | AccountId32 | string,
  baseAssetId: number | u128,
  quoteAssetId: number | u128,
  amplificationCoefficient: number | u16,
  fee: number | Permill
): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>> {
  const pool = api.createType("PalletPabloPoolInitConfiguration", {
    StableSwap: {
      owner: api.createType("AccountId32", managerWallet),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", baseAssetId),
        quote: api.createType("u128", quoteAssetId)
      }),
      amplificationCoefficient: api.createType("Permill", amplificationCoefficient),
      fee: api.createType("Permill", fee)
    }
  });
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.pablo.create(pool))
  );
}
