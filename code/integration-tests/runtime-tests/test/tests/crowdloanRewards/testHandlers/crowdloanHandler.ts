import { KeyringPair } from "@polkadot/keyring/types";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { AnyNumber, IKeyringPair, ITuple } from "@polkadot/types/types";
import { PalletCrowdloanRewardsModelsRemoteAccount } from "@composable/types/interfaces";
import { Compact, u128, u32, u64, Vec } from "@polkadot/types-codec";
import { shares } from "@composabletests/tests/crowdloanRewards/contributions.json";
import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";
import { AccountId32 } from "@polkadot/types/interfaces";
import { ethers, Wallet } from "ethers";

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

const toHexString = (bytes: unknown) =>
  Array.prototype.map.call(bytes, x => ("0" + (x & 0xff).toString(16)).slice(-2)).join("");

// The prefix is defined as pallet config
const proofMessageKsm = (account: IKeyringPair) => "<Bytes>picasso-" + toHexString(account.publicKey) + "</Bytes>";

const proofMessageEth = (account: Uint8Array) => `picasso-${toHexString(account)}`;

export const getAmountAvailableToClaim = (api: ApiPromise, accountId: Uint8Array) =>
  api.rpc.crowdloanRewards.amountAvailableToClaimFor(accountId);

export const getKsmProofMessage = (api: ApiPromise, contributor: KeyringPair, contributorRewardAccount: IKeyringPair) =>
  api.createType("PalletCrowdloanRewardsModelsProof", {
    RelayChain: [contributor.publicKey, { Sr25519: contributor.sign(proofMessageKsm(contributorRewardAccount)) }]
  });

export const getEthProofMessage = async (
  api: ApiPromise,
  contributor: ethers.Signer,
  contributorRewardAccount: IKeyringPair
) => {
  const proofMessage = proofMessageEth(contributorRewardAccount.publicKey);
  const signedMessage = await contributor.signMessage(proofMessage);
  return api.createType("PalletCrowdloanRewardsModelsProof", {
    Ethereum: signedMessage
  });
};

export const getKsmContributorWallet = (testWallet: KeyringPair) => testWallet.derive("/contributor");

