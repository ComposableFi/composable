"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.RpcCrowdloanRewardsTests = void 0;
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
describe("rpc.crowdloanRewards Tests", function () {
    if (!test_configuration_json_1.default.enabledTests.rpc.enabled)
        return;
    // Set timeout to 1 minute.
    this.timeout(60 * 1000);
    let api;
    let walletAlice;
    before("Setting up tests", async function () {
        this.timeout(60 * 1000);
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice } = (0, walletHelper_1.getDevWallets)(newKeyring);
        walletAlice = devWalletAlice;
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    it("rpc.crowdloanRewards.account Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.rpc.account__success)
            this.skip();
        const accountId = walletAlice.derive("/contributor-1/reward").publicKey;
        const result = await RpcCrowdloanRewardsTests.rpcCrowdloanRewardsTest(api, accountId);
        (0, chai_1.expect)(result).to.be.a["bignumber"].that.equals("0");
    });
});
class RpcCrowdloanRewardsTests {
    static async rpcCrowdloanRewardsTest(api, accountId) {
        return await api.rpc.crowdloanRewards.amountAvailableToClaimFor(accountId);
    }
}
exports.RpcCrowdloanRewardsTests = RpcCrowdloanRewardsTests;
//# sourceMappingURL=rpcCrowdloanRewardsTests.js.map