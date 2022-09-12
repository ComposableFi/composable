"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txBondedFinanceBondSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.bondedFinance.offer with provided parameters that should succeed.
 *
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64} offerId
 * @param {u128|number} nbOfBonds
 * @return Transaction event.
 */
async function txBondedFinanceBondSuccessTest(api, wallet, offerId, nbOfBonds) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.bondedFinance.NewBond.is, api.tx.bondedFinance.bond(offerId, nbOfBonds, true));
}
exports.txBondedFinanceBondSuccessTest = txBondedFinanceBondSuccessTest;
//# sourceMappingURL=bondTests.js.map