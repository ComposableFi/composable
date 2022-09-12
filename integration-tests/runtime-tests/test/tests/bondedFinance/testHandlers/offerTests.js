"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txBondedFinanceOfferFailureTest = exports.txBondedFinanceOfferSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param requestParameters wallet public key
 * @return Transaction event.
 */
async function txBondedFinanceOfferSuccessTest(api, wallet, requestParameters) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.bondedFinance.NewOffer.is, api.tx.bondedFinance.offer(requestParameters, true));
}
exports.txBondedFinanceOfferSuccessTest = txBondedFinanceOfferSuccessTest;
/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param requestParameters wallet public key
 * @return Transaction event.
 */
async function txBondedFinanceOfferFailureTest(api, wallet, requestParameters) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.system.ExtrinsicFailed.is, api.tx.bondedFinance.offer(requestParameters, true), true);
}
exports.txBondedFinanceOfferFailureTest = txBondedFinanceOfferFailureTest;
//# sourceMappingURL=offerTests.js.map