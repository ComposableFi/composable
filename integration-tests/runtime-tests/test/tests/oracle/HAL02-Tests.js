"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const addAssetAndInfoTests_1 = require("./testHandlers/addAssetAndInfoTests");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
const chai_1 = require("chai");
const setSignerTests_1 = require("./testHandlers/setSignerTests");
const mintingHelper_1 = require("@composable/utils/mintingHelper");
const addStakeTests_1 = require("./testHandlers/addStakeTests");
const submitPriceTests_1 = require("./testHandlers/submitPriceTests");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const bn_js_1 = __importDefault(require("bn.js"));
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
/**
 * This test suite contains tests for the HAL-02 issue
 * raised by Halborn in the security audit.
 * Audit Date: 19.02.22 - 29.04.22
 *
 * Issue description, Quote:
 * [...]
 * If the proposed price is not in the valid range from the newly chosen price (defined per asset),
 * Oracle, who submitted that price, would lose a portion of its tokens.
 *
 * However, the tokens are not subtracted from the staked balance but the free balance.
 * If there is no free balance in the user's account, slash would not be completed.
 *
 * For example a malicious Oracle might stake all of its tokens. Then Oracle
 * might send an invalid price proposal, manipulating the market. In such
 * a scenario, an Oracle pallet would not be able to punish the malicious Oracle,
 * who then may unstake the tokens and receive the initially staked tokens without penalties.
 *
 */
