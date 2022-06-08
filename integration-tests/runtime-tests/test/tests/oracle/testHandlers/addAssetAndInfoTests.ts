import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

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
 */
export async function txOracleAddAssetAndInfoSuccessTest(
  api: ApiPromise,
  sudoKey: KeyringPair,
  assetId,
  threshold,
  minAnswers,
  maxAnswers,
  blockInterval,
  reward,
  slash
) {
  return await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.oracle.addAssetAndInfo(assetId, threshold, minAnswers, maxAnswers, blockInterval, reward, slash)
    )
  );
}
