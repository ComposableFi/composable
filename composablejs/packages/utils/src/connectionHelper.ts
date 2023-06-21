import * as definitions from "@composable/types/definitions";
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { ApiOptions } from "@polkadot/api/types";

/**
 * Async function to set up a picasso blockchain connection.
 * It reads our chain types & sets them up. Then defines the connection endpoint,
 * as well as creating a WebSocket Provider, and finally establishes the connection.
 *
 * ToDo: Add functionality to read private keys from `external file` or `env variables`.
 *    This will make it possible to use the integration tests against a live network.
 *
 * @return Promise<{ApiPromise, Keyring}> The connected API client object & a ready to use Keyring
 */

export async function getNewConnection(endpoint: string) {
  const rpc = Object.keys(definitions)
    .filter(k => Object.keys((<any>definitions)[k].rpc).length > 0) // eslint-disable-line @typescript-eslint/no-explicit-any
    .reduce((accumulator, key) => ({ ...accumulator, [key]: (<any>definitions)[key].rpc }), {}); // eslint-disable-line @typescript-eslint/no-explicit-any
  const types = Object.values(definitions).reduce((accumulator, { types }) => ({ ...accumulator, ...types }), {});

  const provider = new WsProvider(endpoint);
  const apiOptions: ApiOptions = {
    provider,
    types,
    rpc
  };
  const newClient = await ApiPromise.create(apiOptions);

  // do something before every test,
  // then run the next hook in this array
  const newKeyring = new Keyring({ type: "sr25519" });
  return { newClient, newKeyring };
}
