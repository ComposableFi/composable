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
export function getDevWallets(keyring: Keyring): {
  devWalletAlice: KeyringPair;
  devWalletBob: KeyringPair;
  devWalletCharlie: KeyringPair;
  devWalletDave: KeyringPair;
  devWalletEve: KeyringPair;
  devWalletFerdie: KeyringPair;
} {
  return {
    devWalletAlice: <KeyringPair>keyring.addFromUri("//Alice"),
    devWalletBob: <KeyringPair>keyring.addFromUri("//Bob"),
    devWalletCharlie: <KeyringPair>keyring.addFromUri("//Charlie"),
    devWalletDave: <KeyringPair>keyring.addFromUri("//Dave"),
    devWalletEve: <KeyringPair>keyring.addFromUri("//Eve"),
    devWalletFerdie: <KeyringPair>keyring.addFromUri("//Ferdie")
  };
}
