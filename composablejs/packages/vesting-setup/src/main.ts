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
import { parse } from "csv-parse";
import path from "path";
// This file contains the asset id and the `from` account.
// TODO: Replace mock data
import { config } from "./config.json";
// This file contains the vesting schedules to be generated.
// TODO: Replace mock data
import fs from "fs";

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
 * Check that `config.json` has all the necessary data.
 */
async function sanitizeData() {
  if (!config.asset) {
    return Promise.reject("config.asset is missing");
  }
  if (!config.address) {
    return Promise.reject("config.address is missing");
  }
  if (!config.period) {
    return Promise.reject("config.period is missing");
  }
  if (!config.startDate) {
    return Promise.reject("config.startDate is missing");
  }
  if (!config.endDate) {
    return Promise.reject("config.endDate is missing");
  }
}

type VestedTransfer = {
  beneficiary: string;
  perPeriod: number;
  periodCount: number;
};

const parseCsv = async (): Promise<{ start: number; vestedTransfers: VestedTransfer[] }> => {
  const startDate = new Date(config.startDate);
  // const startDate = new Date();
  const endDate = new Date(config.endDate);

  // TODO: set right values
  const startMoment = startDate.getTime();
  // const startMoment = startDate.getTime() + 30 * 1_000;
  const endMoment = endDate.getTime();
  const period = 12 * 1_000;

  const periods = Math.ceil((endMoment - startMoment) / period);

  const vestedTransfers: VestedTransfer[] = [];

  const sample = fs.readFileSync(path.join(__dirname, "sample.csv"));

  const parser = parse(sample, { delimiter: ",", from_line: 2 });

  // Parse data
  for await (const row of parser) {
    const beneficiary = row[0];
    const totalAmount = Number(row[1].replaceAll(",", ""));
    const perPeriod = totalAmount / periods;
    // Round up
    const perPeriodRounded = Math.ceil(perPeriod);

    // Adjust periods if necessary
    const extraAmount = (perPeriodRounded - perPeriod) * periods;
    const extraPeriods = Math.floor(extraAmount / perPeriodRounded);

    vestedTransfers.push({
      beneficiary,
      perPeriod: perPeriodRounded,
      periodCount: periods - extraPeriods
    });
  }

  return {
    start: startMoment,
    vestedTransfers
  };
};

const main = async () => {
  // Make sure `config.json` meet requirements.
  await sanitizeData();

  // Parse data
  const { start, vestedTransfers } = await parseCsv();

  console.log("Composable Vesting Schedules Initialization");
  console.debug("Connecting...");
  // Establish connection to the node.
  const { api, keyring } = await connect();

  // Get wallets.
  const { devWalletAlice } = getDevWallets(keyring);

  // Create vested transfers.
  const transactions = vestedTransfers.map(schedule => {
    const scheduleInfo = api.createType("ComposableTraitsVestingVestingScheduleInfo", {
      window: api.createType("ComposableTraitsVestingVestingWindow", {
        MomentBased: {
          start: api.createType("u64", start),
          period: api.createType("u64", config.period)
        }
      }),
      periodCount: schedule.periodCount,
      perPeriod: api.createType("u128", schedule.perPeriod)
    });

    return api.tx.vesting.vestedTransfer(config.address, schedule.beneficiary, config.asset, scheduleInfo);
  });

  // Execute transactions.
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
