import {
  EventHandlerContext,
  SubstrateBlock,
  SubstrateEvent,
} from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import * as ss58 from "@subsquid/ss58";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import {
  Currency,
  Event,
  HistoricalLockedValue,
  LockedSource,
} from "subsquid/model";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { chain } from "subsquid/config";
import { getLastLockedValue } from "subsquid/dbHelper";

export const BOB = "5woQTSqveJemxVbj4eodiBTSVfC4AAJ8CQS7SoyoyHWW7MA6";
export const CHARLIE = "5wr4XcyxyJYQb71PbSPxhqujKnsS9UAydBhSypGvFgh2QXBa";

export function createCtx(
  storeMock: Store,
  blockHeight: number
): EventHandlerContext<Store, { event: true }> {
  const blockMock: SubstrateBlock = mock<SubstrateBlock>();
  blockMock.height = blockHeight;
  blockMock.timestamp = 123; // TODO: use better example
  const event: SubstrateEvent = mock<SubstrateEvent>();
  event.id = randomUUID();
  const ctxMock: EventHandlerContext<Store, { event: true }> =
    mock<EventHandlerContext<Store, { event: true }>>();
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
