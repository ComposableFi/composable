import { ApiPromise } from '@polkadot/api';
import { AnyTuple, IEvent } from '@polkadot/types/types';
import { SubmittableExtrinsic, AddressOrPair } from '@polkadot/api/types';

export async function sendUnsignedAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return await sendUnsignedAndWaitFor(api, filter, call);
}

export async function sendAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return await sendAndWaitFor(api, sender, filter, call);
}

/**
 * Sends the given unsigned `call` and waits for an event that fits `filter`.
 * @param api api object
 * @param filter which event to filter for
 * @param call a call that can be submitted to the chain
 * @returns event that fits the filter
 */
export function sendUnsignedAndWaitFor<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>((resolve, reject) => {
    call
      .send(res => {
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
          const event = res.events.find(e => filter(e.event)).event;
          if (filter(event)) {
            resolve(event);
          } else {
            reject(Error("Event record not found"));
          }
        }
      })
      .catch((e) => {
        reject(Error(e.message));
      });
  });
}

/**
 * Signs and sends the given `call` from `sender` and waits for an event that fits `filter`.
 * @param api api object
 * @param sender the sender of the transaction
 * @param filter which event to filter for
 * @param call a call that can be submitted to the chain
 * @returns event that fits the filter
 */
export function sendAndWaitFor<T extends AnyTuple>(
  api: ApiPromise,
  sender: AddressOrPair,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">,
): Promise<IEvent<T>> {
  return new Promise<IEvent<T>>((resolve, reject) => {
    call
      .signAndSend(sender, { nonce: -1 }, res => {
        const { dispatchError, status } = res;
        if (dispatchError) {
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = api.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;
            reject(Error(`${section}.${name}: ${docs.join(" ")}`));
          } else {
            reject(Error(dispatchError.toString()));
          }
        }
        if (status.isInBlock || status.isFinalized) {
          const event = res.events.find(e => filter(e.event)).event;
          if (filter(event)) {
            resolve(event);
          } else {
            reject(Error("Event record not found"));
          }
        }
      })
      .catch((e) => {
        reject(Error(e.message));
      });
  });
}
