"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.RpcCrowdloanRewardsTests = void 0;
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
describe('rpc.crowdloanRewards Tests', function () {
    if (!test_configuration_json_1.default.enabledTests.rpc.enabled)
        return;
    it('rpc.crowdloanRewards.account Tests', async function () {
        if (!test_configuration_json_1.default.enabledTests.rpc.account__success)
            this.skip();
        const accountId = walletAlice.derive('/contributor-1/reward').publicKey;
        const result = await RpcCrowdloanRewardsTests.rpcCrowdloanRewardsTest(accountId);
        (0, chai_1.expect)(result).to.be.a["bignumber"].that.equals('0');
    });
});
class RpcCrowdloanRewardsTests {
    static async rpcCrowdloanRewardsTest(accountId) {
        return await api.rpc.crowdloanRewards.amountAvailableToClaimFor(accountId);
    }
}
exports.RpcCrowdloanRewardsTests = RpcCrowdloanRewardsTests;
