"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.txOracleSetSignerSuccessTest = exports.runBeforeTxOracleSetSigner = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const mintingHelper_1 = require("@composable/utils/mintingHelper");
/**
 * Provides funds for Oracle tests.
 * @param api Connect ApiPromise
 * @param sudoKey KeyringPair with sudo rights
 * @param wallet1 Wallet to provide funds to
 * @param wallet2 Wallet to provide funds to
 */
async function runBeforeTxOracleSetSigner(api, sudoKey, signer) {
    return await (0, mintingHelper_1.mintAssetsToWallet)(api, signer, sudoKey, [1]);
}
exports.runBeforeTxOracleSetSigner = runBeforeTxOracleSetSigner;
/**
 * Tests tx.oracle.submitPrice with provided parameters that should succeed.
 * @param controller Keyring which is a controller.
 * @param signer Keyring which will be set as a signer.
 */
async function txOracleSetSignerSuccessTest(api, controller, signer) {
    return await (0, polkadotjs_1.sendAndWaitForSuccess)(api, controller, api.events.oracle.SignerSet.is, api.tx.oracle.setSigner(signer.address), false);
}
exports.txOracleSetSignerSuccessTest = txOracleSetSignerSuccessTest;
//# sourceMappingURL=setSignerTests.js.map