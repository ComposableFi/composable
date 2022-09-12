"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.getDevWallets = void 0;
/**
 * Helper function to get all devnet wallets.
 * - Alice
 * - Bob
 * - Charlie
 * - Dave
 *
 * @param keyring Keyring object received using the `connectionHelper`
 * @return {
 *   devWalletAlice: KeyringPair,
 *   devWalletBob: KeyringPair,
 *   devWalletCharlie: KeyringPair,
 *   devWalletDave: KeyringPair,
 *   devWalletEve: KeyringPair,
 *   devWalletFerdie: KeyringPair
 * } all devnet wallets
 */
function getDevWallets(keyring) {
    return {
        devWalletAlice: keyring.addFromUri("//Alice"),
        devWalletBob: keyring.addFromUri("//Bob"),
        devWalletCharlie: keyring.addFromUri("//Charlie"),
        devWalletDave: keyring.addFromUri("//Dave"),
        devWalletEve: keyring.addFromUri("//Eve"),
        devWalletFerdie: keyring.addFromUri("//Ferdie")
    };
}
exports.getDevWallets = getDevWallets;
//# sourceMappingURL=walletHelper.js.map