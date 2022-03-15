"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.QuerySystemAccountTests = void 0;
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
/**
 * Example Test
 * Just checking if provided wallet balance >0.
 */
// describe(name, function) groups all query tests for the system pallet.
describe('query.system Tests', function () {
    // Check if group of tests are enabled.
    if (!test_configuration_json_1.default.enabledTests.query.enabled)
        return;
    // This describe groups all system.account query tests.
    describe('query.system.account Tests', function () {
        // Check if group of tests are enabled.
        if (!test_configuration_json_1.default.enabledTests.query.account__success.enabled)
            return;
        // it(name, function) describes a single test.
        it('Wallet balance check should be >0', async function () {
            // Check if this test is enabled.
            if (!test_configuration_json_1.default.enabledTests.query.account__success.balanceGTZero1)
                this.skip();
            await QuerySystemAccountTests.checkBalance(api, walletAlice.address);
        });
    });
});
class QuerySystemAccountTests {
    /**
    * Tests by checking the balance of the supplied account is >0
    * @param {ApiPromise} api Connected API Promise.
    * @param {string} walletAddress wallet public key
    */
    static async checkBalance(api, walletAddress) {
        const { data: balance } = await api.query.system.account(walletAddress);
        (0, chai_1.expect)(balance.free.toBigInt() > 0).to.be.true; // .to.be.greater(0) didn't work for some reason.
    }
}
exports.QuerySystemAccountTests = QuerySystemAccountTests;
