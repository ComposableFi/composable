"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.RpcAssetsTests = void 0;
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
describe("[SHORT] rpc.assets Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.rpc.enabled)
        return;
    let api;
    let walletBobPublicKey;
    before("Setting up tests", async function () {
        this.timeout(60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletBob } = (0, walletHelper_1.getDevWallets)(newKeyring);
        walletBobPublicKey = devWalletBob.address;
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    /**
     * The `assets.balanceOf` RPC provides the amount a wallet holds of a specific asset.
     */
    it("rpc.assets.balanceOf Test #1", async function () {
        if (!test_configuration_json_1.default.enabledTests.rpc.balanceOf__success)
            this.skip();
        const PICA = api.createType("SafeRpcWrapper", 1);
        const PICA_amount = await RpcAssetsTests.rpcAssetsTest(api, PICA, walletBobPublicKey);
        (0, chai_1.expect)(parseInt(PICA_amount.toString())).to.not.equals(0);
    });
    it("rpc.assets.balanceOf Test #2", async function () {
        if (!test_configuration_json_1.default.enabledTests.rpc.balanceOf__success)
            this.skip();
        const KSM = api.createType("SafeRpcWrapper", 4);
        const KSM_amount = await RpcAssetsTests.rpcAssetsTest(api, KSM, walletBobPublicKey);
        (0, chai_1.expect)(parseInt(KSM_amount.toString())).to.be.equals(0);
    });
    it("rpc.assets.balanceOf Test #3", async function () {
        if (!test_configuration_json_1.default.enabledTests.rpc.balanceOf__success)
            this.skip();
        const kUSD = api.createType("SafeRpcWrapper", 129);
        const kUSD_amount = await RpcAssetsTests.rpcAssetsTest(api, kUSD, walletBobPublicKey);
        (0, chai_1.expect)(parseInt(kUSD_amount.toString())).to.be.equals(0);
    });
    it("rpc.assets.listAssets Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.rpc.listAssets__success)
            this.skip();
        const result = await RpcAssetsTests.rpcListAssetsTest(api);
        (0, chai_1.expect)(result).to.have.lengthOf(8);
        result.every(i => (0, chai_1.expect)(i).to.have.all.keys("id", "name"));
        (0, chai_1.expect)(result.map(e => e.id.toNumber())).to.include.members([1, 2, 3, 4, 5, 129, 130, 131]);
        (0, chai_1.expect)(result.map(e => hex_to_ascii(e.name.toString()))).to.include.members([
            "PICA",
            "LAYR",
            "CROWD_LOAN",
            "KSM",
            "PBLO",
            "kUSD",
            "USDT",
            "USDC"
        ]);
    });
});
class RpcAssetsTests {
    static async rpcAssetsTest(apiClient, assetId, publicKey) {
        return await apiClient.rpc.assets.balanceOf(assetId, publicKey);
    }
    static async rpcListAssetsTest(apiClient) {
        return await apiClient.rpc.assets.listAssets();
    }
}
exports.RpcAssetsTests = RpcAssetsTests;
function hex_to_ascii(str1) {
    const hex = str1.toString();
    let str = "";
    //skip 0x
    for (let n = 2; n < hex.length; n += 2) {
        str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
    }
    return str;
}
//# sourceMappingURL=rpcAssetsTests.js.map