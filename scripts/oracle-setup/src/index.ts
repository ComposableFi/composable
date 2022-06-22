#!/usr/bin/env ts-node
/**
 * This is the entry point for the price feed setup script.
 *
 * It does the following:
 * 1. Connect to the node, setup KeyRing & wallets.
 */
import { getDevWallets } from "./utils/walletHelper";
import {
  addOracleStake,
  connect,
  createOracleForAsset,
  registerOffChainWorker,
  setOracleSigner,
  setOracleURL,
  verifyAddOracleStake,
  verifyOffChainWorkerRegister,
  verifyOracleSigner
} from "./handlers";
import { nodes, oracle_parameters, price_feed_settings, auto_register_offchain_worker_enabled } from "./config.json";
import { expect } from "chai";

const main = async () => {
  console.log("Composable Oracle Initialization");
  console.debug("Connecting...");
  // Establish connection to the node.
  const { api, keyring } = await connect();

  // Getting wallets.
  const { devWalletAlice } = getDevWallets(keyring);

  // Setting up the oracle.
  console.log("Setting up the oracle");
  const {
    data: [oracleCreationResult]
  } = await createOracleForAsset(api, devWalletAlice, oracle_parameters);
  expect(oracleCreationResult.isOk).to.be.true;

  // Configuring Picasso with price feed.
  console.log("Setting Price Feed URL");
  await setOracleURL(api, price_feed_settings.key, price_feed_settings.value);

  console.log("Setting up node", nodes[0].address);

  // Registering off-chain worker for node if enabled.
  if (auto_register_offchain_worker_enabled) {
    await registerOffChainWorker(api, nodes[0].mnemonic, nodes[0].address).then(async () => {
      await verifyOffChainWorkerRegister(api, nodes[0].address);
    });
  }

  // Settings oracle signers.
  console.log("Setting node to signer");
  const {
    data: [resultAccount0, resultAccount1]
  } = await setOracleSigner(api, devWalletAlice, nodes[0].address);
  verifyOracleSigner(api, resultAccount0, resultAccount1, nodes[0].address, devWalletAlice);

  // Adding oracle stakes.
  console.log("Adding oracle stakes for node");
  const {
    data: [result]
  } = await addOracleStake(api, devWalletAlice, 9500000000);
  console.log("Oracle Price Feed Initialization finished!");
  verifyAddOracleStake(api, result, devWalletAlice.publicKey);

  // Disconnecting from the node.
  console.debug("disconnecting...");
  await api.disconnect();
};

main()
  .then(() => {
    console.log("Finished setting up oracle!");
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
