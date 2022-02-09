/* eslint-disable no-trailing-spaces */
import {expect} from "chai";
import {txOracleSubmitPriceSuccessTest} from "@composable/tests/tx/oracle/testHandlers/submitPriceTests";
import {txOracleAddAssetAndInfoSuccessTest} from "@composable/tests/tx/oracle/testHandlers/addAssetAndInfoTests";
import {
  runBeforeTxOracleSetSigner,
  txOracleSetSignerSuccessTest
} from "@composable/tests/tx/oracle/testHandlers/setSignerTests";
import {KeyringPair} from "@polkadot/keyring/types";
import {
  runBeforeTxOracleAddStake,
  txOracleAddStakeSuccessTest
} from "@composable/tests/tx/oracle/testHandlers/addStakeTests";
import {txOracleReclaimStakeSuccessTest} from "@composable/tests/tx/oracle/testHandlers/reclaimStakeTests";
import {txOracleRemoveStakeSuccessTest} from "@composable/tests/tx/oracle/testHandlers/removeStakeTests";

/**
 * Contains all TX tests for the pallet:
 * Oracle
 */
function sleep(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

export class TxOracleTests {
  /**
   * Runs all tx tests for the Oracle pallet.
   */
  public static runTxOracleTests() {
    let assetsCountStart:number;
    let newAsset1:number;
    let signedWallet: KeyringPair;
    let controllerWallet: KeyringPair;

    describe('tx.oracle Tests', function () {
      before(async function() {
        // Getting the id for the about to be created asset.
        assetsCountStart = (await api.query.oracle.assetsCount()).toNumber();
        newAsset1=assetsCountStart+1;

        signedWallet = walletAlice.derive('/oracleSigner');
        controllerWallet = walletAlice;
      });
      /**
       * oracle.addAssetAndInfo Success Tests
       *
       * Sudo command success is checked with `.isOk`.
       */
      describe('tx.addAssetAndInfo Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        it('Can add new asset and info', async function () {
          const assetId = api.createType('u128', newAsset1);
          const threshold = api.createType('Percent', 50);
          const minAnswers = api.createType('u32', 2);
          const maxAnswers = api.createType('u32', 5);
          const blockInterval = api.createType('u32', 6);
          const reward = api.createType('u128', 150000000000);
          const slash = api.createType('u128', 100000000000);
          const {data: [result],} = await txOracleAddAssetAndInfoSuccessTest(
            controllerWallet,
            assetId,
            threshold,
            minAnswers,
            maxAnswers,
            blockInterval,
            reward,
            slash
          );
          if (result.isErr)
            console.debug(result.asErr.toString());
          expect(result.isOk).to.be.true;
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
      describe('tx.setSigner Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        before(async function() {
          const sudoKey = walletAlice;
          const {data: [result],} = await runBeforeTxOracleSetSigner(sudoKey, signedWallet);
          expect(result.isOk).to.be.true;
        });
        it('Can set signer', async function () {
          const {data: [resultAccount0, resultAccount1],} = await txOracleSetSignerSuccessTest(controllerWallet, signedWallet).catch(function(exc) {
            return {data:[exc]}; // We can't call this.skip() from here.
          });
          if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
              resultAccount0.message == "oracle.ControllerUsed: This controller is already in use")
            return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
          expect(resultAccount0).to.not.be.an('Error');
          expect(resultAccount1).to.not.be.an('Error');
          expect(resultAccount0.toString()).to.be.equal(api.createType('AccountId32', signedWallet.publicKey).toString());
          expect(resultAccount1.toString()).to.be.equal(api.createType('AccountId32', controllerWallet.publicKey).toString());
        });
      });

      /**
       * oracle.addStake Success Tests
       * To be called by controller.
       *
       * Result is the signer wallets public key.
       */
      describe('tx.addStake Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        before(async function() {
          const sudoKey = walletAlice;
          await runBeforeTxOracleAddStake(sudoKey, controllerWallet, signedWallet);
        });
        it('Can add stake from creator/controller', async function () {
          const stake = api.createType('u128', 250000000000);
          const {data: [result],} = await txOracleAddStakeSuccessTest(controllerWallet, stake);
          expect(result).to.not.be.an('Error');
          expect(result.toString()).to.be
            .equal(api.createType('AccountId32', signedWallet.publicKey).toString());
        });
      });

      /**
       * oracle.submitPrice Success Tests
       * To be called by signer or controller.
       *
       * Result is the signer wallets public key.
       */
      describe('tx.submitPrice Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        it('Can submit new price by signer', async function () {
          const price = api.createType('u128', 10000);
          const assetId = api.createType('u128', newAsset1);
          const {data: [result],} = await txOracleSubmitPriceSuccessTest(signedWallet, price, assetId);
          expect(result).to.not.be.an('Error');
          expect(result.toString()).to.be
            .equal(api.createType('AccountId32', signedWallet.publicKey).toString());
        });
      });

      /**
       * oracle.removeStake Success Tests
       * To be called by controller.
       *
       * Result is the signer wallets public key.
       */
      describe('tx.removeStake Success Test', function () {
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        it('Can remove stake', async function () {
          const controllerWallet = walletAlice;
          const {data: [result],} = await txOracleRemoveStakeSuccessTest(controllerWallet);
          expect(result).to.not.be.an('Error');
          expect(result.toString()).to.be
            .equal(api.createType('AccountId32', signedWallet.publicKey).toString());
        });
      });

      /**
       * oracle.reclaimStake Success Tests
       * To be called by controller.
       * Can only work after a successful removeStake(), and waiting for unblockBlock to be reached.
       *
       * Result is the signer wallets public key.
       */
      describe('tx.reclaimStake Success Test', function () {
        let unlockBlock;
        // Timeout set to 15 minutes
        this.timeout(15 * 60 * 1000);
        before(async function() {
          // Get the block number at which the funds are unlocked.
          const result = await api.query.oracle.declaredWithdraws(signedWallet.address);
          unlockBlock = result.unwrap().unlockBlock;
          expect(unlockBlock.toNumber()).to.be.a('Number');
        });

        it('Can reclaim stake', async function () {
          let currentBlock = await api.query.system.number();
          // Taking a nap until we reach the unlocking block.
          while (unlockBlock.toNumber() >= currentBlock.toNumber()) {
            await sleep(9000);
            currentBlock = await api.query.system.number();
          }
          const controllerWallet = walletAlice; // Controller
          const {data: [result],} = await txOracleReclaimStakeSuccessTest(controllerWallet);
          expect(result).to.not.be.an('Error');
          expect(result.toString()).to.be
            .equal(api.createType('AccountId32', signedWallet.publicKey).toString());
        });
      });
    });
  }
}

// Uncomment to debug
//TxOracleTests.runTxOracleTests();
