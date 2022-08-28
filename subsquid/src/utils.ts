import {
  EventHandlerContext,
  Store,
  SubstrateBlock,
  SubstrateEvent,
} from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import * as ss58 from "@subsquid/ss58";
import { randomUUID } from "crypto";

export const BOB = "5woQTSqveJemxVbj4eodiBTSVfC4AAJ8CQS7SoyoyHWW7MA6";
export const CHARLIE = "5wr4XcyxyJYQb71PbSPxhqujKnsS9UAydBhSypGvFgh2QXBa";

export function createCtx(
  storeMock: Store,
  blockHeight: number
): EventHandlerContext {
  const blockMock: SubstrateBlock = mock<SubstrateBlock>();
  blockMock.height = blockHeight;
  blockMock.timestamp = 123; // TODO: use better example
  const event: SubstrateEvent = mock<SubstrateEvent>();
  event.id = randomUUID();
  const ctxMock: EventHandlerContext = mock<EventHandlerContext>();
  const ctx: EventHandlerContext = instance(ctxMock);
  ctx.store = instance(storeMock);
  ctx.block = blockMock;
  ctx.event = event;
  if (ctx.extrinsic) {
    ctx.extrinsic.signer = BOB;
  }

  return ctx;
}

export function createAccount(): Uint8Array {
  return ss58.codec("picasso").decode(BOB);
}

export function encodeAccount(account: Uint8Array): string {
  return ss58.codec("picasso").encode(account);
}
