"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleReclaimStakeSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.oracle.reclaimStake with provided parameters that should succeed.
 * @param api Connected ApiPromise
 * @param controller KeyringPair which is a controller.
 */
async function txOracleReclaimStakeSuccessTest(api, controller) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, controller, api.events.oracle.StakeReclaimed.is, api.tx.oracle.reclaimStake(), false);
}
exports.txOracleReclaimStakeSuccessTest = txOracleReclaimStakeSuccessTest;
//# sourceMappingURL=reclaimStakeTests.js.map