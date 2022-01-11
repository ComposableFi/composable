/* eslint-disable no-trailing-spaces */
/* eslint-disable max-len */
/**
 * Inserts default values into the devnet for development.
 **/
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { testTransactionGenerator } from './generators/exampleGenerators/testTransactionGenerator';
import { crowdloanRewardGenerator } from './generators/crowdloanGenerators/crowdloanRewardGenerator';


// ToDo: Change endpoint to be read from env variables or run parameters.
const endpoint = 'ws://127.0.0.1:9988';
const provider = new WsProvider(endpoint);

let walletAlice: KeyringPair;
let walletBob: KeyringPair;
let walletCharlie: KeyringPair;
let walletDave: KeyringPair;
let walletEve: KeyringPair;
let walletFerdie: KeyringPair;

/**
 * Call default generation here.
 * @param {ApiPromise} api Connected API Promise.
**/
async function createDefaultData(api: ApiPromise, sudoKey: KeyringPair) {
  await testTransactionGenerator.testTransaction(api, walletAlice, walletBob.address);
  await crowdloanRewardGenerator.testCrowdloanRewards(api, sudoKey, walletAlice);
  // ToDo: Add additional data generator calls here.
  //       Consider splitting it up into groups of similiar generators, to keep it clean.
}

/**
 * Application entry point
**/
async function main() {
  // Instantiate the API
  const api = await ApiPromise.create({ provider: provider });
  // Constuct the keyring after the API (crypto has an async init)
  const keyring = new Keyring({ type: 'sr25519' });

  /*  Get keys for dev accounts.
      ToDo (D. Roth): Read public/private keys from external file to be usable in live environment.
            and ability to specify keys using env variables or using run parameters.
  */
  walletAlice = keyring.addFromUri('//Alice');
  walletBob = keyring.addFromUri('//Bob');
  walletCharlie = keyring.addFromUri('//Charlie');
  walletDave = keyring.addFromUri('//Dave');
  walletEve = keyring.addFromUri('//Eve');
  walletFerdie = keyring.addFromUri('//Ferdie');

  console.info('Creating dummy data...');
  await createDefaultData(api, walletAlice);
  console.info('Finished creating dummy data.');

  api.disconnect();
}

if (require.main === module) {
  main().catch(console.error).finally(() => process.exit());
}
