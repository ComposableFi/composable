"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.RpcAssetsTests = void 0;
const chai_1 = require("chai");
const test_configuration_json_1 = __importDefault(require("./test_configuration.json"));
describe('rpc.assets Tests', function () {
    if (!test_configuration_json_1.default.enabledTests.rpc.enabled)
        return;
    it('rpc.assets.balanceOf Tests', async function () {
        if (!test_configuration_json_1.default.enabledTests.rpc.balanceOf__success)
            this.skip();
        const asset_id = api.createType('CurrencyId', '123456789123456789');
        const publicKey = walletAlice.address;
        const result = await RpcAssetsTests.rpcAssetsTest(asset_id, publicKey);
        (0, chai_1.expect)(result).to.be.a["bignumber"].that.equals('0');
    });
});
class RpcAssetsTests {
    static async rpcAssetsTest(asset_id, publicKey) {
        return await api.rpc.assets.balanceOf(asset_id, publicKey);
    }
}
exports.RpcAssetsTests = RpcAssetsTests;
