"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleRemoveStakeSuccessTest = void 0;
/**
 * Tests tx.oracle.removeStake with provided parameters that should succeed.
 * @param controller KeyringPair which is a controller.
 */
const polkadotjs_1 = require("@composable/utils/polkadotjs");
async function txOracleRemoveStakeSuccessTest(controller) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, controller, api.events.oracle.StakeRemoved.is, api.tx.oracle.removeStake(), false);
}
exports.txOracleRemoveStakeSuccessTest = txOracleRemoveStakeSuccessTest;
