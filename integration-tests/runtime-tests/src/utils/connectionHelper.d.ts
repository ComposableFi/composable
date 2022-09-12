import { ApiPromise, Keyring } from "@polkadot/api";
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
export declare function getNewConnection(): Promise<{
    newClient: ApiPromise;
    newKeyring: Keyring;
}>;
