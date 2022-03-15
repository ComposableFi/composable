"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RpcLendingTests = void 0;
const chai_1 = require("chai");
describe('rpc.lending Tests', function () {
    // Check if group of tests are enabled.
    // if (!testConfiguration.enabledTests.query.enabled)
    // return;
    // repeat this block as needed for every test case defined in the class below.
    it('rpc.lending.getBorrowLimit Tests', async function () {
        const result = await RpcLendingTests.rpcLendingTest();
        (0, chai_1.expect)(result).to.be.a["bignumber"].that.equals('0');
    });
});
class RpcLendingTests {
    static async rpcLendingTest() {
        const accountId = walletAlice.derive('/contributor-1/reward').publicKey;
        const result = await api.rpc.lending.getBorrowLimit("1", accountId);
        return result;
    }
}
exports.RpcLendingTests = RpcLendingTests;
