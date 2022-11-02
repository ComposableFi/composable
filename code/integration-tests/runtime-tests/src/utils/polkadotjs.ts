import { ApiPromise } from "@polkadot/api";
import { AnyTuple, IEvent } from "@polkadot/types/types";
import { AddressOrPair, SubmittableExtrinsic } from "@polkadot/api/types";

/**
 * Sends an unsigned extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export async function sendUnsignedAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
  intendedToFail = false
): Promise<IEvent<T>> {
  return await sendUnsignedAndWaitFor(api, filter, call, intendedToFail);
}

/**
 * Sends a signed extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export async function sendAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
  intendedToFail = false
): Promise<IEvent<T>> {
  return await sendAndWaitFor(api, sender, filter, call, intendedToFail);
}

/**
 * Sends a signed extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export async function sendAndWaitForSuccessWithDelay<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
  waitTime: number,
  intendedToFail = false
): Promise<IEvent<T>> {
  await setTimeout(() => {
    /*NotEmpty*/
  }, waitTime);
  return await sendAndWaitFor(api, sender, filter, call, intendedToFail);
}

/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export async function sendWithBatchAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">[],
  intendedToFail: boolean
): Promise<IEvent<T>> {
  return await sendAndWaitForWithBatch(api, sender, filter, call, intendedToFail);
}

/**
 * Waits for N amount of blocks.
 * @param {ApiPromise} api Connected API Client.
 * @param {number} n Amount of blocks.
 * @return The current block number after waiting.
 */
export async function waitForBlocks(api: ApiPromise, n = 1) {
  return await waitForBlockHandler(api, n);
}

/**
 * Helper to wait for n blocks.
 * @param {ApiPromise} api Connected API Client.
 * @param {number} n Block wait duration.
 * @return The current block number after waiting.
 */
export async function waitForBlockHandler(api: ApiPromise, n: number) {
  const originBlock = await api.query.system.number();
  let currentBlock = await api.query.system.number();
  while (currentBlock.toNumber() < originBlock.toNumber() + n) {
    await sleep(3000);
    currentBlock = await api.query.system.number();
  }
  return currentBlock;
}

function sleep(ms: number) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Sends the given unsigned `call` and waits for an event that fits `filter`.
 * @param {ApiPromise} api api object
 * @param {IEvent} filter which event to filter for
 * @param {SubmittableExtrinsic<Promise>} call a call that can be submitted to the chain
 * @param {boolean} intendedToFail If true a failed submission will be counted as a success.
 * @returns event that fits the filter
 */
export function sendUnsignedAndWaitFor<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
  intendedToFail: boolean
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>(function (resolve, reject) {
    call
      .send(function (res) {
        const { dispatchError, status } = res;
        if (dispatchError) {
          if (dispatchError.isModule) {
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          if (res.events.find(e => filter(e.event)) == undefined) return reject(status.toString());
          // @ts-ignore
          const event = res.events.find(e => filter(e.event)).event;
          if (filter(event)) {
            resolve(event);
          } else {
            reject(Error("Event record not found"));
          }
        }
      })
      .catch(function (e) {
        reject(Error(e.stack));
      });
  });
}

/**
 * Signs and sends the given `call` from `sender` and waits for an event that fits `filter`.
 * @param api api object
 * @param sender the sender of the transaction
 * @param filter which event to filter for
 * @param call a call that can be submitted to the chain
 * @param {boolean} intendedToFail If true a failed submission will be counted as a success.
 * @returns event that fits the filter
 */
export function sendAndWaitFor<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
  intendedToFail: boolean
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>(function (resolve, reject) {
    call
      .signAndSend(sender, { nonce: -1 }, function (res) {
        const { dispatchError, status } = res;
        if (dispatchError) {
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) resolve(event);
            }
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) resolve(event);
            }
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          if (res.events.find(e => filter(e.event)) == undefined) return reject(status.toString());
          // @ts-ignore
          const event = res.events.find(e => filter(e.event)).event;
          if (filter(event)) {
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) reject(event);
            }
            resolve(event);
          } else {
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) resolve(event);
            }
            reject(Error("1014: Priority is too low:"));
          }
        }
      })
      .catch(function (e) {
        reject(Error(e.stack));
      });
  });
}

/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
export function sendAndWaitForWithBatch<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">[],
  intendedToFail: boolean
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>(function (resolve, reject) {
    api.tx.utility
      .batchAll(call)
      .signAndSend(sender, { nonce: -1 }, function (res) {
        const { dispatchError, status } = res;
        if (dispatchError) {
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) resolve(event);
            }
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) resolve(event);
            }
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          if (res.events.find(e => filter(e.event)) == undefined) return reject(status.toString());
          // @ts-ignore
          const event = res.events.find(e => filter(e.event)).event;
          if (filter(event)) {
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) reject(event);
            }
            resolve(event);
          } else {
            if (intendedToFail) {
              // @ts-ignore
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event)) resolve(event);
            }
            reject(Error("1014: Priority is too low:"));
          }
        }
      })
      .catch(function (e) {
        reject(Error(e.stack));
      });
  });
}
