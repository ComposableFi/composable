import { ApiPromise } from '@polkadot/api';
import { AnyTuple, IEvent } from '@polkadot/types/types';
import { SubmittableExtrinsic, AddressOrPair } from '@polkadot/api/types';

export async function sendUnsignedAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
  intendedToFail=false
): Promise<IEvent<T>> {
  return await sendUnsignedAndWaitFor(api, filter, call, intendedToFail);
}

export async function sendAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
  intendedToFail=false
): Promise<IEvent<T>> {
  return await sendAndWaitFor(api, sender, filter, call, intendedToFail);
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
  intendedToFail:boolean
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
          if (res.events.find(e => filter(e.event)) == undefined)
            return reject(status.toString());
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
  intendedToFail:boolean
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
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event))
                resolve(event);
            }
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            if (intendedToFail) {
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event))
                resolve(event);
            }
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          if (res.events.find(e => filter(e.event)) == undefined)
            return reject(status.toString());
          const event = res.events.find(e => filter(e.event)).event;
          if (filter(event)) {
            if (intendedToFail) {
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event))
                reject(event);
            }
            resolve(event);
          } else {
            if (intendedToFail) {
              const event = res.events.find(e => filter(e.event)).event;
              if (filter(event))
                resolve(event);
            }
            reject(Error("Event record not found"));
          }
        }
      })
      .catch(function (e) {
        reject(Error(e.stack));
      });
  });
}
