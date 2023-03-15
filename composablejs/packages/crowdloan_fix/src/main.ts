#!/usr/bin/env ts-node
import "@composable/types/augment-api";
import "@composable/types/augment-types";
import { getDevWallets, getNewConnection } from "@composable/utils";
import *  as updateSchedule from "@composable/crowdloan_fix/update_schedule";
import {
  fixCrowdloanEntry,
  getContributorData,
  getContributorsToModify,
  getContributorsToReplace
} from "@composable/crowdloan_fix/handler";
import BN from "bn.js";
import yesno from "yesno";


const main = async () => {
  console.log("Crowdloan Pallet Fix");

  console.log("Connecting...");
  // Establish connection to the node.
  const endpoint = process.env.ENDPOINT ?? "ws://127.0.0.1:9988";
  const { newClient, newKeyring } = await getNewConnection(endpoint);
  // ToDo: Replace Alice with live wallet!
  const { devWalletAlice } = getDevWallets(newKeyring);

  let updates: { RemoteAccountOf: string, RewardAmountOf: BN, VestingPeriodOf: BN }[];
  // Here the actual magic happens
  // 1. Replace jobs
  console.info("Step 1. Collecting Replacement Jobs");
  updates = await getContributorsToReplace(newClient);


  // 2. Modification jobs
  console.info("Step 2. Collecting Modification Jobs");
  updates = updates.concat(await getContributorsToModify(newClient));

  // 3. Transaction
  console.info("Step 3. Submitting updates");
  console.info("Following updates will be submitted:");
  for (let i = 0; i < updates.length; i++) {
    console.info("\nWallet:", updates[i].RemoteAccountOf['RelayChain'].toString(), `\nAmount: ${updates[i].RewardAmountOf.toNumber()} | Vesting Period: ${updates[i].VestingPeriodOf.toNumber()}`);
  }
  const ok = await yesno({ question: "Are you sure you want to continue?" });
  if (!ok)
    process.exit(2);
  await fixCrowdloanEntry(newClient, devWalletAlice, updates);


  // Disconnecting from the node.
  console.debug("disconnecting...");
  await newClient.disconnect();
};

main()
  .then(() => {
    console.log("Crowdloan data fix finished!");
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
