import { expect } from "chai";
import { KeyringPair } from "@polkadot/keyring/types";
import testConfiguration from "./test_configuration.json";
import {
  txOracleAddAssetAndInfoSuccessTest,
  verifyOracleCreation
} from "@composabletests/tests/oracle/testHandlers/addAssetAndInfoTests";
import { txOracleSetSignerSuccessTest } from "@composabletests/tests/oracle/testHandlers/setSignerTests";
import { txOracleAddStakeSuccessTest } from "@composabletests/tests/oracle/testHandlers/addStakeTests";
import { txOracleSubmitPriceSuccessTestHandler } from "@composabletests/tests/oracle/testHandlers/submitPriceTests";
import { txOracleRemoveStakeSuccessTest } from "@composabletests/tests/oracle/testHandlers/removeStakeTests";
import { txOracleReclaimStakeSuccessTest } from "@composabletests/tests/oracle/testHandlers/reclaimStakeTests";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import BN from "bn.js";

/**
 * Contains all TX tests for the pallet:
 * Oracle
 */
describe("[LAUNCH] Oracle Tests", function () {
  if (!testConfiguration.enabledTests.enabled) return;
  this.retries(0);

  let api: ApiPromise;
  let assetsCountStart: number;
  let newAsset1: number;
  let signerWallet1: KeyringPair, signerWallet2: KeyringPair, signerWallet3: KeyringPair, signerWallet4: KeyringPair;
  let controllerWallet: KeyringPair, sudoKey: KeyringPair;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice } = getDevWallets(newKeyring);
    // Getting the id for the about to be created asset.
    assetsCountStart = (await api.query.oracle.assetsCount()).toNumber();
    newAsset1 = assetsCountStart + 1;

    signerWallet1 = devWalletAlice.derive("/tests/oracle/signer1");
    signerWallet2 = devWalletAlice.derive("/tests/oracle/signer2");
    signerWallet3 = devWalletAlice.derive("/tests/oracle/signer3");
    signerWallet4 = devWalletAlice.derive("/tests/oracle/signer4");
    controllerWallet = devWalletAlice;
    sudoKey = devWalletAlice;
  });

  before("Providing funds", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, controllerWallet, sudoKey, [1, newAsset1]);
    await mintAssetsToWallet(api, signerWallet1, sudoKey, [1, newAsset1]);
    await mintAssetsToWallet(api, signerWallet2, sudoKey, [1, newAsset1]);
    await mintAssetsToWallet(api, signerWallet3, sudoKey, [1, newAsset1]);
    await mintAssetsToWallet(api, signerWallet4, sudoKey, [1, newAsset1]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  /**
   * oracle.addAssetAndInfo Success Tests
   *
   * Sudo command success is checked with `.isOk`.
   */
  describe("tx.addAssetAndInfo Success Test", function () {
    if (!testConfiguration.enabledTests.addAssetAndInfo__success.enabled) return;
    // Timeout set to 2 minutes
    this.timeout(2 * 60 * 1000);
    it("[SHORT] Can add new asset and info", async function () {
      if (!testConfiguration.enabledTests.addAssetAndInfo__success.add1) this.skip();
      const assetId = api.createType("u128", newAsset1);
      const threshold = api.createType("Percent", 70);
      const minAnswers = api.createType("u32", 3);
      const maxAnswers = api.createType("u32", 5);
      const blockInterval = api.createType("u32", 6);
      const rewardWeight = api.createType("u128", 150000000000);
      const slash = api.createType("u128", 100000000000);
      const data = await txOracleAddAssetAndInfoSuccessTest(
        api,
        controllerWallet,
        assetId,
        threshold,
        minAnswers,
        maxAnswers,
        blockInterval,
        rewardWeight,
        slash,
        true
      );
      await verifyOracleCreation(api, data, { threshold, minAnswers, maxAnswers, blockInterval, rewardWeight, slash });
    });
  });

  /**
   * oracle.setSigner Success Tests
   * To be called by controller.
   *
   * In `before` we give the signer wallet enough funds to become a signer.
   *
   * We get 2 results here.
   * resultAccount0: The signer wallets public key.
   * resultAccount1: The controller wallets public key.
   */
  describe("tx.setSigner Success Test", function () {
    if (!testConfiguration.enabledTests.setSigner__success.enabled) return;
    // Timeout set to 4 minutes
    this.timeout(4 * 60 * 1000);
    it("Can set signer #1", async function () {
      if (!testConfiguration.enabledTests.setSigner__success.set1) this.skip();

      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, controllerWallet, signerWallet1).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (
        resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
        resultAccount0.message == "oracle.ControllerUsed: This controller is already in use"
      )
        return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", signerWallet1.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );
      const signerToControllerResultWrapped = await api.query.oracle.signerToController(signerWallet1.publicKey);
      const signerToControllerResult = signerToControllerResultWrapped.unwrap();
      expect(signerToControllerResult.toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );
    });
    it("Can set signer #2", async function () {
      if (!testConfiguration.enabledTests.setSigner__success.set1) this.skip();

      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, signerWallet1, signerWallet2).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (
        resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
        resultAccount0.message == "oracle.ControllerUsed: This controller is already in use"
      )
        return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", signerWallet2.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", signerWallet1.publicKey).toString());
      const signerToControllerResultWrapped = await api.query.oracle.signerToController(signerWallet2.publicKey);
      const signerToControllerResult = signerToControllerResultWrapped.unwrap();
      expect(signerToControllerResult.toString()).to.be.equal(
        api.createType("AccountId32", signerWallet1.publicKey).toString()
      );
    });
    it("Can set signer #3", async function () {
      if (!testConfiguration.enabledTests.setSigner__success.set1) this.skip();

      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, signerWallet2, signerWallet3).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (
        resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
        resultAccount0.message == "oracle.ControllerUsed: This controller is already in use"
      )
        return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", signerWallet3.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", signerWallet2.publicKey).toString());
      const signerToControllerResultWrapped = await api.query.oracle.signerToController(signerWallet3.publicKey);
      const signerToControllerResult = signerToControllerResultWrapped.unwrap();
      expect(signerToControllerResult.toString()).to.be.equal(
        api.createType("AccountId32", signerWallet2.publicKey).toString()
      );
    });
    it("Can set signer #4", async function () {
      if (!testConfiguration.enabledTests.setSigner__success.set1) this.skip();

      const {
        data: [resultAccount0, resultAccount1]
      } = await txOracleSetSignerSuccessTest(api, signerWallet3, signerWallet4).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (
        resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
        resultAccount0.message == "oracle.ControllerUsed: This controller is already in use"
      )
        return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", signerWallet4.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", signerWallet3.publicKey).toString());
      const signerToControllerResultWrapped = await api.query.oracle.signerToController(signerWallet4.publicKey);
      const signerToControllerResult = signerToControllerResultWrapped.unwrap();
      expect(signerToControllerResult.toString()).to.be.equal(
        api.createType("AccountId32", signerWallet3.publicKey).toString()
      );

      // Setting back else signer can not add stake
      const {
        data: [result2Account0, result2Account1]
      } = await txOracleSetSignerSuccessTest(api, signerWallet4, controllerWallet).catch(function (exc) {
        return { data: [exc] }; /* We can't call this.skip() from here. */
      });

      if (
        result2Account0.message == "oracle.SignerUsed: This signer is already in use" ||
        result2Account0.message == "oracle.ControllerUsed: This controller is already in use"
      )
        return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
      expect(result2Account0.toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );
      expect(result2Account1.toString()).to.be.equal(api.createType("AccountId32", signerWallet4.publicKey).toString());
      const signerToControllerResult2Wrapped = await api.query.oracle.signerToController(controllerWallet.publicKey);
      const signerToControllerResult2 = signerToControllerResult2Wrapped.unwrap();
      expect(signerToControllerResult2.toString()).to.be.equal(
        api.createType("AccountId32", signerWallet4.publicKey).toString()
      );
    });
  });

  /**
   * oracle.addStake Success Tests
   * To be called by controller.
   *
   * Result is the signer wallets public key.
   */
  describe("tx.addStake Success Test", function () {
    if (!testConfiguration.enabledTests.addStake__success.enabled) return;
    // Timeout set to 4 minutes
    this.timeout(4 * 60 * 1000);
    it("Signers & controller can add their stake", async function () {
      if (!testConfiguration.enabledTests.addStake__success.add1) this.skip();
      const stake = api.createType("u128", 500000000000000);
      const [result1, result2, result3, result4, result5] = await Promise.all([
        txOracleAddStakeSuccessTest(api, controllerWallet, stake),
        txOracleAddStakeSuccessTest(api, signerWallet1, stake),
        txOracleAddStakeSuccessTest(api, signerWallet2, stake),
        txOracleAddStakeSuccessTest(api, signerWallet3, stake),
        txOracleAddStakeSuccessTest(api, signerWallet4, stake)
      ]);
      expect(result1.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet1.publicKey).toString());
      expect(result2.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet2.publicKey).toString());
      expect(result3.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet3.publicKey).toString());
      expect(result4.data[0].toString()).to.be.equal(api.createType("AccountId32", signerWallet4.publicKey).toString());
      expect(result5.data[0].toString()).to.be.equal(
        api.createType("AccountId32", controllerWallet.publicKey).toString()
      );

      expect(stake)
        .to.be.bignumber.equal(result1.data[1])
        .to.be.bignumber.equal(result2.data[1])
        .to.be.bignumber.equal(result3.data[1])
        .to.be.bignumber.equal(result4.data[1])
        .to.be.bignumber.equal(result5.data[1]);
    });
  });

  /**
   * oracle.submitPrice Success Tests
   * To be called by signer or controller.
   *
   * Result is the signer wallets public key.
   *
   * For the chain to report priceChanged events,
   * we'll need to have a price first.
   * Therefore, we submit some prices and wait a few blockes,
   * before starting the actual price submission test.
   */
  describe("tx.submitPrice Success Test", function () {
    if (!testConfiguration.enabledTests.submitPrice_before_adjustments__success.enabled) return;

    before("Preparing submit price to work", async function () {
      // Timeout set to 4 minutes
      this.timeout(10 * 60 * 1000);
      const price = 100_000;
      await txOracleSubmitPriceSuccessTestHandler(
        api,
        controllerWallet,
        signerWallet1,
        signerWallet2,
        signerWallet3,
        signerWallet4,
        newAsset1,
        price
      );
      console.info("        Waiting a few blocks to get a new oracle price request...");
      await waitForBlocks(api, 12);
    });

    it("Can submit new price by signers with a slashable price", async function () {
      if (!testConfiguration.enabledTests.submitPrice_before_adjustments__success.submit1) this.skip();
      // Timeout set to 4 minutes
      this.timeout(10 * 60 * 1000);
      const price = 10_000_000;
      const res = await txOracleSubmitPriceSuccessTestHandler(
        api,
        controllerWallet,
        signerWallet1,
        signerWallet2,
        signerWallet3,
        signerWallet4,
        newAsset1,
        price,
        true,
        true
      );
      expect(res).to.not.be.an("Error");
    });

    it("Can submit new price by signers without a slashable price", async function () {
      if (!testConfiguration.enabledTests.submitPrice_before_adjustments__success.submit1) this.skip();
      // Timeout set to 4 minutes
      this.timeout(10 * 60 * 1000);
      await waitForBlocks(api, 6);
      const price = 1_000_000;
      const res = await txOracleSubmitPriceSuccessTestHandler(
        api,
        controllerWallet,
        signerWallet1,
        signerWallet2,
        signerWallet3,
        signerWallet4,
        newAsset1,
        price,
        false,
        true
      );
      expect(res).to.not.be.an("Error");
    });
  });

  /**
   * oracle.adjustRewards Success Tests
   * To be called by sudo.
   *
   * Adjusts the oracle rewards.
   * ToDo: Find good way to test!
   */
  describe("tx.adjustRewards Success Test", function () {
    if (!testConfiguration.enabledTests.adjustRewards__success.enabled) return;
    // Timeout set to 2 minutes
    this.timeout(10 * 60 * 1000);
    it("Can adjust oracle inflation", async function () {
      if (!testConfiguration.enabledTests.adjustRewards__success.adjust1) this.skip();
      const annualCostPerOracle = 100_000_000_000;
      const numIdealOracles = 5;
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        controllerWallet,
        api.events.oracle.RewardingAdjustment.is,
        api.tx.sudo.sudo(api.tx.oracle.adjustRewards(annualCostPerOracle, numIdealOracles))
      );
      expect(result).to.not.be.an("Error");
    });
  });

  /**
   * oracle.removeStake Success Tests
   * To be called by controller.
   *
   * Result is the signer wallets public key.
   */
  describe("tx.removeStake Success Test", function () {
    if (!testConfiguration.enabledTests.removeStake__success.enabled) return;
    // Timeout set to 2 minutes
    this.timeout(2 * 60 * 1000);
    it("Can remove stakes", async function () {
      if (!testConfiguration.enabledTests.removeStake__success.remove1) this.skip();
      const {
        data: [result]
      } = await txOracleRemoveStakeSuccessTest(api, controllerWallet);
      expect(result.toString()).to.be.equal(api.createType("AccountId32", signerWallet1.publicKey).toString());
    });
  });

  /**
   * oracle.reclaimStake Success Tests
   * To be called by controller.
   * Can only work after a successful removeStake(), and waiting for unlockBlock to be reached.
   *
   * Result is the signer wallets public key.
   */
  describe("tx.reclaimStake Success Test", function () {
    if (!testConfiguration.enabledTests.reclaimStake__success.enabled) return;
    let unlockBlock;
    // Timeout set to 20 minutes
    this.timeout(20 * 60 * 1000);
    this.slow(15 * 60 * 1000);
    it("Can reclaim stake", async function () {
      this.skip();
      if (!testConfiguration.enabledTests.reclaimStake__success.reclaim1) this.skip();
      // Get the block number at which the funds are unlocked.
      const declaredWithdrawsResult = await api.query.oracle.declaredWithdraws(signerWallet1.address);
      unlockBlock = declaredWithdrawsResult.unwrap().unlockBlock;
      expect(unlockBlock.toNumber()).to.be.a("Number");
      const currentBlock = await api.query.system.number();
      expect(currentBlock.toNumber()).to.be.a("Number");
      // Taking a nap until we reach the unlocking block.
      await waitForBlocks(api, unlockBlock.toNumber() - currentBlock.toNumber());
      const walletFundsBefore = await api.rpc.assets.balanceOf("1", controllerWallet.publicKey);
      const {
        data: [result]
      } = await txOracleReclaimStakeSuccessTest(api, controllerWallet);
      expect(result.toString()).to.be.equal(api.createType("AccountId32", signerWallet1.publicKey).toString());
      const walletFundsAfter = await api.rpc.assets.balanceOf("1", controllerWallet.publicKey);
      expect(new BN(walletFundsAfter.toString())).to.be.bignumber.greaterThan(new BN(walletFundsBefore.toString()));
    });
  });
});
