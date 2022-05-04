/* eslint-disable no-var */
import '@composable/../../src/types/interfaces/augment-api';
import '@composable/../../src/types/interfaces/augment-types';
import * as definitions from '@composable/../../src/types/interfaces/definitions';

import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ApiOptions } from '@polkadot/api/types';
import Web3 from 'web3';

import chai_bn from "chai-bn";
import BN from "bn.js";
import chai from "chai";

global.useTestnetWallets = true;
global.testSudoCommands = true;
// ToDo (D. Roth): Read public/private keys from external file to be usable in live environment.
//       and ability to specify keys using env variables or using run parameters.


exports.mochaHooks = {
  async beforeAll() {
    this.timeout(5 * 60 * 1000);
    // Enable and inject BN dependency
    chai.use(chai_bn(BN));
    const rpc = Object.keys(definitions)
      .filter(k => Object.keys(definitions[k].rpc).length > 0)
      .reduce((accumulator, key) => ({ ...accumulator, [key]: definitions[key].rpc }), {});
    const types = Object.values(definitions).reduce((accumulator, { types }) => ({ ...accumulator, ...types }), {});

    global.endpoint = 'ws://' + (process.env.ENDPOINT ?? '127.0.0.1:9988');
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
  },
  async afterAll() {
    return await global.api.disconnect();
  }
}
