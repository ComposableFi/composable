"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleAddStakeSuccessTest = exports.runBeforeTxOracleAddStake = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const mintingHelper_1 = require("@composable/utils/mintingHelper");
/**
 * Provides funds for Oracle tests.
 * @param api Connect ApiPromise
 * @param sudoKey KeyringPair with sudo rights
 * @param wallet1 Wallet to provide funds to
 * @param wallet2 Wallet to provide funds to
 */
async function runBeforeTxOracleAddStake(api, sudoKey, wallet1, wallet2) {
    await (0, mintingHelper_1.mintAssetsToWallet)(api, wallet1, sudoKey, [1]);
    await (0, mintingHelper_1.mintAssetsToWallet)(api, wallet2, sudoKey, [1]);
}
exports.runBeforeTxOracleAddStake = runBeforeTxOracleAddStake;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param api Connect ApiPromise
 * @param sender Connected API Promise w/ sudo rights.
 * @param {u128} stake Staking amount.
 */
async function txOracleAddStakeSuccessTest(api, sender, stake) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sender, api.events.oracle.StakeAdded.is, api.tx.oracle.addStake(stake), false);
}
exports.txOracleAddStakeSuccessTest = txOracleAddStakeSuccessTest;
//# sourceMappingURL=addStakeTests.js.map