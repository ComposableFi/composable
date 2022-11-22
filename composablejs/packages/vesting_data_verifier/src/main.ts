#!/usr/bin/env ts-node
import "@composable/types/augment-api";
import "@composable/types/augment-types";
import { getNewConnection } from "@composable/utils";
import { verifyVestingPalletData } from "@composable/vesting_data_verifier/handler";


const CONTRIBUTOR_LIST_URL =
  "https://raw.githubusercontent.com/ComposableFi/composable/261966d2cf9a9c5ce8b4f7440d3040f866199b23/composablejs/packages/vesting-setup/src/sample.csv";

const main = async () => {
  console.log("Vesting Pallet Verifier");

  console.log("Connecting...");
  // Establish connection to the node.
  const endpoint = process.env.ENDPOINT ?? "ws://127.0.0.1:9988";
  const { newClient } = await getNewConnection(endpoint);

  const contributorsUrl = process.env.CONTRIBUTOR_LIST_URL ?? CONTRIBUTOR_LIST_URL;

  // Here the actual magic happens
  // @ts-ignore
  await verifyVestingPalletData(newClient, contributorsUrl);

  // Disconnecting from the node.
  console.debug("disconnecting...");
  await newClient.disconnect();
};

main()
  .then(() => {
    console.log("Vesting pallet data verification finished!");
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
