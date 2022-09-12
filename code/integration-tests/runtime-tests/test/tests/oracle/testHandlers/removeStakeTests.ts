import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

/**
 * Tests tx.oracle.removeStake with provided parameters that should succeed.
 * @param controller KeyringPair which is a controller.
 */
export async function txOracleRemoveStakeSuccessTest(api: ApiPromise, controller: KeyringPair) {
  return await sendAndWaitForSuccess(
    api,
    controller,
    api.events.oracle.StakeRemoved.is,
    api.tx.oracle.removeStake(),
    false
  );
}
