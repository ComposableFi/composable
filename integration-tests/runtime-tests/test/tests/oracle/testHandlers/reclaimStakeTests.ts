/**
 * Tests tx.oracle.reclaimStake with provided parameters that should succeed.
 * @param controller KeyringPair which is a controller.
 */
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";


export async function txOracleReclaimStakeSuccessTest(controller) {
  return await sendAndWaitForSuccess(
    api,
    controller,
    api.events.oracle.StakeReclaimed.is,
    api.tx.oracle.reclaimStake(),
    false
  );
}
