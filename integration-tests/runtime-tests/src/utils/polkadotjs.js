"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.sendAndWaitForWithBatch = exports.sendAndWaitFor = exports.sendUnsignedAndWaitFor = exports.waitForBlockHandler = exports.waitForBlocks = exports.sendWithBatchAndWaitForSuccess = exports.sendAndWaitForSuccess = exports.sendUnsignedAndWaitForSuccess = void 0;
/**
 * Sends an unsigned extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
async function sendUnsignedAndWaitForSuccess(api, filter, call, intendedToFail = false) {
    return await sendUnsignedAndWaitFor(api, filter, call, intendedToFail);
}
exports.sendUnsignedAndWaitForSuccess = sendUnsignedAndWaitForSuccess;
/**
 * Sends a signed extrinsic and waits for success.
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
async function sendAndWaitForSuccess(api, sender, filter, call, intendedToFail = false) {
    return await sendAndWaitFor(api, sender, filter, call, intendedToFail);
}
exports.sendAndWaitForSuccess = sendAndWaitForSuccess;
/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
async function sendWithBatchAndWaitForSuccess(api, sender, filter, call, intendedToFail) {
    return await sendAndWaitForWithBatch(api, sender, filter, call, intendedToFail);
}
exports.sendWithBatchAndWaitForSuccess = sendWithBatchAndWaitForSuccess;
/**
 * Waits for N amount of blocks.
 * @param {ApiPromise} api Connected API Client.
 * @param {number} n Amount of blocks.
 * @return The current block number after waiting.
 */
async function waitForBlocks(api, n = 1) {
    return await waitForBlockHandler(api, n);
}
exports.waitForBlocks = waitForBlocks;
/**
 * Helper to wait for n blocks.
 * @param {ApiPromise} api Connected API Client.
 * @param {number} n Block wait duration.
 * @return The current block number after waiting.
 */
async function waitForBlockHandler(api, n) {
    const originBlock = await api.query.system.number();
    let currentBlock = await api.query.system.number();
    while (currentBlock.toNumber() < originBlock.toNumber() + n) {
        await sleep(3000);
        currentBlock = await api.query.system.number();
    }
    return currentBlock;
}
exports.waitForBlockHandler = waitForBlockHandler;
function sleep(ms) {
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
function sendUnsignedAndWaitFor(api, filter, call, intendedToFail) {
    return new Promise(function (resolve, reject) {
        call
            .send(function (res) {
            const { dispatchError, status } = res;
            if (dispatchError) {
                if (dispatchError.isModule) {
                    const decoded = api.registry.findMetaError(dispatchError.asModule);
                    const { docs, name, section } = decoded;
                    reject(Error(`${section}.${name}: ${docs.join(" ")}`));
                }
                else {
                    reject(Error(dispatchError.toString()));
                }
            }
            if (status.isInBlock || status.isFinalized) {
                if (res.events.find(e => filter(e.event)) == undefined)
                    return reject(status.toString());
                const event = res.events.find(e => filter(e.event)).event;
                if (filter(event)) {
                    resolve(event);
                }
                else {
                    reject(Error("Event record not found"));
                }
            }
        })
            .catch(function (e) {
            reject(Error(e.stack));
        });
    });
}
exports.sendUnsignedAndWaitFor = sendUnsignedAndWaitFor;
/**
 * Signs and sends the given `call` from `sender` and waits for an event that fits `filter`.
 * @param api api object
 * @param sender the sender of the transaction
 * @param filter which event to filter for
 * @param call a call that can be submitted to the chain
 * @param {boolean} intendedToFail If true a failed submission will be counted as a success.
 * @returns event that fits the filter
 */
function sendAndWaitFor(api, sender, filter, call, intendedToFail) {
    return new Promise(function (resolve, reject) {
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
                }
                else {
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
                }
                else {
                    if (intendedToFail) {
                        const event = res.events.find(e => filter(e.event)).event;
                        if (filter(event))
                            resolve(event);
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
exports.sendAndWaitFor = sendAndWaitFor;
/**
 * Sends multiple signed extrinsics and waits for success
 * @param {ApiPromise} api Connected API Client.
 * @param {AddressOrPair} sender Wallet initiating the transaction.
 * @param {IEvent<AnyTuple>} filter Success event to be waited for.
 * @param {SubmittableExtrinsic<Promise>} call Extrinsic call.
 * @param {boolean} intendedToFail If set to true the transaction is expected to fail.
 * @returns event that fits the filter
 */
function sendAndWaitForWithBatch(api, sender, filter, call, intendedToFail) {
    return new Promise(function (resolve, reject) {
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
                        const event = res.events.find(e => filter(e.event)).event;
                        if (filter(event))
                            resolve(event);
                    }
                    reject(Error(`${section}.${name}: ${docs.join(" ")}`));
                }
                else {
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
                }
                else {
                    if (intendedToFail) {
                        const event = res.events.find(e => filter(e.event)).event;
                        if (filter(event))
                            resolve(event);
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
exports.sendAndWaitForWithBatch = sendAndWaitForWithBatch;
//# sourceMappingURL=polkadotjs.js.map