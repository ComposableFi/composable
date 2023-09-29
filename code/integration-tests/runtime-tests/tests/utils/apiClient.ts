import {ApiPromise, WsProvider} from "@polkadot/api";
import {cryptoWaitReady} from "@polkadot/util-crypto";

export async function initializeApi(endpoint: string): Promise<ApiPromise>{
  await cryptoWaitReady();
  const wsProvider = new WsProvider(endpoint);
  const api = await ApiPromise.create({ provider: wsProvider });
  await api.isReady;
  return api;
}