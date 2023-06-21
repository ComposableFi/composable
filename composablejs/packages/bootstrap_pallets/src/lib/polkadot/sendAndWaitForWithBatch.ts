import { ApiPromise } from "@polkadot/api";
import { AddressOrPair, SubmittableExtrinsic } from "@polkadot/api/types";
import { AnyTuple } from "@polkadot/types-codec/types";
import { IEvent } from "@polkadot/types/types";

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
        .batch(call)
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