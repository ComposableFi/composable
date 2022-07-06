import { ApiPromise, WsProvider } from "@polkadot/api";

export function createRPC(definitions: any): any {
  return Object.keys(definitions)
    .filter(k => {
      if (!(definitions as any)[k].rpc) {
        return false;
      } else {
        return Object.keys((definitions as any)[k].rpc).length > 0;
      }
    })
    .reduce(
      (accumulator, key) => ({
        ...accumulator,
        [key]: (definitions as any)[key].rpc
      }),
      {}
    );
}

export function createTypes(definitions: any): any {
  return Object.keys(definitions)
    .filter(key => Object.keys((definitions as any)[key].types).length > 0)
    .reduce(
      (accumulator, key) => ({
        ...accumulator,
        ...(definitions as any)[key].types
      }),
      {}
    );
}

export const buildApi = async (substrateNodeUrl: string, types: any, rpc: any): Promise<ApiPromise> => {
  const wsProvider = new WsProvider(substrateNodeUrl, 1000);
  const api = await ApiPromise.create({ provider: wsProvider, types, rpc });

  return await api.isReady;
};
