"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txBondedFinanceBondFailureTest = exports.txBondedFinanceBondSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128} nbOfBonds
 */
async function txBondedFinanceBondSuccessTest(wallet, offerId, nbOfBonds) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.bondedFinance.NewBond.is, api.tx.bondedFinance.bond(offerId, nbOfBonds, true));
}
exports.txBondedFinanceBondSuccessTest = txBondedFinanceBondSuccessTest;
/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128} nbOfBonds
 */
async function txBondedFinanceBondFailureTest(wallet, offerId, nbOfBonds) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.system.ExtrinsicFailed.is, api.tx.bondedFinance.bond(offerId, nbOfBonds, true), true);
}
exports.txBondedFinanceBondFailureTest = txBondedFinanceBondFailureTest;
