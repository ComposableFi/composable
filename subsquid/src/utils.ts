import { EventHandlerContext, SubstrateBlock, SubstrateEvent } from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import * as ss58 from "@subsquid/ss58";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import BigNumber from "bignumber.js";
import { RequestInfo, RequestInit } from "node-fetch";

export const BOB = "5woQTSqveJemxVbj4eodiBTSVfC4AAJ8CQS7SoyoyHWW7MA6";
export const CHARLIE = "5wr4XcyxyJYQb71PbSPxhqujKnsS9UAydBhSypGvFgh2QXBa";

export function createCtx(storeMock: Store, blockHeight: number): EventHandlerContext<Store, { event: true }> {
  const blockMock: SubstrateBlock = mock<SubstrateBlock>();
  blockMock.height = blockHeight;
  blockMock.timestamp = 123; // TODO: use better example
  const event: SubstrateEvent = mock<SubstrateEvent>();
  event.id = randomUUID();
  const ctxMock: EventHandlerContext<Store, { event: true }> = mock<EventHandlerContext<Store, { event: true }>>();
  const ctx: EventHandlerContext<Store, { event: true }> = instance(ctxMock);
  ctx.store = instance(storeMock);
  ctx.block = blockMock;
  ctx.event = event;
  if (ctx.event.extrinsic?.signature?.address) {
    ctx.event.extrinsic.signature.address = BOB;
  }

  return ctx;
}

export function createAccount(): Uint8Array {
  return ss58.codec("picasso").decode(BOB);
}

export function encodeAccount(account: Uint8Array): string {
  return ss58.codec("picasso").encode(account);
}

// Get amount without decimals
export function getAmountWithoutDecimals(amount: bigint, decimals: number): BigNumber {
  return BigNumber(amount.toString()).div(BigNumber(10 ** decimals));
}

export function divideBigInts(a: bigint, b: bigint): number {
  const quote = BigNumber(a.toString());
  const base = BigNumber(b.toString());
  return quote.div(base).toNumber();
}

export const fetch = <TResponse>(url: RequestInfo, init?: RequestInit): Promise<TResponse> =>
  import("node-fetch").then(({ default: nodeFetch }) => {
    return nodeFetch(url, init)
      .then(res => {
        if (res.ok) {
          return res.json();
        }
        throw new Error(res.statusText);
      })
      .then(data => data as TResponse)
      .catch(err => {
        throw new Error(err);
      });
  });

export const fetchRetry = async <TResponse>(url: RequestInfo, init?: RequestInit, retries = 5): Promise<TResponse> => {
  return fetch<TResponse>(url, init)
    .then(res => res)
    .catch(async err => {
      if (retries > 0) {
        await new Promise(resolve => setTimeout(resolve, 1000));
        console.log("Retrying...", url);
        return fetchRetry(url, init, retries - 1);
      }
      throw new Error(err);
    });
};
