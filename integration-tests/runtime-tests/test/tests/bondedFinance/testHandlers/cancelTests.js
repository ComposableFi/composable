"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txBondedFinanceCancelSudoSuccessTest = exports.txBondedFinanceCancelFailureTest = exports.txBondedFinanceCancelSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.bondedFinance.cancel with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
async function txBondedFinanceCancelSuccessTest(api, wallet, offerId) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.bondedFinance.OfferCancelled.is, api.tx.bondedFinance.cancel(offerId));
}
exports.txBondedFinanceCancelSuccessTest = txBondedFinanceCancelSuccessTest;
/**
 * Tests tx.bondedFinance.cancel with provided parameters that should fail.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
async function txBondedFinanceCancelFailureTest(api, wallet, offerId) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.system.ExtrinsicFailed.is, api.tx.bondedFinance.cancel(offerId), true);
}
exports.txBondedFinanceCancelFailureTest = txBondedFinanceCancelFailureTest;
/**
 * Tests tx.bondedFinance.cancel as SUDO with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise w/ sudo rights.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
async function txBondedFinanceCancelSudoSuccessTest(api, wallet, offerId) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.bondedFinance.cancel(offerId)));
}
exports.txBondedFinanceCancelSudoSuccessTest = txBondedFinanceCancelSudoSuccessTest;
//# sourceMappingURL=cancelTests.js.map