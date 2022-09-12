"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.TxGovernanceRegistryTests = void 0;
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const mintingHelper_1 = require("@composable/utils/mintingHelper");
/**
 * Governance Registry Extrinsic Tests
 *
 * 1. Create governance asset.
 * 2. Remove governance asset.
 * 3. Set root for governance asset.
 */
describe("tx.governanceRegistry Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.tx.enabled)
        return;
    let api;
    let walletAlice, assetSigner;
    let assetID;
    before("Setting up the tests", async function () {
        this.timeout(2 * 60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice } = (0, walletHelper_1.getDevWallets)(newKeyring);
        walletAlice = devWalletAlice;
        assetSigner = walletAlice.derive("/governanceRegistry/signer");
        assetID = api.createType("u128", 1000);
    });
    before("Providing funds", async function () {
        this.timeout(2 * 60 * 1000);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, assetSigner, walletAlice, [assetID.toNumber()]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    describe("tx.governanceRegistry.set Tests", function () {
        this.timeout(2 * 60 * 1000);
        it("Setting assets governance wallet", async function () {
            if (!test_configuration_json_1.default.enabledTests.tx.set__success)
                this.skip();
            const value = assetSigner.publicKey;
            const { data: [result] } = await TxGovernanceRegistryTests.setAsset(api, walletAlice, assetID, value);
            (0, chai_1.expect)(result.isOk).to.be.true;
            const queryResult = await api.query.governanceRegistry.originsByAssetId(assetID);
            (0, chai_1.expect)(queryResult.unwrap().isSigned).to.be.true;
        });
    });
    describe("tx.governanceRegistry.remove Tests", function () {
        this.timeout(2 * 60 * 1000);
        it("Removing governance asset", async function () {
            if (!test_configuration_json_1.default.enabledTests.tx.remove__success)
                this.skip();
            const { data: [result] } = await TxGovernanceRegistryTests.removeAsset(api, walletAlice, assetID);
            (0, chai_1.expect)(result.isOk).to.be.true;
            const queryResult = await api.query.governanceRegistry.originsByAssetId(assetID);
            (0, chai_1.expect)(queryResult.isNone).to.be.true;
        });
    });
    describe("tx.governanceRegistry.grantRoot Tests", function () {
        this.timeout(2 * 60 * 1000);
        it("Grant root for governance asset", async function () {
            if (!test_configuration_json_1.default.enabledTests.tx.remove__success)
                this.skip();
            const { data: [result] } = await TxGovernanceRegistryTests.grantRoot(api, walletAlice, assetID);
            (0, chai_1.expect)(result.isOk).to.be.true;
            const queryResult = await api.query.governanceRegistry.originsByAssetId(assetID);
            (0, chai_1.expect)(queryResult.unwrap().toString()).to.be.equal("Root");
        });
    });
});
class TxGovernanceRegistryTests {
    /**
     * Sets the value of an `asset_id` to the signed account id. Only callable by root.
     *
     * @param {ApiPromise} api Connected API Promise.
     * @param {Uint8Array|string} walletAddress wallet public key
     * @param {u128} assetID asset id
     * @param {AccountId32|Uint8Array} value Wallet to be set to
     */
    static async setAsset(api, wallet, assetID, value) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.governanceRegistry.set(assetID, value)));
    }
    /**
     * Removes mapping of an `asset_id`. Only callable by root.
     *
     * @param {ApiPromise} api Connected API Promise.
     * @param {Uint8Array|string} wallet Wallet making the transaction.
     * @param {u128} assetID Asset id to be removed.
     */
    static async removeAsset(api, wallet, assetID) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.governanceRegistry.remove(assetID)));
    }
    /**
     * Sets the value of an `asset_id` to root. Only callable by root.
     *
     * @param {ApiPromise} api Connected API Promise.
     * @param {Uint8Array|string} wallet Wallet making the transaction.
     * @param {u128} assetID Asset id to be set.
     */
    static async grantRoot(api, wallet, assetID) {
        return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.governanceRegistry.grantRoot(assetID)));
    }
}
exports.TxGovernanceRegistryTests = TxGovernanceRegistryTests;
//# sourceMappingURL=txGovernanceRegsitryTests.js.map