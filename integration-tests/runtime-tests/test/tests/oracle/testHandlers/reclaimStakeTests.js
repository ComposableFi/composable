"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleReclaimStakeSuccessTest = void 0;
/**
 * Tests tx.oracle.reclaimStake with provided parameters that should succeed.
 * @param controller KeyringPair which is a controller.
 */
const polkadotjs_1 = require("@composable/utils/polkadotjs");
async function txOracleReclaimStakeSuccessTest(controller) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, controller, api.events.oracle.StakeReclaimed.is, api.tx.oracle.reclaimStake(), false);
}
exports.txOracleReclaimStakeSuccessTest = txOracleReclaimStakeSuccessTest;
