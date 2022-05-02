import { u128 } from "@polkadot/types-codec";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";


export async function runBeforeTxOracleAddStake(sudoKey, wallet1, wallet2) {
  await mintAssetsToWallet(
    wallet1,
    sudoKey,
    [1]
  );
  await mintAssetsToWallet(
    wallet2,
    sudoKey,
    [1]
  );
}

/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param sender Connected API Promise w/ sudo rights.
 * @param {u128} stake Staking amount.
 */
export async function txOracleAddStakeSuccessTest(sender, stake: u128) {
  return await sendAndWaitForSuccess(
    api,
    sender,
    api.events.oracle.StakeAdded.is,
    api.tx.oracle.addStake(stake),
    false
  );
}
