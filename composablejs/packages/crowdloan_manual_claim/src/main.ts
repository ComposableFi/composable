#!/usr/bin/env ts-node
import "@composable/types/augment-api";
import "@composable/types/augment-types";
import { getNewConnection, sendUnsignedAndWaitForSuccess } from "@composable/utils";
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

async function verifyAssociation(
  api: ApiPromise,
  resultRemoteAccount: PalletCrowdloanRewardsModelsRemoteAccount,
  resultRewardAccount: AccountId32,
  rewardAccount: KeyringPair,
  testWalletRewardSum: BN,
  initialAssociateClaimPercent: number,
  remoteAccountObject: PalletCrowdloanRewardsModelsRemoteAccount,
  dontCheckAmounts = false
) {
  expect(resultRewardAccount.toString()).to.be.equal(
    api.createType("AccountId32", rewardAccount.publicKey).toString()
  );

  // Verifying query.
  const associationQuery = await api.query.crowdloanRewards.associations(rewardAccount.publicKey);
  expect(resultRemoteAccount.toString()) // Result from extrinsic.
    .to.be.equal(associationQuery.unwrap().toString()) // Result from query.
    .to.be.equal(remoteAccountObject.toString()); // Expected

  const expectedClaimedAmount = testWalletRewardSum
    .div(new BN(100).divn(initialAssociateClaimPercent))
    .mul(new BN(10).pow(new BN(12)));

  const lockedAmount = await api.query.balances.locks(rewardAccount.publicKey);
  expect(lockedAmount.length).to.be.equal(1);
  if (!dontCheckAmounts)
    expect(lockedAmount[0].amount).to.be.bignumber.closeTo(
      expectedClaimedAmount,
      expectedClaimedAmount.div(new BN(10000)) // Within 0.01%
    );
}

async function verifyKsmAssociation(
  api: ApiPromise,
  resultRemoteAccount: PalletCrowdloanRewardsModelsRemoteAccount,
  resultRewardAccount: AccountId32,
  rewardAccount: KeyringPair,
  testWalletRewardSum: BN,
  initialAssociateClaimPercent: number,
  ksmContributorWallet: Uint8Array,
  dontCheckAmounts = true
) {
  const remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
    RelayChain: ksmContributorWallet
  });
  return await verifyAssociation(
    api,
    resultRemoteAccount,
    resultRewardAccount,
    rewardAccount,
    testWalletRewardSum,
    initialAssociateClaimPercent,
    remoteAccountObject,
    dontCheckAmounts
  );
}

const main = async () => {
  console.log("Crowdloan Pallet Verifier");

  console.log("Connecting...");
  // Establish connection to the node.
  const endpoint = process.env.ENDPOINT ?? "ws://127.0.0.1:9988";
  const { newClient, newKeyring } = await getNewConnection(endpoint);
  const contributorWallet = newKeyring.addFromMnemonic("");
  const rewardAccount = newKeyring.addFromMnemonic("");
  const proofMessage = getKsmProofMessage(newClient, contributorWallet, rewardAccount);
  const {
    data: [resultRemoteAccount, resultRewardAccount]
  } = await sendUnsignedAndWaitForSuccess(
    newClient,
    newClient.events.crowdloanRewards.Associated.is,
    newClient.tx.crowdloanRewards.associate(rewardAccount.publicKey, proofMessage)
  );

  // Verification
  await verifyKsmAssociation(
    newClient,
    resultRemoteAccount,
    resultRewardAccount,
    rewardAccount,
    TEST_WALLET_PICA_REWARD_AMOUNT,
    INITIAL_ASSOCIATE_CLAIM_PERCENT
  );

  // Disconnecting from the node.
  console.debug("disconnecting...");
  await newClient.disconnect();
};

main()
  .then(() => {
    console.log("Crowdloan data verification finished!");
    process.exit(0);
  })
  .catch(err => {
    console.error(err.toString());
    process.exit(1);
  });
