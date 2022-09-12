import { ApiPromise } from "@polkadot/api";
import { AnyTuple, IEvent } from "@polkadot/types/types";
import { SubmittableExtrinsic, AddressOrPair } from "@polkadot/api/types";
/**
 * Sends an unsigned extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export declare function sendUnsignedAndWaitForSuccess<T extends AnyTuple>(api: ApiPromise, filter: (event: IEvent<AnyTuple>) => event is IEvent<T>, call: SubmittableExtrinsic<"promise">, intendedToFail?: boolean): Promise<IEvent<T>>;
/**
 * Sends a signed extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export declare function sendAndWaitForSuccess<T extends AnyTuple>(api: ApiPromise, sender: AddressOrPair, filter: (event: IEvent<AnyTuple>) => event is IEvent<T>, call: SubmittableExtrinsic<"promise">, intendedToFail?: boolean): Promise<IEvent<T>>;
/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export declare function sendWithBatchAndWaitForSuccess<T extends AnyTuple>(api: ApiPromise, sender: AddressOrPair, filter: (event: IEvent<AnyTuple>) => event is IEvent<T>, call: SubmittableExtrinsic<"promise">[], intendedToFail: boolean): Promise<IEvent<T>>;
/**
 * Waits for N amount of blocks.
 * @param {ApiPromise} api Connected API Client.
 * @param {number} n Amount of blocks.
 * @return The current block number after waiting.
 */
export declare function waitForBlocks(api: ApiPromise, n?: number): Promise<import("@polkadot/types-codec").u32>;
/**
 * Helper to wait for n blocks.
 * @param {ApiPromise} api Connected API Client.
 * @param {number} n Block wait duration.
 * @return The current block number after waiting.
 */
export declare function waitForBlockHandler(api: ApiPromise, n: any): Promise<import("@polkadot/types-codec").u32>;
/**
 * Sends the given unsigned `call` and waits for an event that fits `filter`.
 * @param {ApiPromise} api api object
 * @param {IEvent} filter which event to filter for
 * @param {SubmittableExtrinsic<Promise>} call a call that can be submitted to the chain
 * @param {boolean} intendedToFail If true a failed submission will be counted as a success.
 * @returns event that fits the filter
 */
export declare function sendUnsignedAndWaitFor<T extends AnyTuple>(api: ApiPromise, filter: (event: IEvent<AnyTuple>) => event is IEvent<T>, call: SubmittableExtrinsic<"promise">, intendedToFail: boolean): Promise<IEvent<T>>;
/**
 * Signs and sends the given `call` from `sender` and waits for an event that fits `filter`.
 * @param api api object
 * @param sender the sender of the transaction
 * @param filter which event to filter for
 * @param call a call that can be submitted to the chain
 * @param {boolean} intendedToFail If true a failed submission will be counted as a success.
 * @returns event that fits the filter
 */
export declare function sendAndWaitFor<T extends AnyTuple>(api: ApiPromise, sender: AddressOrPair, filter: (event: IEvent<AnyTuple>) => event is IEvent<T>, call: SubmittableExtrinsic<"promise">, intendedToFail: boolean): Promise<IEvent<T>>;
/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export declare function sendAndWaitForWithBatch<T extends AnyTuple>(api: ApiPromise, sender: AddressOrPair, filter: (event: IEvent<AnyTuple>) => event is IEvent<T>, call: SubmittableExtrinsic<"promise">[], intendedToFail: boolean): Promise<IEvent<T>>;
