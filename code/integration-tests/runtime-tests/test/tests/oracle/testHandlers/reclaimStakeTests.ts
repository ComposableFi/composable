import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

/**
 * Tests tx.oracle.reclaimStake with provided parameters that should succeed.
 * @param api Connected ApiPromise
 * @param controller KeyringPair which is a controller.
 */
export async function txOracleReclaimStakeSuccessTest(api: ApiPromise, controller: KeyringPair) {
  return await sendAndWaitForSuccess(
    api,
    controller,
    api.events.oracle.StakeReclaimed.is,
    api.tx.oracle.reclaimStake(),
    false
  );
}
