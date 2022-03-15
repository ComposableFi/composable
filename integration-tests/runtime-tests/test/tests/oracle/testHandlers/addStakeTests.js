"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleAddStakeSuccessTest = exports.runBeforeTxOracleAddStake = void 0;
const chai_1 = require("chai");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
async function runBeforeTxOracleAddStake(sudoKey, wallet1, wallet2) {
    const { data: [result1], } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.assets.mintInto(1, wallet1.publicKey, 555555555555)));
    (0, chai_1.expect)(result1.isOk).to.be.true;
    const { data: [result2], } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.assets.mintInto(1, wallet2.publicKey, 555555555555)));
    (0, chai_1.expect)(result2.isOk).to.be.true;
    return;
}
exports.runBeforeTxOracleAddStake = runBeforeTxOracleAddStake;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param sender Connected API Promise w/ sudo rights.
 * @param {u128} stake Staking amount.
 */
async function txOracleAddStakeSuccessTest(sender, stake) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sender, api.events.oracle.StakeAdded.is, api.tx.oracle.addStake(stake), false);
}
exports.txOracleAddStakeSuccessTest = txOracleAddStakeSuccessTest;
