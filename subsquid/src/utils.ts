import {
  EventHandlerContext,
  Store,
  SubstrateBlock,
  SubstrateEvent,
} from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import { randomFill, randomUUID } from "crypto";
import * as ss58 from "@subsquid/ss58";

export function createCtx(storeMock: Store, blockHeight: number) {
  let blockMock: SubstrateBlock = mock<SubstrateBlock>();
  blockMock.height = blockHeight;
  let event: SubstrateEvent = mock<SubstrateEvent>();
  event.id = randomUUID();
  let ctxMock: EventHandlerContext = mock<EventHandlerContext>();
  let ctx: EventHandlerContext = instance(ctxMock);
  ctx.store = instance(storeMock);
  ctx.block = blockMock;
  ctx.event = event;

  return ctx;
}

export function createAccount() {
  let acc = Uint8Array.of(...new Array<any>(32));
  randomFill(acc, (err) => (err != null ? console.log(err) : ""));
  return acc;
}

export function encodeAccount(account: Uint8Array) {
  return ss58.codec("picasso").encode(account);
}
