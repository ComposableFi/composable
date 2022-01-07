/* eslint-disable no-var */
import '@composable/types/interfaces/augment-api';
import '@composable/types/interfaces/augment-types';
import * as definitions from '@composable/types/interfaces/definitions';
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ApiOptions } from '@polkadot/api/types';
import Web3 from 'web3';
import { args } from "./args";

global.useTestnetWallets = true;
global.testSudoCommands = true;
// ToDo (D. Roth): Read public/private keys from external file to be usable in live environment.
//       and ability to specify keys using env variables or using run parameters.

export async function runBefore() {
  const chai = require('chai');
  const BN = require('bn.js');

  // Enable and inject BN dependency
  chai.use(require('chai-bn')(BN));

  // console.log(JSON.stringify(definitions, null, 4));
  const rpc = Object.keys(definitions).reduce((accumulator, key) => ({ ...accumulator, [key]: definitions[key].rpc }), {});
  const types = Object.values(definitions).reduce((accumulator, { types }) => ({ ...accumulator, ...types }), {});
  // console.log("rpc: ", JSON.stringify(rpc, null, 4));
  // console.log("types: ", JSON.stringify(types, null, 4));

  global.endpoint = `ws://${args.h}:${args.p}`;
  const provider = new WsProvider(global.endpoint);
  console.debug(`Establishing connection to ${global.endpoint}...`);
  const apiOptions: ApiOptions = {
    provider, types, rpc
  };
  global.api = await ApiPromise.create(apiOptions);

  global.web3 = new Web3();

  // do something before every test,
  // then run the next hook in this array
  global.keyring = new Keyring({ type: 'sr25519' });

  if (global.useTestnetWallets === true) {
    global.walletAlice = global.keyring.addFromUri('//Alice');
    global.walletBob = global.keyring.addFromUri('//Bob');
    global.walletCharlie = global.keyring.addFromUri('//Charlie');
    global.walletDave = global.keyring.addFromUri('//Dave');
    global.walletEve = global.keyring.addFromUri('//Eve');
    global.walletFerdie = global.keyring.addFromUri('//Ferdie');
  }
  return;
}

export async function runAfter() {
  await global.api.disconnect();
}