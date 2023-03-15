import { ApiPromise } from "@polkadot/api";
import { sendAndWaitForSuccess } from "@composable/utils";
import { KeyringPair } from "@polkadot/keyring/types";
import BN from "bn.js";
import * as updateSchedule from "@composable/crowdloan_fix/update_schedule";
import * as assert from "assert";

export async function getContributorsToReplace(api: ApiPromise) {
  const updates: { RemoteAccountOf: string, RewardAmountOf: BN, VestingPeriodOf: BN }[] = [];
  for (const key in updateSchedule.replace) {
    const newPublicKey = updateSchedule.replace[key]["new"];
    const oldPublicKey = updateSchedule.replace[key]["old"];
    let amount, vestingPeriod;
    try {
      const { rewardAmount, rewardVestingPeriod } = await getContributorData(api, oldPublicKey);
      amount = rewardAmount;
      vestingPeriod = rewardVestingPeriod;
    } catch (e) {
      printErrorMessage(e, oldPublicKey);
      continue;
    }
    const updateDataNewAccount = {
      RemoteAccountOf: { "RelayChain": newPublicKey },
      RewardAmountOf: new BN(amount),
      VestingPeriodOf: new BN(vestingPeriod)
    };
    updates.push(updateDataNewAccount);
    const updateDataOldAccount = {
      RemoteAccountOf: { "RelayChain": oldPublicKey },
      RewardAmountOf: new BN("0"),
      VestingPeriodOf: new BN("0")
    };
    updates.push(updateDataOldAccount);
    console.info(`Replacing \`${oldPublicKey}\` with \`${newPublicKey}\`\nAmount: ${amount} | vestingPeriod: ${vestingPeriod}`);
  }
  return updates;
}

export async function getContributorsToModify(api: ApiPromise) {
  const updates: { RemoteAccountOf: string, RewardAmountOf: BN, VestingPeriodOf: BN }[] = [];
  for (const key in updateSchedule.modify) {
    const publicKey = updateSchedule.modify[key]["publicKey"];
    const value = updateSchedule.modify[key]["newAmount"];
    let vestingPeriod;
    try {
      const { rewardAmount: oldRewardAmount, rewardVestingPeriod } = await getContributorData(api, publicKey);
      vestingPeriod = rewardVestingPeriod;
    } catch (e) {
      printErrorMessage(e, publicKey);
      continue;
    }
    const updateData = {
      RemoteAccountOf: { "RelayChain": publicKey },
      RewardAmountOf: new BN(value.toString()),
      VestingPeriodOf: new BN(vestingPeriod)
    };
    updates.push(updateData);
    console.info(`Updating \`${publicKey}\` with:\nAmount: ${value} | vestingPeriod: ${vestingPeriod}`);
  }
  return updates;
}

function printErrorMessage(e: Error, publicKey: string) {
  console.error("##################################################################");
  console.error(`Could not find contributor ${publicKey}'s reward data on chain!`);
  console.error(e.toString());
  console.error("Ignoring but please be aware.");
  console.error("##################################################################");
}

export async function getContributorData(api: ApiPromise, publicKey: string) {
  const associations = await api.query.crowdloanRewards.associations(publicKey);
  const rewardData = await api.query.crowdloanRewards.rewards({ "RelayChain": api.createType("AccountId32", associations.toHuman()["RelayChain"]) });
  const rewardAmount = new BN(rewardData.toHuman()["total"].toString().replaceAll(",", ""));
  const rewardVestingPeriod = new BN(rewardData.toHuman()["vestingPeriod"].toString().replaceAll(",", ""));
  return { rewardAmount, rewardVestingPeriod };
}

/**
 * Updates a single crowdloan contributor entry based on data provided.
 *
 * @param api Connected API Client
 * @param sudoKey
 * @param additions
 */
export async function fixCrowdloanEntry(api: ApiPromise, sudoKey: KeyringPair, additions: { RemoteAccountOf: string, RewardAmountOf: BN, VestingPeriodOf: BN }[]) {
  const { data: [result] } = await sendAndWaitForSuccess(
    // @ts-ignore
    api,
    sudoKey,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.crowdloanRewards.add(additions))
  );
  assert.ok(result.isOk);
}
