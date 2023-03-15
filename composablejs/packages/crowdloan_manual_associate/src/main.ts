#!/usr/bin/env ts-node
import "@composable/types/augment-api";
import "@composable/types/augment-types";
import { getDevWallets, getNewConnection, sendUnsignedAndWaitForSuccess } from "@composable/utils";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { IKeyringPair } from "@polkadot/types/types";
import {contributor_wallet_privateKey, reward_wallet_privateKey} from '@composable/crowdloan_fix/claim_configuration'


const TESTNET_DEBUG_MODE = false;

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
  let rewardWallet: KeyringPair, contributorWallet:KeyringPair;
  if (TESTNET_DEBUG_MODE) {
    const { devWalletAlice } = getDevWallets(newKeyring);
    rewardWallet = devWalletAlice.derive("/test/crowdloan/contributor0");
    contributorWallet = devWalletAlice.derive("/test/crowdloan/contributor0/contributor");
  } else {
    rewardWallet = newKeyring.addFromMnemonic(reward_wallet_privateKey);
    contributorWallet = newKeyring.addFromMnemonic(contributor_wallet_privateKey);
  }

  const proofMessage = getKsmProofMessage(newClient, contributorWallet, rewardWallet);

  const { data: [resultRemoteAccount, resultRewardAccount, resultAmount] } = await sendUnsignedAndWaitForSuccess(
    newClient,
    newClient.events.crowdloanRewards.Associated.is,
    newClient.tx.crowdloanRewards.associate(rewardWallet.publicKey, proofMessage)
  );

  console.info(`Contributor: ${resultRemoteAccount.toString()}\nClaimed for ${resultRewardAccount.toString()}\nThe amount of ${resultAmount.toString()}`);

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
