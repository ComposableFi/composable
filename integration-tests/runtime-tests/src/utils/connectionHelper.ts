import chai from "chai";
import chai_bn from "chai-bn";
import BN from "bn.js";
import * as definitions from "@composable/types/interfaces/definitions";
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
export async function getNewConnection() {
  // Enable and inject BN dependency
  chai.use(chai_bn(BN));
  const rpc = Object.keys(definitions)
    .filter(k => Object.keys(definitions[k].rpc).length > 0)
    .reduce((accumulator, key) => ({ ...accumulator, [key]: definitions[key].rpc }), {});
  const types = Object.values(definitions).reduce((accumulator, { types }) => ({ ...accumulator, ...types }), {});

  const endpoint = "ws://" + (process.env.ENDPOINT ?? "127.0.0.1:9988");
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
