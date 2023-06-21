import { expect } from "chai";
import { KeyringPair } from "@polkadot/keyring/types";
import testConfiguration from "./test_configuration.json";
import {
  getEthProofMessage,
  getKsmContributorWallet,
  getKsmProofMessage,
  TxCrowdloanRewardsTests
} from "@composabletests/tests/crowdloanRewards/testHandlers/crowdloanHandler";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess, sendUnsignedAndWaitForSuccess } from "@composable/utils/polkadotjs";
import BN from "bn.js";
import { Wallet } from "ethers";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";

const AMOUNT_CONTRIBUTOR_WALLETS = 11;
const AMOUNT_ETH_CONTRIBUTOR_WALLETS = 4;
const TEST_WALLET_PICA_REWARD_AMOUNT = new BN(100);
const INITIAL_ASSOCIATE_CLAIM_PERCENT = 25;

describe("[SHORT] CrowdloanRewards Tests", function () {
  if (!testConfiguration.enabledTests.tx.enabled) return;
  this.retries(0);
  let api: ApiPromise;

  let sudoKey: KeyringPair;

  let contributorsRewardAmount: BN;

  let contributorRewardWallets: KeyringPair[];
  let ethContributorWallets: Wallet[];
  let notContributor: KeyringPair;

  before("Setting up tests", async function () {
    this.timeout(2 * 60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    notContributor = sudoKey.derive("/test/crowdloan/not-contributor");

    contributorRewardWallets = [];
    for (let i = 0; i <= AMOUNT_CONTRIBUTOR_WALLETS; i++) {
      contributorRewardWallets.push(devWalletAlice.derive("/test/crowdloan/contributor" + i));
    }
    ethContributorWallets = [];
    for (let i = 0; i <= AMOUNT_ETH_CONTRIBUTOR_WALLETS; i++) {
      ethContributorWallets.push(Wallet.createRandom());
    }

    // Funding the wallets with small initial balance.
    await mintAssetsToWallet(api, contributorRewardWallets[1], sudoKey, [1], 1_000_000_000_000n); // Test #1.7
    await mintAssetsToWallet(api, contributorRewardWallets[3], sudoKey, [1], 1_000_000_000_000n); // Test #1.9
    await mintAssetsToWallet(api, notContributor, sudoKey, [1], 1_000_000_000_000n);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  it("1.1  I can, as sudo, populate the Crowdloan pallet with the list of contributorRewardWallets.", async function () {
    this.timeout(10 * 60 * 1000);
    const { fullRewardAmount, allContributors } = await TxCrowdloanRewardsTests.txCrowdloanRewardsPopulateTest(
      api,
      sudoKey,
      contributorRewardWallets,
      ethContributorWallets,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      999_999_999_999_999n,
      [1] // Short vesting period req. for #1.16
    );
    contributorsRewardAmount = fullRewardAmount;
    await TxCrowdloanRewardsTests.verifyCrowdloanRewardsPopulation(api, allContributors);
  });

  /*
  The following steps occur after the pallet has been populated with contributorRewardWallets.
   */
  it("1.2  I can not associate my KSM contributor wallet before the crowdloan pallet has been initialized.", async function () {
    this.timeout(2 * 60 * 1000);
    const rewardAccount = contributorRewardWallets[0];
    const proofMessage = getKsmProofMessage(api, getKsmContributorWallet(rewardAccount), rewardAccount);
    await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(rewardAccount.publicKey, proofMessage)
    ).catch(function (err) {
      expect(err.toString()).to.contain("Custom error: 3");
    });
  });

  it("1.18  I can not, as sudo, initialize the crowdloan pallet without providing at least as many funds as will be rewarded.", async function () {
    this.timeout(2 * 60 * 1000);
    // First testing initialization without any funds.
    const {
      data: [sudoResult]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.initialize())
    );
    expect(sudoResult.isErr).to.be.true;
    // @ts-ignore
    expect(sudoResult.asErr.asModule.index).to.be.bignumber.equal(new BN("58"));
    // @ts-ignore
    expect(sudoResult.asErr.asModule.error.toHex()).to.be.equal("0x03000000"); // Error index 3 == RewardsNotFunded

    // Second testing initialization with too little funds.
    await TxCrowdloanRewardsTests.mintAndTransferFundsToCrowdloanPallet(api, sudoKey, Math.pow(10, 12)); // Sending 1 PICA which is not enough.
    const {
      data: [sudoResult2]
    } = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.initialize())
    );
    expect(sudoResult2.isErr).to.be.true;
    // @ts-ignore
    expect(sudoResult2.asErr.asModule.index).to.be.bignumber.equal(new BN("58"));
    // @ts-ignore
    expect(sudoResult2.asErr.asModule.error.toHex()).to.be.equal("0x03000000"); // Error index 3 == RewardsNotFunded
  });

  it("1.3  I can, as sudo, initialize the Crowdloan Pallet", async function () {
    this.timeout(60 * 2 * 1000);
    await TxCrowdloanRewardsTests.mintAndTransferFundsToCrowdloanPallet(
      api,
      sudoKey,
      contributorsRewardAmount.sub(new BN(10).pow(new BN(12)))
    ); // Subtracting 1 PICA from earlier test #1.17

    const {
      data: [result]
    } = await TxCrowdloanRewardsTests.txCrowdloanRewardsInitializeTest(api, sudoKey);
    expect(result.isOk).to.be.true;
  });

  /*
  The following steps occur after the pallet was populated & initialised.
   */
  it("1.4  A user, without initial funds, can associate their contributor KSM wallet with a correct proof & claim 25% of the reward as locked balance.", async function () {
    this.timeout(2 * 60 * 1000);
    const rewardAccount = contributorRewardWallets[0];
    const proofMessage = getKsmProofMessage(api, getKsmContributorWallet(rewardAccount), rewardAccount);
    const {
      data: [resultRemoteAccount, resultRewardAccount]
    } = await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(rewardAccount.publicKey, proofMessage)
    );

    // Verification
    await TxCrowdloanRewardsTests.verifyKsmAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      rewardAccount,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      INITIAL_ASSOCIATE_CLAIM_PERCENT
    );
  });

  it("1.5  A user (#1.4) can not transfer their claimed funds.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = contributorRewardWallets[0];
    const testAmount = new BN(10).pow(new BN(12)); // 1 PICA
    const testTransactions = [
      api.tx.assets.transfer(1, sudoKey.publicKey, testAmount, true),
      api.tx.assets.transferNative(sudoKey.publicKey, testAmount, true)
    ];
    // We can not batch these transactions, due to our default batch transaction function, aborting on failure.
    await sendAndWaitForSuccess(api, wallet, api.events.balances.Transfer.is, testTransactions[0]).catch(function (
      err
    ) {
      expect(err.toString()).to.contain("balances.LiquidityRestrictions");
    });
    await sendAndWaitForSuccess(api, wallet, api.events.balances.Transfer.is, testTransactions[1]).catch(function (
      err
    ) {
      expect(err.toString()).to.contain("balances.LiquidityRestrictions");
    });
  });

  it("1.6  A user (#1.4) can claim a second time and pays transaction fees using the claimed, locked balance from earlier.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = contributorRewardWallets[0];
    const {
      data: [resultRemoteAccount, resultRewardAccount, resultAmount]
    } = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    );
    expect(resultRemoteAccount).to.not.be.an("Error");
    expect(resultRewardAccount.isEmpty).to.be.false;
    expect(resultAmount).to.be.bignumber.greaterThan(new BN(0));
  });

  it("1.7  A user, with initial funds, can associate their contributor KSM wallet with a correct proof & claim 25% of the reward as locked balance.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = contributorRewardWallets[1];
    const proofMessage = getKsmProofMessage(api, getKsmContributorWallet(wallet), wallet);
    const {
      data: [resultRemoteAccount, resultRewardAccount]
    } = await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, proofMessage)
    );

    // Verification
    await TxCrowdloanRewardsTests.verifyKsmAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      wallet,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      INITIAL_ASSOCIATE_CLAIM_PERCENT,
      getKsmContributorWallet(wallet).publicKey,
      true
    );
  });

  it("1.8  A user, without initial funds, can associate their contributor ETH wallet with a correct proof & claim 25% of the reward as locked balance.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = contributorRewardWallets[2];
    const ethWallet = ethContributorWallets[0];
    const proofMessage = await getEthProofMessage(api, ethWallet, wallet);
    const {
      data: [resultRemoteAccount, resultRewardAccount]
    } = await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, proofMessage)
    );

    // Verification
    await TxCrowdloanRewardsTests.verifyEthAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      wallet,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      INITIAL_ASSOCIATE_CLAIM_PERCENT,
      ethWallet
    );
  });

  it("1.9  Another user, with initial funds, can associate their contributor ETH wallet with a correct proof & claim 25% of the reward as locked balance.", async function () {
    this.timeout(2 * 60 * 1000);

    const wallet = contributorRewardWallets[3];
    const ethWallet = ethContributorWallets[1];
    const proofMessage = await getEthProofMessage(api, ethWallet, wallet);
    const {
      data: [resultRemoteAccount, resultRewardAccount]
    } = await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, proofMessage)
    );

    // Verification
    await TxCrowdloanRewardsTests.verifyEthAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      wallet,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      INITIAL_ASSOCIATE_CLAIM_PERCENT,
      ethWallet
    );
  });

  it("1.10  When claiming after transferring all initial funds from the account (#1.11), the newly claimed balance will be locked.", async function () {
    this.timeout(60 * 2 * 1000);
    const wallet = contributorRewardWallets[3];
    // Moving all funds from wallet.
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.balances.Transfer.is,
      api.tx.assets.transferAllNative(sudoKey.publicKey, false)
    );
    expect(result).to.not.be.an("Error");

    // Claiming
    const {
      data: [resultRemoteAccount, resultRewardAccount, resultAmount]
    } = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    );
    expect(resultRemoteAccount).to.not.be.an("Error");
    expect(resultRewardAccount.isEmpty).to.be.false;
    expect(resultAmount).to.be.bignumber.greaterThan(new BN(0));
    // All remaining available balance should be locked.
    await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.balances.Transfer.is,
      api.tx.assets.transferNative(sudoKey.publicKey, 1_000_000_000_000, false)
    ).catch(function (err) {
      expect(err.toString()).to.contain("balances.LiquidityRestrictions");
    });
  });

  it("1.11  Multiple users can associate successfully, at the same time.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallets = [
      contributorRewardWallets[4],
      contributorRewardWallets[5],
      contributorRewardWallets[6],
      contributorRewardWallets[7]
    ];

    const txs = [];
    for (const wallet of wallets) {
      txs.push(
        sendUnsignedAndWaitForSuccess(
          api,
          api.events.crowdloanRewards.Associated.is,
          api.tx.crowdloanRewards.associate(
            wallet.publicKey,
            getKsmProofMessage(api, getKsmContributorWallet(wallet), wallet)
          )
        )
      );
    }
    const results = await Promise.all(txs);

    // Verification
    for (let i = 0; i < wallets.length; i++) {
      await TxCrowdloanRewardsTests.verifyKsmAssociation(
        api,
        results[i].data[0],
        results[i].data[1],
        wallets[i],
        TEST_WALLET_PICA_REWARD_AMOUNT,
        INITIAL_ASSOCIATE_CLAIM_PERCENT
      );
    }
  });

  it("1.12  Multiple contributorRewardWallets (#1.12) can claim at the same time.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallets = [
      contributorRewardWallets[4],
      contributorRewardWallets[5],
      contributorRewardWallets[6],
      contributorRewardWallets[7]
    ];

    const txs = [];
    for (const wallet of wallets) {
      txs.push(
        sendAndWaitForSuccess(api, wallet, api.events.crowdloanRewards.Claimed.is, api.tx.crowdloanRewards.claim())
      );
    }
    const results = await Promise.all(txs);
    for (let i = 0; i < results.length; i++) {
      expect(results[i].data[0]).to.not.be.an("Error");
      expect(results[i].data[2]).to.be.bignumber.greaterThan(new BN(0));
    }
  });

  it("1.13  A user can not claim twice within the same block.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = contributorRewardWallets[0];
    await Promise.all([
      TxCrowdloanRewardsTests.sendClaimsWithDelay(api, wallet, 0),
      TxCrowdloanRewardsTests.sendClaimsWithDelay(api, wallet, 100)
    ]).catch(function (err) {
      expect(err.toString()).to.contain("crowdloanRewards.NothingToClaim");
    });
  });

  it("1.14  An already associated wallet can not associate again with a different reward type account.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = contributorRewardWallets[8];
    const ethWallet = ethContributorWallets[4];
    const proofMessage = await getEthProofMessage(api, ethWallet, wallet);
    const {
      data: [resultRemoteAccount, resultRewardAccount]
    } = await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, proofMessage)
    );

    // Verification
    await TxCrowdloanRewardsTests.verifyEthAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      wallet,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      INITIAL_ASSOCIATE_CLAIM_PERCENT,
      ethWallet
    );
    // One test claim for good measurement.
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    );
    expect(result).to.not.be.an("Error");

    // Now we try to re- associate with a different contributor but the same reward wallet.
    const newProofMessage = getKsmProofMessage(api, getKsmContributorWallet(wallet), wallet);
    await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, newProofMessage),
      true
    ).catch(function (err) {
      expect(err.toString()).to.contain("Custom error: 2");
    });
  });

  it("1.15  An already associated wallet can not associate the same reward account type a second time.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = contributorRewardWallets[9];

    const proofMessage = getKsmProofMessage(api, getKsmContributorWallet(wallet), wallet);
    const {
      data: [resultRemoteAccount, resultRewardAccount]
    } = await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, proofMessage)
    );

    // Verification
    await TxCrowdloanRewardsTests.verifyKsmAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      wallet,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      INITIAL_ASSOCIATE_CLAIM_PERCENT
    );
    // One test claim for good measurement.
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    );
    expect(result).to.not.be.an("Error");

    // ACTUAL TEST #1.15
    // Now we try to re- associate with a different contributor but the same reward wallet.
    const newProofMessage = getKsmProofMessage(api, getKsmContributorWallet(wallet), wallet);
    await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, newProofMessage),
      true
    ).catch(function (err) {
      expect(err.toString()).to.contain("Custom error: 2");
    });
  });

  it("1.16  Someone can re- associate their contributor wallet to a different Picasso wallet.", async function () {
    this.timeout(2 * 60 * 1000);
    const rewardAccount = contributorRewardWallets[10];
    const proofMessage = getKsmProofMessage(
      api,
      getKsmContributorWallet(contributorRewardWallets[1]),
      contributorRewardWallets[10]
    );
    const {
      data: [resultRemoteAccount, resultRewardAccount]
    } = await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(rewardAccount.publicKey, proofMessage)
    );
    // Verification
    await TxCrowdloanRewardsTests.verifyKsmAssociation(
      api,
      resultRemoteAccount,
      resultRewardAccount,
      rewardAccount,
      TEST_WALLET_PICA_REWARD_AMOUNT,
      INITIAL_ASSOCIATE_CLAIM_PERCENT,
      getKsmContributorWallet(contributorRewardWallets[1]).publicKey
    );
  });

  it("1.17  A user can not claim without associating first.", async function () {
    this.timeout(2 * 60 * 1000);
    await sendAndWaitForSuccess(
      api,
      notContributor,
      api.events.crowdloanRewards.Claimed.is,
      api.tx.crowdloanRewards.claim()
    ).catch(function (err) {
      expect(err.toString()).to.contain("crowdloanRewards.NotAssociated");
    });
  });

  it("1.19  A user can not associate with a KSM wallet which isn't a contributor.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = notContributor;
    const proofMessage = getKsmProofMessage(api, getKsmContributorWallet(wallet), wallet);
    await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, proofMessage)
    ).catch(function (err) {
      expect(err.toString()).to.contain("Custom error: 1");
    });
  });

  it("1.20  A user can not associate with a wallet which isn't a contributor.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = notContributor;
    const ethWallet = Wallet.createRandom();
    const proofMessage = await getEthProofMessage(api, ethWallet, wallet);
    await sendUnsignedAndWaitForSuccess(
      api,
      api.events.crowdloanRewards.Associated.is,
      api.tx.crowdloanRewards.associate(wallet.publicKey, proofMessage)
    ).catch(function (err) {
      expect(err.toString()).to.contain("Custom error: 1");
    });
  });

  it("1.21  I can, as sudo, unlock funds for a wallet.", async function () {
    this.timeout(2 * 60 * 1000);
    const wallet = sudoKey;
    const walletsToBeUnlocked = [contributorRewardWallets[0], contributorRewardWallets[1]];
    const publicKeysToBeUnlocked = [walletsToBeUnlocked[0].publicKey, walletsToBeUnlocked[1].publicKey];
    const {
      data: [result]
    } = await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.crowdloanRewards.unlockRewardsFor(publicKeysToBeUnlocked))
    );
    expect(result.isOk).to.be.true;
    await TxCrowdloanRewardsTests.verifyRewardsUnlock(api, walletsToBeUnlocked, sudoKey.publicKey);
  });
});
