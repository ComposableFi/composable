"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleSetSignerSuccessTest = exports.runBeforeTxOracleSetSigner = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
async function runBeforeTxOracleSetSigner(sudoKey, signer) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.assets.mintInto(1, signer.publicKey, 555555555555)));
}
exports.runBeforeTxOracleSetSigner = runBeforeTxOracleSetSigner;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param controller Keyring which is a controller.
 * @param signer Keyring which will be set as a signer.
 */
async function txOracleSetSignerSuccessTest(controller, signer) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, controller, api.events.oracle.SignerSet.is, api.tx.oracle.setSigner(signer.address), false);
}
exports.txOracleSetSignerSuccessTest = txOracleSetSignerSuccessTest;
