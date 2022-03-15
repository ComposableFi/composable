"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleSubmitPriceSuccessTest = void 0;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param signer Connected API Promise w/ sudo rights.
 * @param price Price to be submitted.
 * @param assetId Specifies asset id.
 */
const polkadotjs_1 = require("@composable/utils/polkadotjs");
async function txOracleSubmitPriceSuccessTest(signer, price, assetId) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, signer, api.events.oracle.PriceSubmitted.is, api.tx.oracle.submitPrice(price, assetId), false);
}
exports.txOracleSubmitPriceSuccessTest = txOracleSubmitPriceSuccessTest;
