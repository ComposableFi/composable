import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";
/***
 * This mints a specific `amount` to all specified `assetIDs` to a defined `wallet`.
 *
 * @param api Connected api client
 * @param wallet The wallet receiving the assets.
 * @param sudoKey The sudo key making the transaction.
 * @param assetIDs All assets to be minted to wallet.
 * @param amount Mint amount.
 */
export declare function mintAssetsToWallet(api: ApiPromise, wallet: KeyringPair, sudoKey: KeyringPair, assetIDs: number[], amount?: bigint): Promise<void>;
/***
 * Returns the passed amount as 12 decimal tokens for better readibility
 * @param Accepts either string or number
 * @returns valid tokens with 12 decimals omitted
 */
export declare function Pica(value: string | number): bigint;
