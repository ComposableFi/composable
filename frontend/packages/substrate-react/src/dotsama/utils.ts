import { ApiPromise, WsProvider } from "@polkadot/api";
import { SubstrateNetworkId } from "shared";
import { ChainApi } from "../dotsama/types";

export const getSigner = async (
  applicationName: string,
  address: string
): Promise<any> => {
  const extensionPackage = await import("@polkadot/extension-dapp");
  const { web3FromAddress, web3Enable } = extensionPackage;
  await web3Enable(applicationName);
  const injector = await web3FromAddress(address);
  return injector.signer;
};

export async function createChainApi(
  substrateApi: Partial<{ [chainId in SubstrateNetworkId]: ChainApi }>,
  supportedChains: {
    chainId: SubstrateNetworkId;
    rpcUrl: string;
    rpc: any;
    types: any;
  }[]
): Promise<{ [chainId in SubstrateNetworkId]: ChainApi }> {
  let newRecord: { [chainId in SubstrateNetworkId]: ChainApi } = Object.keys(
    substrateApi
  ).reduce((acc, curr) => {
    return {
      ...acc,
      [curr]: {
        ...substrateApi[curr as SubstrateNetworkId],
      },
    };
  }, {} as { [chainId in SubstrateNetworkId]: ChainApi });

  let connectionPromises: Array<Promise<boolean>> = [];

  for (const element of supportedChains) {
    connectionPromises.push(
      new Promise(async (res, _rej) => {
        const { chainId, rpcUrl, rpc, types } = element;
        try {
          const wsProvider = new WsProvider(rpcUrl);
          const parachainApi = new ApiPromise({
            provider: wsProvider,
            rpc,
            types,
          });

          await parachainApi.isReadyOrError;
          if (parachainApi.isConnected) {
            newRecord[chainId].apiStatus = "connected";
            newRecord[chainId].parachainApi = parachainApi;
            res(true);
          } else {
            newRecord[chainId].apiStatus = "failed";
            newRecord[chainId].parachainApi = undefined;
            res(false);
          }
        } catch (err) {
          res(false);
        }
      })
    );
  }

  await Promise.all(connectionPromises);

  return newRecord;
}
