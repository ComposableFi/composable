"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txBondedFinanceOfferFailureTest = exports.txBondedFinanceOfferSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {} requestParameters wallet public key
 */
async function txBondedFinanceOfferSuccessTest(wallet, requestParameters) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.bondedFinance.NewOffer.is, api.tx.bondedFinance.offer(requestParameters, true));
}
exports.txBondedFinanceOfferSuccessTest = txBondedFinanceOfferSuccessTest;
/**
 * Tests tx.bondedFinance.offer with provided parameters that should fail.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {} requestParameters wallet public key
 */
async function txBondedFinanceOfferFailureTest(wallet, requestParameters) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.system.ExtrinsicFailed.is, api.tx.bondedFinance.offer(requestParameters, true), true);
}
exports.txBondedFinanceOfferFailureTest = txBondedFinanceOfferFailureTest;
