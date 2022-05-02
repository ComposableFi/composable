import { IKeyringPair } from "@polkadot/types/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";


/**
 * Tests tx.oracle.addAssetAndInfo with provided parameters that should succeed.
 * @param {IKeyringPair} sudoKey Connected API Promise w/ sudo rights.
 * @param assetId Id for the asset
 * @param threshold Percent close to mean to be rewarded
 * @param minAnswers Min answers before aggregation
 * @param maxAnswers Max answers to aggregate
 * @param blockInterval blocks until oracle triggered
 * @param reward reward amount for correct answer
 * @param slash slash amount for bad answer
 */
export async function txOracleAddAssetAndInfoSuccessTest(
  sudoKey: IKeyringPair,
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
      api.tx.oracle.addAssetAndInfo(
        assetId,
        threshold,
        minAnswers,
        maxAnswers,
        blockInterval,
        reward,
        slash
      )
    )
  );
}
