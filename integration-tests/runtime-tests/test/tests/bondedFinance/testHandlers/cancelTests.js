"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txBondedFinanceCancelSudoSuccessTest = exports.txBondedFinanceCancelFailureTest = exports.txBondedFinanceCancelSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
async function txBondedFinanceCancelSuccessTest(wallet, offerId) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.bondedFinance.OfferCancelled.is, api.tx.bondedFinance.cancel(offerId));
}
exports.txBondedFinanceCancelSuccessTest = txBondedFinanceCancelSuccessTest;
/**
 * Tests tx.bondedFinance.cancel with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 */
async function txBondedFinanceCancelFailureTest(wallet, offerId) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.system.ExtrinsicFailed.is, api.tx.bondedFinance.cancel(offerId), true);
}
exports.txBondedFinanceCancelFailureTest = txBondedFinanceCancelFailureTest;
/**
 * Tests tx.bondedFinance.cancel as SUDO with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise w/ sudo rights.
 * @param {u64} offerId
 */
async function txBondedFinanceCancelSudoSuccessTest(wallet, offerId) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.bondedFinance.cancel(offerId)));
}
exports.txBondedFinanceCancelSudoSuccessTest = txBondedFinanceCancelSudoSuccessTest;
