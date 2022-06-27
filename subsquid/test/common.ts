import {
  EventHandlerContext,
  Store,
  SubstrateBlock,
  SubstrateEvent,
} from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import { randomFill, randomUUID } from "crypto";

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
