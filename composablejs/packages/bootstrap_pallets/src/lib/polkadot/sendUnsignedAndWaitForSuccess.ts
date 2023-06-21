import { ApiPromise } from "@polkadot/api";
import { AnyTuple, IEvent } from "@polkadot/types/types";
import { SubmittableExtrinsic } from "@polkadot/api/types";

export async function sendUnsignedAndWaitForSuccess<T extends AnyTuple>(
  api: ApiPromise,
  filter: (event: IEvent<AnyTuple>) => event is IEvent<T>,
  call: SubmittableExtrinsic<"promise">
): Promise<IEvent<T>> {
  return await sendUnsignedAndWaitFor(api, filter, call);
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
  call: SubmittableExtrinsic<"promise">
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
          // @ts-ignore
          const event = res.events.find(e => filter(e.event)).event;
          if (filter(event)) {
            resolve(event);
          } else {
            reject(Error("Event record not found"));
          }
        }
      })
      .catch(e => {
        reject(Error(e.message));
      });
  });
}
