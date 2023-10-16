import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {cryptoWaitReady} from "@polkadot/util-crypto";

export async function initializeApi(endpoint: string): Promise<ApiPromise>{
  await cryptoWaitReady();
  const wsProvider = new WsProvider(endpoint);
  const api = await ApiPromise.create({ provider: wsProvider });
  await api.isReady;
  return api;
}

export function getWallets(testSuite: string){
  const keyring = new Keyring({type: 'sr25519'});
  const testWallet = keyring.createFromUri(`//Alice/${testSuite}`);
  const sudoKey = keyring.createFromUri('//Alice');
  return {sudoKey, testWallet};
}