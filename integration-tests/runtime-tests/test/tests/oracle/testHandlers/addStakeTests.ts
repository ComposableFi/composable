import { u128 } from "@polkadot/types-codec";
import { expect } from "chai";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";


export async function runBeforeTxOracleAddStake(sudoKey, wallet1, wallet2) {
  const { data: [result1] } = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(1, wallet1.publicKey, 555555555555)
    )
  );
  expect(result1.isOk).to.be.true;
  const { data: [result2] } = await sendAndWaitForSuccess(
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(
      api.tx.assets.mintInto(1, wallet2.publicKey, 555555555555)
    )
  );
  expect(result2.isOk).to.be.true;
  return;
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
