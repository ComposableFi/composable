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
const getOracleStake = async (api, wallet) => new bn_js_1.default((await api.query.oracle.oracleStake(wallet.publicKey)).toString());
/**
 * This test suite contains tests for the HAL-01 issue
 * raised by Halborn in the security audit.
 * Audit Date: 19.02.22 - 29.04.22
 *
 * Issue description, Quote:
 * [...]
 * To prevent malicious Oracles from manipulating the asset's price,
 * every proposal which would not be in the acceptable range results
 * in a slash of Oracle balance. However, two scenarios are possible
 * where this mechanism can be exploited.
 *
 * Suppose exactly half of the proposed prices would be malicious,
 * i.e., substantially increasing of decreasing an asset's price.
 * In that case, all Oracles might get slashes, regardless if they
 * submitted a plausible price or not.
 *
 * On the other hand, if most of the proposed prices were malicious,
 * then such a situation would result in legitimate Oracles getting slashed.
 *
 */
describe("HAL01 [Oracle] Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.HAL01)
        return;
    let api;
    let assetID;
    let walletHAL01_1, walletHAL01_2, walletHAL01_3, walletHAL01_4, controllerWallet, sudoKey;
    before("HAL01: Setting up tests", async function () {
        this.timeout(60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice } = (0, walletHelper_1.getDevWallets)(newKeyring);
        sudoKey = devWalletAlice;
        controllerWallet = devWalletAlice.derive("/HAL01/oracleController");
        walletHAL01_1 = devWalletAlice.derive("/HAL01/oracleSigner1");
        walletHAL01_2 = devWalletAlice.derive("/HAL01/oracleSigner2");
        walletHAL01_3 = devWalletAlice.derive("/HAL01/oracleSigner3");
        walletHAL01_4 = devWalletAlice.derive("/HAL01/oracleSigner4");
        assetID = 1001;
    });
    before("HAL01: Providing funds", async function () {
        this.timeout(5 * 60 * 1000);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, controllerWallet, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL01_1, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL01_2, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL01_3, sudoKey, [1, assetID]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletHAL01_4, sudoKey, [1, assetID]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    it("HAL01: Creating oracle", async function () {
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
    describe("HAL01: Setting signers", function () {
        it("HAL01: Setting signer 1", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, controllerWallet, walletHAL01_1).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
        });
        it("HAL01: Setting signer 2", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL01_1, walletHAL01_2).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
        });
        it("HAL01: Setting signer 3", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL01_2, walletHAL01_3).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
        });
        it("HAL01: Setting signer 4", async function () {
            this.timeout(2 * 60 * 1000);
            const { data: [resultAccount0, resultAccount1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL01_3, walletHAL01_4).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use")
                return this.skip(); // If the test is run a second time on the same chain, we already have a signer set.
            (0, chai_1.expect)(resultAccount0).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount1).to.not.be.an("Error");
            (0, chai_1.expect)(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
            (0, chai_1.expect)(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
            // We need to further elect a new signer,
            // else signer 4 won't be able to add its stake.
            const { data: [result2Account0, result2Account1] } = await (0, setSignerTests_1.txOracleSetSignerSuccessTest)(api, walletHAL01_4, controllerWallet).catch(function (exc) {
                return { data: [exc] }; /* We can't call this.skip() from here. */
            });
            (0, chai_1.expect)(result2Account0).to.not.be.an("Error");
            (0, chai_1.expect)(result2Account1).to.not.be.an("Error");
            (0, chai_1.expect)(result2Account0.toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
            (0, chai_1.expect)(result2Account1.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
        });
    });
    describe("HAL01: Adding stakes", function () {
        it("HAL01: Adding stakes", async function () {
            this.timeout(2 * 60 * 1000);
            const stake = api.createType("u128", 25000000000000);
            const [{ data: [result] }, { data: [result2] }, { data: [result3] }, { data: [result4] }] = await Promise.all([
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL01_1, stake),
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL01_2, stake),
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL01_3, stake),
                (0, addStakeTests_1.txOracleAddStakeSuccessTest)(api, walletHAL01_4, stake)
            ]);
            (0, chai_1.expect)(result).to.not.be.an("Error");
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
            (0, chai_1.expect)(result2).to.not.be.an("Error");
            (0, chai_1.expect)(result2.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
            (0, chai_1.expect)(result3).to.not.be.an("Error");
            (0, chai_1.expect)(result3.toString()).to.be.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
            (0, chai_1.expect)(result4).to.not.be.an("Error");
            (0, chai_1.expect)(result4.toString()).to.be.equal(api.createType("AccountId32", controllerWallet.publicKey).toString());
        });
    });
    describe("HAL01: Test Scenarios", function () {
        this.retries(0);
        it("HAL01: Scenario 1: 50% of Oracles are malicious", async function () {
            this.timeout(10 * 60 * 1000);
            const correctPrice = api.createType("u128", 100);
            const maliciousPrice = api.createType("u128", 900);
            const asset = api.createType("u128", assetID);
            const [oracleStakeWallet1BeforeTransaction, oracleStakeWallet2BeforeTransaction, oracleStakeWallet3BeforeTransaction, oracleStakeWallet4BeforeTransaction] = await Promise.all([
                getOracleStake(api, walletHAL01_1),
                getOracleStake(api, walletHAL01_2),
                getOracleStake(api, walletHAL01_3),
                getOracleStake(api, walletHAL01_4)
            ]);
            (0, chai_1.expect)(oracleStakeWallet1BeforeTransaction).to.be.bignumber.greaterThan("0");
            (0, chai_1.expect)(oracleStakeWallet2BeforeTransaction).to.be.bignumber.greaterThan("0");
            (0, chai_1.expect)(oracleStakeWallet3BeforeTransaction).to.be.bignumber.greaterThan("0");
            (0, chai_1.expect)(oracleStakeWallet4BeforeTransaction).to.be.bignumber.greaterThan("0");
            // Submit 2 correct & 2 malicious prices.
            await Promise.all([
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL01_1, correctPrice, asset),
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL01_2, correctPrice, asset),
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL01_3, maliciousPrice, asset),
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL01_4, maliciousPrice, asset)
            ]).then(async function ([{ data: [result1AccountID, result1AssetID, result1ReportedPrice] }, { data: [result2AccountID, result2AssetID, result2ReportedPrice] }, { data: [result3AccountID, result3AssetID, result3ReportedPrice] }, { data: [result4AccountID, result4AssetID, result4ReportedPrice] }]) {
                (0, chai_1.expect)(result1AssetID.toNumber())
                    .to.be.equal(result2AssetID.toNumber())
                    .to.be.equal(result3AssetID.toNumber())
                    .to.be.equal(result4AssetID.toNumber())
                    .to.be.equal(asset.toNumber());
                (0, chai_1.expect)(result1ReportedPrice.toNumber())
                    .to.be.equal(result2ReportedPrice.toNumber())
                    .to.be.equal(correctPrice.toNumber());
                (0, chai_1.expect)(result3ReportedPrice.toNumber())
                    .to.be.equal(result4ReportedPrice.toNumber())
                    .to.be.equal(maliciousPrice.toNumber());
                (0, chai_1.expect)(result1AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
                (0, chai_1.expect)(result2AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_2.publicKey).toString());
                (0, chai_1.expect)(result3AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
                (0, chai_1.expect)(result4AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
                const [oracleStakeWallet1AfterTransaction, oracleStakeWallet2AfterTransaction, oracleStakeWallet3AfterTransaction, oracleStakeWallet4AfterTransaction] = await Promise.all([
                    getOracleStake(api, walletHAL01_1),
                    getOracleStake(api, walletHAL01_2),
                    getOracleStake(api, walletHAL01_3),
                    getOracleStake(api, walletHAL01_4)
                ]);
                // Nobody should get slashed.
                (0, chai_1.expect)(oracleStakeWallet1BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet1AfterTransaction);
                (0, chai_1.expect)(oracleStakeWallet2BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet2AfterTransaction);
                (0, chai_1.expect)(oracleStakeWallet3BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet3AfterTransaction);
                (0, chai_1.expect)(oracleStakeWallet4BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet4AfterTransaction);
            });
        });
        it("HAL01: Scenario 2: >50% of Oracles are malicious", async function () {
            this.timeout(10 * 60 * 1000);
            const correctPrice = api.createType("u128", 100);
            const maliciousPrice = api.createType("u128", 900);
            const asset = api.createType("u128", assetID);
            const [oracleStakeWallet1BeforeTransaction, oracleStakeWallet2BeforeTransaction, oracleStakeWallet3BeforeTransaction, oracleStakeWallet4BeforeTransaction] = await Promise.all([
                getOracleStake(api, walletHAL01_1),
                getOracleStake(api, walletHAL01_2),
                getOracleStake(api, walletHAL01_3),
                getOracleStake(api, walletHAL01_4)
            ]);
            (0, chai_1.expect)(oracleStakeWallet1BeforeTransaction).to.be.bignumber.greaterThan("0");
            (0, chai_1.expect)(oracleStakeWallet2BeforeTransaction).to.be.bignumber.greaterThan("0");
            (0, chai_1.expect)(oracleStakeWallet3BeforeTransaction).to.be.bignumber.greaterThan("0");
            (0, chai_1.expect)(oracleStakeWallet4BeforeTransaction).to.be.bignumber.greaterThan("0");
            await (0, polkadotjs_1.waitForBlocks)(api, 6);
            // Submit 1 correct & 2 malicious prices.
            await Promise.all([
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL01_1, correctPrice, asset),
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL01_3, maliciousPrice, asset),
                (0, submitPriceTests_1.txOracleSubmitPriceSuccessTest)(api, walletHAL01_4, maliciousPrice, asset)
            ]).then(async function ([{ data: [result1AccountID, result1AssetID, result1ReportedPrice] }, { data: [result3AccountID, result3AssetID, result3ReportedPrice] }, { data: [result4AccountID, result4AssetID, result4ReportedPrice] }]) {
                (0, chai_1.expect)(result1AssetID.toNumber())
                    .to.be.equal(result3AssetID.toNumber())
                    .to.be.equal(result4AssetID.toNumber())
                    .to.be.equal(asset.toNumber());
                (0, chai_1.expect)(result1ReportedPrice.toNumber()).to.be.equal(correctPrice.toNumber());
                (0, chai_1.expect)(result3ReportedPrice.toNumber())
                    .to.be.equal(result4ReportedPrice.toNumber())
                    .to.be.equal(maliciousPrice.toNumber());
                (0, chai_1.expect)(result1AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_1.publicKey).toString());
                (0, chai_1.expect)(result3AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_3.publicKey).toString());
                (0, chai_1.expect)(result4AccountID.toString()).to.equal(api.createType("AccountId32", walletHAL01_4.publicKey).toString());
                const [oracleStakeWallet1AfterTransaction, oracleStakeWallet2AfterTransaction, oracleStakeWallet3AfterTransaction, oracleStakeWallet4AfterTransaction] = await Promise.all([
                    getOracleStake(api, walletHAL01_1),
                    getOracleStake(api, walletHAL01_2),
                    getOracleStake(api, walletHAL01_3),
                    getOracleStake(api, walletHAL01_4)
                ]);
                // Nobody should get slashed.
                (0, chai_1.expect)(oracleStakeWallet1BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet1AfterTransaction);
                (0, chai_1.expect)(oracleStakeWallet2BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet2AfterTransaction);
                (0, chai_1.expect)(oracleStakeWallet3BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet3AfterTransaction);
                (0, chai_1.expect)(oracleStakeWallet4BeforeTransaction).to.be.bignumber.equal(oracleStakeWallet4AfterTransaction);
            });
        });
    });
});
//# sourceMappingURL=HAL01-Tests.js.map