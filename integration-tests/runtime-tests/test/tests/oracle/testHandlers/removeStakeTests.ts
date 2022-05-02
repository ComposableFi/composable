import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";

/**
 * Tests tx.oracle.removeStake with provided parameters that should succeed.
 * @param controller KeyringPair which is a controller.
 */
export async function txOracleRemoveStakeSuccessTest(controller) {
  return await sendAndWaitForSuccess(
    api,
    controller,
    api.events.oracle.StakeRemoved.is,
    api.tx.oracle.removeStake(),
    false
  );
}
