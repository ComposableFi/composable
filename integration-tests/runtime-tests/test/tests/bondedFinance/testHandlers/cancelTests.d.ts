/// <reference types="@composable/types/interfaces/types-lookup" />
import { IKeyringPair } from "@polkadot/types/types";
import { ApiPromise } from "@polkadot/api";
import { u64 } from "@polkadot/types-codec";
/**
 * Tests tx.bondedFinance.cancel with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
export declare function txBondedFinanceCancelSuccessTest(api: ApiPromise, wallet: IKeyringPair, offerId: u64 | number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").u128]>>;
/**
 * Tests tx.bondedFinance.cancel with provided parameters that should fail.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
export declare function txBondedFinanceCancelFailureTest(api: ApiPromise, wallet: IKeyringPair, offerId: u64 | number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types/lookup").SpRuntimeDispatchError, import("@polkadot/types/lookup").FrameSupportWeightsDispatchInfo]>>;
/**
 * Tests tx.bondedFinance.cancel as SUDO with provided parameters that should succeed.
 * @param {ApiPromise} api Connected API Client.
 * @param {IKeyringPair} wallet Connected API Promise w/ sudo rights.
 * @param {u64|number} offerId
 * @return Transaction event.
 */
export declare function txBondedFinanceCancelSudoSuccessTest(api: ApiPromise, wallet: IKeyringPair, offerId: u64 | number): Promise<import("@polkadot/types/types").IEvent<[import("@polkadot/types-codec").Result<import("@polkadot/types-codec").Null, import("@polkadot/types/lookup").SpRuntimeDispatchError>]>>;
