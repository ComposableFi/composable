import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { Bool, u128, u32 } from "@polkadot/types-codec";
import { AnyNumber } from "@polkadot/types-codec/types";
import { Percent } from "@polkadot/types/interfaces/runtime";

/**
 * Tests tx.oracle.addAssetAndInfo with provided parameters that should succeed.
 * @param api Connect ApiPromise
 * @param {KeyringPair} sudoKey Connected API Promise w/ sudo rights.
 * @param assetId Id for the asset
 * @param threshold Percent close to mean to be rewarded
 * @param minAnswers Min answers before aggregation
 * @param maxAnswers Max answers to aggregate
 * @param blockInterval blocks until oracle triggered
 * @param reward reward amount for correct answer
 * @param slash slash amount for bad answer
 * @param emitPriceChanges Emit price changes event
 */
export async function txOracleAddAssetAndInfoSuccessTest(
  api: ApiPromise,
  sudoKey: KeyringPair,
  assetId: number | u128 | AnyNumber | Uint8Array,
  threshold: Percent | AnyNumber | Uint8Array,
  minAnswers: number | u32 | AnyNumber | Uint8Array,
  maxAnswers: number | u32 | AnyNumber | Uint8Array,
  blockInterval: number | u32 | AnyNumber | Uint8Array,
  reward: number | u128 | AnyNumber | Uint8Array,
  slash: number | u128 | AnyNumber | Uint8Array,
  emitPriceChanges: boolean | Bool
) {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.oracle.addAssetAndInfo(
        assetId,
        threshold,
        minAnswers,
        maxAnswers,
        blockInterval,
        reward,
        slash,
        emitPriceChanges
      )
    )
  );
}
