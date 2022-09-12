"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleRemoveStakeSuccessTest = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 * Tests tx.oracle.removeStake with provided parameters that should succeed.
 * @param controller KeyringPair which is a controller.
 */
async function txOracleRemoveStakeSuccessTest(api, controller) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, controller, api.events.oracle.StakeRemoved.is, api.tx.oracle.removeStake(), false);
}
exports.txOracleRemoveStakeSuccessTest = txOracleRemoveStakeSuccessTest;
//# sourceMappingURL=removeStakeTests.js.map