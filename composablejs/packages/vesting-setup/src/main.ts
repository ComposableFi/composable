#!/usr/bin/env ts-node
/**
 * This is the entry point for the vesting schedules initialization script.
 *
 * It creates vesting schedules based on `schedules.json` and `config.json`
 */
import "@composable/types/augment-api";
import "@composable/types/augment-types";

import { getDevWallets, sendAndWaitForSuccess, getNewConnection } from "@composable/utils";
import { ApiPromise, Keyring } from "@polkadot/api";
import { expect } from "chai";
// This file contains the asset id and the `from` account.
// TODO: Replace mock data
import { config } from "./config.json";
// This file contains the vesting schedules to be generated.
// TODO: Replace mock data
import { schedules } from "./schedules.json";

/**
 * Establishes connection.
 *
 * @returns {
 * api: ApiPromise,
 * keyring: Keyring,
 * walletAlice: KeyringPair,
 * }
 */
async function connect(): Promise<{ api: ApiPromise; keyring: Keyring }> {
  const endpoint = "ws://" + (process.env.ENDPOINT ?? "127.0.0.1:9988");
  const { newClient, newKeyring } = await getNewConnection(endpoint);
  return { api: newClient, keyring: newKeyring };
}

/**
 * Check that both `config.json` and `schedules.json` meet minimum requirements.
 * TODO: check that data is always the correct type. Will wait for actual schedules for this.
 */
async function sanitizeData() {
  if (!config.asset) {
    return Promise.reject("config.asset is missing");
  }
  if (!config.address) {
    return Promise.reject("config.address is missing");
  }
  if (!config.windowType) {
    return Promise.reject("config.windowType is missing");
  }
  if (config.windowType !== "block" && config.windowType !== "moment") {
    return Promise.reject(
      `config.windowType must be either "block" or "moment", but is currently "${config.windowType}"`
    );
  }

  const schedulesAreSanitized = schedules.every(
    schedule =>
      !!(
        schedule.start &&
        schedule.vestingPeriodCount &&
        schedule.windowPeriod &&
        schedule.perPeriodAmount &&
        schedule.beneficiary
      )
  );

  if (!schedulesAreSanitized) {
    return Promise.reject("At least one schedule is missing some parameter.");
  }
}

const main = async () => {
  // Make sure `config.json` and `schedules.json` meet requirements.
  await sanitizeData();

  console.log("Composable Vesting Schedules Initialization");
  console.debug("Connecting...");
  // Establish connection to the node.
  const { api, keyring } = await connect();

  // Get wallets.
  const { devWalletAlice } = getDevWallets(keyring);

  // Create vested transfers.
  const transactions = schedules.map(schedule => {
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        [config.windowType === "block" ? "BlockNumberBased" : "MomentBased"]: {
          start: api.createType("u64", schedule.start),
          period: api.createType("u64", schedule.windowPeriod)
        }
      }),
      periodCount: schedule.vestingPeriodCount,
      perPeriod: api.createType("u128", schedule.perPeriodAmount)
    });

    return api.tx.vesting.vestedTransfer(config.address, schedule.beneficiary, config.asset, scheduleInfo);
  });

  // Execute transactions.
  console.log("Creating schedules...");
  const {
    data: [result]
  } = await sendAndWaitForSuccess(
    api,
    devWalletAlice,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.utility.batch(transactions))
  );

  // Assert successful creation.
  expect(result.isOk).to.be.true;

  console.log("Disconnecting...");
  await api.disconnect();
};

main()
  .then(() => {
    console.log("Schedules created successfully!");
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
