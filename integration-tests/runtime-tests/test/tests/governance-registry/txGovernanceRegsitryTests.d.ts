import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { Null, Result, u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces";
import { IEvent } from "@polkadot/types/types";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";
export declare class TxGovernanceRegistryTests {
    /**
     * Sets the value of an `asset_id` to the signed account id. Only callable by root.
     *
     * @param {ApiPromise} api Connected API Promise.
     * @param {Uint8Array|string} walletAddress wallet public key
     * @param {u128} assetID asset id
     * @param {AccountId32|Uint8Array} value Wallet to be set to
     */
    static setAsset(api: ApiPromise, wallet: KeyringPair, assetID: u128, value: AccountId32 | Uint8Array): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>>;
    /**
     * Removes mapping of an `asset_id`. Only callable by root.
     *
     * @param {ApiPromise} api Connected API Promise.
     * @param {Uint8Array|string} wallet Wallet making the transaction.
     * @param {u128} assetID Asset id to be removed.
     */
    static removeAsset(api: ApiPromise, wallet: KeyringPair, assetID: u128): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>>;
    /**
     * Sets the value of an `asset_id` to root. Only callable by root.
     *
     * @param {ApiPromise} api Connected API Promise.
     * @param {Uint8Array|string} wallet Wallet making the transaction.
     * @param {u128} assetID Asset id to be set.
     */
    static grantRoot(api: ApiPromise, wallet: KeyringPair, assetID: u128): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>>;
}
