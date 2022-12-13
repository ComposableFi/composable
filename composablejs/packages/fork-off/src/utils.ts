import { getNewConnection } from "@composable/utils";

/**
 * Get Kusama latest version from the RPC
 */
export async function getKusamaVersion(): Promise<string> {
  const kusamaRpc = "wss://kusama-rpc.polkadot.io";
  const { newClient: api } = await getNewConnection(kusamaRpc);

  const version = (await api.rpc.system.version()).toString();
  return version.split("-")[0];
}
