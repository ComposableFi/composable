#!/usr/bin/env ts-node
import "@composable/types/augment-api";
import "@composable/types/augment-types";
import { getDevWallets, getNewConnection, sendUnsignedAndWaitForSuccess } from "@composable/utils";
import *  as updateSchedule from "@composable/crowdloan_fix/update_schedule";
import BN from "bn.js";
import yesno from "yesno";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { IKeyringPair } from "@polkadot/types/types";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types";
import { AccountId32 } from "@polkadot/types/interfaces";
import { expect } from "chai";


export const getKsmProofMessage = (api: ApiPromise, contributor: KeyringPair, contributorRewardAccount: IKeyringPair) =>
  api.createType("PalletCrowdloanRewardsModelsProof", {
    RelayChain: [contributor.publicKey, { Sr25519: contributor.sign(proofMessageKsm(contributorRewardAccount)) }]
  });

const proofMessageKsm = (account: IKeyringPair) => "<Bytes>picasso-" + toHexString(account.publicKey) + "</Bytes>";

const toHexString = (bytes: unknown) =>
  Array.prototype.map.call(bytes, x => ("0" + (x & 0xff).toString(16)).slice(-2)).join("");


const main = async () => {
  console.log("Crowdloan Pallet Manual Claim Helper");

  console.log("Connecting...");
  // Establish connection to the node.
  const endpoint = process.env.ENDPOINT ?? "ws://127.0.0.1:9988";
  const { newClient, newKeyring } = await getNewConnection(endpoint);
  const {devWalletAlice} = getDevWallets(newKeyring);
  // const contributorWallet = newKeyring.addFromMnemonic("");
  // const rewardAccount = newKeyring.addFromMnemonic("");

  const rewardWallet = devWalletAlice.derive("/test/crowdloan/contributor0");
  const contributorWallet = devWalletAlice.derive("/test/crowdloan/contributor0/contributor");

  const proofMessage = getKsmProofMessage(newClient, contributorWallet, rewardWallet);

  await sendUnsignedAndWaitForSuccess(
    newClient,
    newClient.events.crowdloanRewards.Associated.is,
    newClient.tx.crowdloanRewards.associate(rewardWallet.publicKey, proofMessage)
  );

  // Disconnecting from the node.
  console.debug("disconnecting...");
  await newClient.disconnect();
};

main()
  .then(() => {
    console.log("Crowdloan data manual claim finished!");
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
