/* eslint-disable no-var */
import { ApiPromise, Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import Web3 from "web3";

declare global {
  var useTestnetWallets: boolean;
  var testSudoCommands: boolean;
  var endpoint: string;
  var api: ApiPromise;
  var keyring: Keyring;
  var walletAlice: KeyringPair;
  var walletBob: KeyringPair;
  var walletCharlie: KeyringPair;
  var walletDave: KeyringPair;
  var walletEve: KeyringPair;
  var walletFerdie: KeyringPair;
  var web3: Web3;
}
