import { Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
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
export declare function getDevWallets(keyring: Keyring): {
    devWalletAlice: KeyringPair;
    devWalletBob: KeyringPair;
    devWalletCharlie: KeyringPair;
    devWalletDave: KeyringPair;
    devWalletEve: KeyringPair;
    devWalletFerdie: KeyringPair;
};
