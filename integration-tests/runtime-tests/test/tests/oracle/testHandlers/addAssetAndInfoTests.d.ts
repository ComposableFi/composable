/// <reference types="@composable/types/interfaces/types-lookup" />
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
/**
 * Tests tx.oracle.addAssetAndInfo with provided parameters that should succeed.
 * @param api Connect ApiPromise
 * @param {KeyringPair} sudoKey Connected API Promise w/ sudo rights.
 * @param assetId Id for the asset
 * @param threshold Percent close to mean to be rewarded
 * @param minAnswers Min answers before aggregation
 * @param maxAnswers Max answers to aggregate
 * @param blockInterval blocks until oracle triggered
 * @param reward reward amount for correct answer
 * @param slash slash amount for bad answer
 */
export declare function txOracleAddAssetAndInfoSuccessTest(api: ApiPromise, sudoKey: KeyringPair, assetId: any, threshold: any, minAnswers: any, maxAnswers: any, blockInterval: any, reward: any, slash: any): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