export class TxCrowdloanRewardsTests {
  public static async mintAndTransferFundsToCrowdloanPallet(
    api: ApiPromise,
    sudoKey: KeyringPair,
    amount: u128 | Compact<u128> | AnyNumber
  ) {
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.assets.mintInto(1, sudoKey.publicKey, amount))
    );
    expect(result).to.not.be.an("Error");
    const palletPublicKey = api.consts.crowdloanRewards.accountId;
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.balances.Transfer.is,
      api.tx.balances.transfer(palletPublicKey, amount)
    );
  }

  public static txCrowdloanRewardsInitializeTest(api: ApiPromise, sudoKey: KeyringPair) {
    return sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.initialize())
    );
  }

  public static async txCrowdloanRewardsPopulateTest(
    api: ApiPromise,
    sudoKey: KeyringPair,
    testWallets: KeyringPair[],
    ethContributors: Wallet[],
    testWalletShareAmountPICA: BN,
    vestingPeriod: number | bigint | BN,
    shortVestingTimeWalletIndices: number[] | undefined = undefined
  ) {
    let fullRewardAmount = new BN(0);

    const vestingTime = api.createType("u64", vestingPeriod);

    let contributors: Array<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]> = [];
    // Before we go through all the contributors, we inject our test wallet at the very beginning.
    const testContributorReward = api.createType("u128", testWalletShareAmountPICA.mul(new BN(10).pow(new BN(12))));
    for (const ethContributor of ethContributors) {
      const testContributorRemoteObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
        Ethereum: ethContributor.address
      });
      fullRewardAmount = fullRewardAmount.add(testContributorReward);
      contributors.push([testContributorRemoteObject, testContributorReward, vestingTime]);
    }
    for (const [i, testWallet] of testWallets.entries()) {
      const testContributorRemoteObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
        RelayChain: getKsmContributorWallet(testWallet).publicKey
      });
      fullRewardAmount = fullRewardAmount.add(testContributorReward);
      contributors.push([
        testContributorRemoteObject,
        testContributorReward,
        !!shortVestingTimeWalletIndices && shortVestingTimeWalletIndices.includes(i)
          ? api.createType("u64", 500000) // We want some testing wallets to have a very short vesting period.
          : vestingTime
      ]);
    }

    // Now we can continue collecting & populating our actual contributors.
    // Iterating through our list of contributors
    let i = 0;
    const allContributors: Array<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]> = [];
    for (const [key, value] of Object.entries(shares)) {
      let remoteAccountObject: PalletCrowdloanRewardsModelsRemoteAccount;
      // Creating either an ethereum or ksm contributor object.
      if (key.startsWith("0x"))
        remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", { Ethereum: key });
      else
        remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
          RelayChain: api.createType("AccountId32", key)
        });
      const currentContributorAmount = new BN(parseInt(value)).mul(new BN(10).pow(new BN(12)));
      fullRewardAmount = fullRewardAmount.add(currentContributorAmount);
      contributors.push([remoteAccountObject, api.createType("u128", currentContributorAmount), vestingTime]);

      // Every 2500th iteration we send our list of contributors, else we'd break the block data size limit.
      if (
        (i % 2500 == 0 && i != 0) ||
        (Object.entries(shares).length - i < 2500 && Object.entries(shares).length == i - 1)
      ) {
        // Actual population step.
        const {
          data: [result]
        } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTestHandler(api, sudoKey, contributors);
        expect(result.isOk).to.be.true;
        contributors.forEach(contributor => allContributors.push(contributor));
        contributors = [];
      }
      i++;
    }
    return { fullRewardAmount, allContributors };
  }

  public static async verifyCrowdloanRewardsPopulation(
    api: ApiPromise,
    contributors: Array<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>
  ) {
    for (const contributor of contributors) {
      const rewardsQuery = await api.query.crowdloanRewards.rewards(contributor[0]);
      expect(rewardsQuery.unwrap().claimed).to.be.bignumber.equal(new BN(0));
      expect(rewardsQuery.unwrap().total).to.be.bignumber.equal(contributor[1]);
      expect(rewardsQuery.unwrap().vestingPeriod).to.be.bignumber.equal(contributor[2]);
    }
  }

  public static async txCrowdloanRewardsPopulateTestHandler(
    api: ApiPromise,
    sudoKey: KeyringPair,
    contributors:
      | [PalletCrowdloanRewardsModelsRemoteAccount, u128, u32][]
      | Vec<ITuple<[PalletCrowdloanRewardsModelsRemoteAccount, u128, u64]>>
      | [
          string | Uint8Array | PalletCrowdloanRewardsModelsRemoteAccount | { RelayChain: any } | { Ethereum: any },
          u128 | AnyNumber,
          AnyNumber | u64
        ][]
  ) {
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.populate(contributors))
    );
  }

  private static async verifyAssociation(
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

  public static async verifyKsmAssociation(
    api: ApiPromise,
    resultRemoteAccount: PalletCrowdloanRewardsModelsRemoteAccount,
    resultRewardAccount: AccountId32,
    rewardAccount: KeyringPair,
    testWalletRewardSum: BN,
    initialAssociateClaimPercent: number,
    ksmContributorWallet: Uint8Array = getKsmContributorWallet(rewardAccount).publicKey,
    dontCheckAmounts = true
  ) {
    const remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
      RelayChain: ksmContributorWallet
    });
    return await TxCrowdloanRewardsTests.verifyAssociation(
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

  public static async verifyEthAssociation(
    api: ApiPromise,
    resultRemoteAccount: PalletCrowdloanRewardsModelsRemoteAccount,
    resultRewardAccount: AccountId32,
    rewardAccount: KeyringPair,
    testWalletRewardSum: BN,
    initialAssociateClaimPercent: number,
    ethContributorWallet: Wallet
  ) {
    const remoteAccountObject = api.createType("PalletCrowdloanRewardsModelsRemoteAccount", {
      Ethereum: ethContributorWallet.address
    });
    return await TxCrowdloanRewardsTests.verifyAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      rewardAccount,
      testWalletRewardSum,
      initialAssociateClaimPercent,
      remoteAccountObject
    );
  }

  public static async sendClaimsWithDelay(api: ApiPromise, wallet: KeyringPair, delay = 0) {
    await sleep(delay);
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    );
  }

  public static async verifyRewardsUnlock(
    api: ApiPromise,
    walletsToBeUnlocked: KeyringPair[],
    transferReceiverWallet: Uint8Array
  ) {
    for (const wallet of walletsToBeUnlocked) {
      const walletBalances = await api.query.system.account(wallet.publicKey);
      expect(walletBalances.data.miscFrozen).to.be.bignumber.equal(new BN(0));
      expect(walletBalances.data.free).to.be.bignumber.greaterThan(new BN(10).pow(new BN(12)));

      // Transferring 1 PICA as test.
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.balances.Transfer.is,
        api.tx.assets.transferNative(transferReceiverWallet, new BN(10).pow(new BN(12)), true)
      );
      expect(result).to.not.be.an("Error");
    }
  }
}