describe("HAL02 [Oracle] Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.HAL02)
        return;
    let api;
    let assetID;
    let walletHAL02_1, walletHAL02_2, walletHAL02_3, walletHAL02_4, controllerWallet, sudoKey;
    before("HAL02: Setting up tests", async function () {
        this.timeout(60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice } = (0, walletHelper_1.getDevWallets)(newKeyring);
        sudoKey = devWalletAlice;
        controllerWallet = devWalletAlice.derive("/HAL02/oracleController");
        walletHAL02_1 = devWalletAlice.derive("/HAL02/oracleSigner1");
        walletHAL02_2 = devWalletAlice.derive("/HAL02/oracleSigner2");
        walletHAL02_3 = devWalletAlice.derive("/HAL02/oracleSigner3");
        walletHAL02_4 = devWalletAlice.derive("/HAL02/oracleSigner4");
        assetID = 1000;
    });
    before("HAL02: Providing funds", async function () {
        this.timeout(5 * 60 * 1000);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, controllerWallet, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL02_1, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL02_2, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL02_3, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL02_4, sudoKey, [1, assetID]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    it("HAL02: Creating oracle", async function () {
        this.timeout(2 * 60 * 1000);
        const assetId = api.createType("u128", assetID);
        const threshold = api.createType("Percent", 80);
        const minAnswers = api.createType("u32", 3);
        const maxAnswers = api.createType("u32", 5);
        const blockInterval = api.createType("u32", 6);
        const reward = api.createType("u128", 150000000);
        const slash = api.createType("u128", 100000000);
        const { data: [result] } = await (0, addAssetAndInfoTests_1.txOracleAddAssetAndInfoSuccessTest)(api, sudoKey, assetId, threshold, minAnswers, maxAnswers, blockInterval, reward, slash);
        (0, chai_1.expect)(result.isOk).to.be.true;
    });
    describe("HAL02: Setting signers", function () {
        it("HAL02: Setting signer 1", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, controllerWallet, walletHAL02_1).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_1.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
        });
        it("HAL02: Setting signer 2", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL02_1, walletHAL02_2).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_1.publicKey).toString());
        });
        it("HAL02: Setting signer 3", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL02_2, walletHAL02_3).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_3.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
        });
        it("HAL02: Setting signer 4", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL02_3, walletHAL02_4).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_4.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_3.publicKey).toString());
            // We need to further elect a new signer,
            // else signer 4 won't be able to add its stake.
            const { data: [result2Account0, result2Account1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL02_4, controllerWallet).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            (0, chai_1.expect)(result2Account0).to.not.be.an("Error");
            (0, chai_1.expect)(result2Account1).to.not.be.an("Error");
            (0, chai_1.expect)(result2Account0.toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
            (0, chai_1.expect)(result2Account1.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_4.publicKey).toString());
        });
    });
    describe("HAL02: Adding stakes", function () {
        it("HAL02: Adding stakes", async function () {
            this.timeout(2 * 60 * 1000);
            const stake = api.createType("u128", 2500000000000);
            const [{ data: [result] }] = await Promise.all([
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL02_1, stake),
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL02_2, stake),
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL02_3, stake),
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL02_4, stake)
            ]);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
        });
    });
    describe("HAL02: Test Scenarios", function () {
        this.retries(0);
        it("HAL02: Scenario 1: Oracle stake of malicious actor should get slashed", async function () {
            this.timeout(10 * 60 * 1000);
            const correctPrice = api.createType("u128", 100);
            const maliciousPrice = api.createType("u128", 900);
            const asset = api.createType("u128", assetID);
            const balanceWallet1BeforeTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_1.publicKey)).toString());
            const balanceWallet2BeforeTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_2.publicKey)).toString());
            const balanceWallet3BeforeTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_3.publicKey)).toString());
            const balanceWallet4BeforeTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_4.publicKey)).toString());
            const oracleStakeWallet1BeforeTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_1.publicKey)).toString());
            const oracleStakeWallet2BeforeTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_2.publicKey)).toString());
            const oracleStakeWallet3BeforeTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_3.publicKey)).toString());
            const oracleStakeWallet4BeforeTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_4.publicKey)).toString());
            // Submit 2 correct & 2 malicious prices.
            await Promise.all([
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL02_1, correctPrice, asset),
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL02_2, correctPrice, asset),
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL02_3, maliciousPrice, asset)
            ]).then(async function ([{ data: [result1AccountID, result1AssetID, result1ReportedPrice] }, { data: [result2AccountID, result2AssetID, result2ReportedPrice] }, { data: [result3AccountID, result3AssetID, result3ReportedPrice] }]) {
                (0, chai_1.expect)(result1AssetID.toNumber())
                    .to.be.equal(result2AssetID.toNumber())
                    .to.be.equal(result3AssetID.toNumber())
                    .to.be.equal(asset.toNumber());
                (0, chai_1.expect)(result1ReportedPrice.toNumber())
                    .to.be.equal(result2ReportedPrice.toNumber())
                    .to.be.equal(correctPrice.toNumber());
                (0, chai_1.expect)(result3ReportedPrice.toNumber()).to.be.equal(maliciousPrice.toNumber());
                (0, chai_1.expect)(result1AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL02_1.publicKey).toString());
                (0, chai_1.expect)(result2AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL02_2.publicKey).toString());
                (0, chai_1.expect)(result3AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL02_3.publicKey).toString());
                // Waiting a few blocks to make sure the slashing took place.
                await (0, polkadotjs_1.waitForBlocks)(api, 3);
                const balanceWallet1AfterTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_1.publicKey)).toString());
                const balanceWallet2AfterTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_2.publicKey)).toString());
                const balanceWallet3AfterTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_3.publicKey)).toString());
                const balanceWallet4AfterTransaction = new bn_js_1.default((await api.rpc.assets.balanceOf(asset.toString(), walletHAL02_4.publicKey)).toString());
                const oracleStakeWallet1AfterTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_1.publicKey)).toString());
                const oracleStakeWallet2AfterTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_2.publicKey)).toString());
                const oracleStakeWallet3AfterTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_3.publicKey)).toString());
                const oracleStakeWallet4AfterTransaction = new bn_js_1.default((await api.query.oracle.oracleStake(walletHAL02_4.publicKey)).toString());
                // Malicious price providers oracle stash should get slashed.
                (0, chai_1.expect)(oracleStakeWallet3AfterTransaction).to.be.bignumber.lessThan(oracleStakeWallet3BeforeTransaction);
                // The other ones shouldn't.
                (0, chai_1.expect)(oracleStakeWallet1AfterTransaction).to.be.bignumber.equal(oracleStakeWallet1BeforeTransaction);
                (0, chai_1.expect)(oracleStakeWallet2AfterTransaction).to.be.bignumber.equal(oracleStakeWallet2BeforeTransaction);
                (0, chai_1.expect)(oracleStakeWallet4AfterTransaction).to.be.bignumber.equal(oracleStakeWallet4BeforeTransaction);
                // Wallet Balances shouldn't get slashed.
                (0, chai_1.expect)(balanceWallet1BeforeTransaction).to.be.bignumber.equal(balanceWallet1AfterTransaction);
                (0, chai_1.expect)(balanceWallet2BeforeTransaction).to.be.bignumber.equal(balanceWallet2AfterTransaction);
                (0, chai_1.expect)(balanceWallet3BeforeTransaction).to.be.bignumber.equal(balanceWallet3AfterTransaction);
                (0, chai_1.expect)(balanceWallet4BeforeTransaction).to.be.bignumber.equal(balanceWallet4AfterTransaction);
            });
        });
    });
});
//# sourceMappingURL=HAL02-Tests.js.map