/// <reference types="@composable/types/interfaces/types-lookup" />
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { AnyNumber } from "@polkadot/types-codec/types";
import { u128, u64 } from "@polkadot/types-codec";
export declare class CurrencyFactoryTests {
    static setMetadata(api: ApiPromise, sudoKey: KeyringPair, assetId: any, metadata: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    static addRange(api: ApiPromise, sudoKey: KeyringPair, range: AnyNumber | u64): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
    static initializeNewAsset(api: ApiPromise, sudoKey: KeyringPair, amount: AnyNumber | u128, beneficiary: Uint8Array): Promise<import("@polkadot/types/types").IEvent<[u128, import("@polkadot/types/interfaces/runtime").AccountId32, u128]>>;
    /**
     * Converts hex to ascii.
     * Source: https://stackoverflow.com/questions/3745666/how-to-convert-from-hex-to-ascii-in-javascript/3745677#3745677
     * @param hexx
     */
    static hex2a(hexx: any): string;
}
