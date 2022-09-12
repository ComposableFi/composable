"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleAddAssetAndInfoSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.oracle.addAssetAndInfo with provided parameters that should succeed.
 * @param api Connect ApiPromise
 * @param {KeyringPair} sudoKey Connected API Promise w/ sudo rights.
 * @param assetId Id for the asset
 * @param threshold Percent close to mean to be rewarded
 * @param minAnswers Min answers before aggregation
 * @param maxAnswers Max answers to aggregate
 * @param blockInterval blocks until oracle triggered
 * @param reward reward amount for correct answer
 * @param slash slash amount for bad answer
 */
async function txOracleAddAssetAndInfoSuccessTest(api, sudoKey, assetId, threshold, minAnswers, maxAnswers, blockInterval, reward, slash) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.oracle.addAssetAndInfo(assetId, threshold, minAnswers, maxAnswers, blockInterval, reward, slash)));
}
exports.txOracleAddAssetAndInfoSuccessTest = txOracleAddAssetAndInfoSuccessTest;
//# sourceMappingURL=addAssetAndInfoTests.js.map