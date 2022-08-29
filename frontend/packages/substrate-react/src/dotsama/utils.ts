import { ApiPromise, WsProvider } from '@polkadot/api';
import { ParachainApi, ParachainId } from './types';

export const getSigner = async (
  applicationName: string,
  address: string
): Promise<any> => {
  const extensionPackage = await import('@polkadot/extension-dapp');
  const { web3FromAddress, web3Enable } = extensionPackage;
  await web3Enable(applicationName);
  const injector = await web3FromAddress(address);
  return injector.signer;
};

export async function createParachainApis(substrateApi: { [chainId in ParachainId]: ParachainApi }, supportedChains: {
  chainId: ParachainId;
  rpcUrl: string;
  rpc: any;
  types: any;
}[]): Promise<{ [chainId in ParachainId]: ParachainApi }> {
  let newRecord: { [chainId in ParachainId]: ParachainApi } = Object.keys(substrateApi).reduce((acc, curr) => {
    return {
      ...acc,
      [curr]: {
        ...substrateApi[curr as ParachainId]
      }
    }
  }, {} as { [chainId in ParachainId]: ParachainApi });

  let connectionPromises: Array<Promise<boolean>> = [];

  for (let i = 0; i < supportedChains.length; i++) {
    connectionPromises.push(new Promise(async (res, _rej) => {
      const { chainId, rpcUrl, rpc, types } = supportedChains[i];
      try {
        const wsProvider = new WsProvider(rpcUrl);
        const parachainApi = new ApiPromise({ provider: wsProvider, rpc, types });
  
        await parachainApi.isReady;
        newRecord[chainId].apiStatus = 'connected';
        newRecord[chainId].parachainApi = parachainApi;
        res(true);
      } catch (err) {
        console.log(`Error connecting API for ${chainId} `, err);
        newRecord[chainId].apiStatus = 'failed';
        newRecord[chainId].parachainApi = undefined;
        res(false);
      }
    }))
  }

  await Promise.all(connectionPromises);

  return newRecord;
}