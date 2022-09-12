"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const addAssetAndInfoTests_1 = require("@composabletests/tests/oracle/testHandlers/addAssetAndInfoTests");
const setSignerTests_1 = require("@composabletests/tests/oracle/testHandlers/setSignerTests");
const addStakeTests_1 = require("@composabletests/tests/oracle/testHandlers/addStakeTests");
const submitPriceTests_1 = require("@composabletests/tests/oracle/testHandlers/submitPriceTests");
const removeStakeTests_1 = require("@composabletests/tests/oracle/testHandlers/removeStakeTests");
const reclaimStakeTests_1 = require("@composabletests/tests/oracle/testHandlers/reclaimStakeTests");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
/**
 * Contains all TX tests for the pallet:
 * Oracle
 */
describe("tx.oracle Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.enabled)
        return;
    let api;
    let assetsCountStart;
    let newAsset1;
    let signedWallet;
    let controllerWallet;
    before("Setting up the tests", async function () {
        this.timeout(60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice } = (0, walletHelper_1.getDevWallets)(newKeyring);
        // Getting the id for the about to be created asset.
        assetsCountStart = (await api.query.oracle.assetsCount()).toNumber();
        newAsset1 = assetsCountStart + 1;
        signedWallet = devWalletAlice.derive("/oracleSigner");
        controllerWallet = devWalletAlice;
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
        if (!test_configuration_json_1.default.enabledTests.addAssetAndInfo__success.enabled)
            return;
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        it("[SHORT] Can add new asset and info", async function () {
            if (!test_configuration_json_1.default.enabledTests.addAssetAndInfo__success.add1)
                this.skip();
            const assetId = api.createType("u128", newAsset1);
            const threshold = api.createType("Percent", 50);
            const minAnswers = api.createType("u32", 2);
            const maxAnswers = api.createType("u32", 5);
            const blockInterval = api.createType("u32", 6);
            const reward = api.createType("u128", 150000000000);
            const slash = api.createType("u128", 100000000000);
            const { data: [result] } = await (0, addAssetAndInfoTests_1.txOracleAddAssetAndInfoSuccessTest)(api, controllerWallet, assetId, threshold, minAnswers, maxAnswers, blockInterval, reward, slash);
            if (result.isErr)
                console.debug(result.asErr.toString());
            (0, chai_1.expect)(result.isOk).to.be.true;
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
        if (!test_configuration_json_1.default.enabledTests.setSigner__success.enabled)
            return;
        // Timeout set to 4 minutes
        this.timeout(4 * 60 * 1000);
        it("Can set signer", async function () {
            if (!test_configuration_json_1.default.enabledTests.setSigner__success.set1)
                this.skip();
            await (0, setSignerTests_1.runBeforeTxOracleSetSigner)(api, controllerWallet, signedWallet); // Making sure we have funds.
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, controllerWallet, signedWallet).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
                resultAccount0.message == "oracle.ControllerUsed: This controller is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", signedWallet.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
        });
    });
    /**
     * oracle.addStake Success Tests
     * To be called by controller.
     *
     * Result is the signer wallets public key.
     */
    describe("tx.addStake Success Test", function () {
        if (!test_configuration_json_1.default.enabledTests.addStake__success.enabled)
            return;
        // Timeout set to 4 minutes
        this.timeout(4 * 60 * 1000);
        it("Can add stake from creator/controller", async function () {
            if (!test_configuration_json_1.default.enabledTests.addStake__success.add1)
                this.skip();
            await (0, addStakeTests_1.runBeforeTxOracleAddStake)(api, controllerWallet, controllerWallet, signedWallet); // Preparing the signer to have funds.
            const stake = api.createType("u128", 250000000000);
            const { data: [result] } = await (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, controllerWallet, stake);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", signedWallet.publicKey).toString());
        });
    });
    /**
     * oracle.submitPrice Success Tests
     * To be called by signer or controller.
     *
     * Result is the signer wallets public key.
     */
    describe("tx.submitPrice Success Test", function () {
        if (!test_configuration_json_1.default.enabledTests.submitPrice__success.enabled)
            return;
        // Timeout set to 4 minutes
        this.timeout(4 * 60 * 1000);
        it("Can submit new price by signer", async function () {
            if (!test_configuration_json_1.default.enabledTests.submitPrice__success.submit1)
                this.skip();
            const price = api.createType("u128", 10000);
            const assetId = api.createType("u128", newAsset1);
            const { data: [result] } = await (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, signedWallet, price, assetId);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", signedWallet.publicKey).toString());
        });
    });
    /**
     * oracle.removeStake Success Tests
     * To be called by controller.
     *
     * Result is the signer wallets public key.
     */
    describe("tx.removeStake Success Test", function () {
        if (!test_configuration_json_1.default.enabledTests.removeStake__success.enabled)
            return;
        // Timeout set to 2 minutes
        this.timeout(2 * 60 * 1000);
        it("Can remove stake", async function () {
            if (!test_configuration_json_1.default.enabledTests.removeStake__success.remove1)
                this.skip();
            const { data: [result] } = await (0, removeStakeTests_1.txOracleRemoveStakeSuccessTest)(api, controllerWallet);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", signedWallet.publicKey).toString());
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
        if (!test_configuration_json_1.default.enabledTests.reclaimStake__success.enabled)
            return;
        let unlockBlock;
        // Timeout set to 20 minutes
        this.timeout(20 * 60 * 1000);
        this.slow(1200000);
        it("Can reclaim stake", async function () {
            if (!test_configuration_json_1.default.enabledTests.reclaimStake__success.reclaim1)
                this.skip();
            // Get the block number at which the funds are unlocked.
            const declaredWithdrawsResult = await api.query.oracle.declaredWithdraws(signedWallet.address);
            unlockBlock = declaredWithdrawsResult.unwrap().unlockBlock;
            (0, chai_1.expect)(unlockBlock.toNumber()).to.be.a("Number");
            const currentBlock = await api.query.system.number();
            (0, chai_1.expect)(currentBlock.toNumber()).to.be.a("Number");
            // Taking a nap until we reach the unlocking block.
            await (0, polkadotjs_1.waitForBlocks)(api, unlockBlock.toNumber() - currentBlock.toNumber());
            const { data: [result] } = await (0, reclaimStakeTests_1.txOracleReclaimStakeSuccessTest)(api, controllerWallet);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", signedWallet.publicKey).toString());
        });
    });
});
//# sourceMappingURL=txOracleTests.js.map