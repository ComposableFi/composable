import {
  EventHandlerContext,
  Store,
  SubstrateBlock,
  SubstrateEvent,
} from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import { randomUUID } from "crypto";
import * as ss58 from "@subsquid/ss58";
import { Account, PicassoTransaction, PicassoTransactionType } from "./model";

const BOB = "5woQTSqveJemxVbj4eodiBTSVfC4AAJ8CQS7SoyoyHWW7MA6";

export function createCtx(
  storeMock: Store,
  blockHeight: number
): EventHandlerContext {
  const blockMock: SubstrateBlock = mock<SubstrateBlock>();
  blockMock.height = blockHeight;
  const event: SubstrateEvent = mock<SubstrateEvent>();
  event.id = randomUUID();
  const ctxMock: EventHandlerContext = mock<EventHandlerContext>();
  const ctx: EventHandlerContext = instance(ctxMock);
  ctx.store = instance(storeMock);
  ctx.block = blockMock;
  ctx.event = event;

  return ctx;
}

export function createAccount(): Uint8Array {
  return ss58.codec("picasso").decode(BOB);
}

export function encodeAccount(account: Uint8Array): string {
  return ss58.codec("picasso").encode(account);
}

/**
 * Creates PicassoTransaction
 * @param ctx
 * @param who
 * @param transactionType
 * @param id
 */
export function createTransaction(
  ctx: EventHandlerContext,
  who: string,
  transactionType: PicassoTransactionType,
  id?: string
): PicassoTransaction {
  return new PicassoTransaction({
    id: id || randomUUID(),
    eventId: ctx.event.id,
    transactionId: ctx.event.id, // TODO: change
    who,
    transactionType,
    blockNumber: BigInt(ctx.block.height),
    date: new Date(ctx.block.timestamp),
  });
}

export function updateBalance(account: Account, ctx: EventHandlerContext) {
  const tip = ctx.extrinsic?.tip;

  if (tip) {
    account.balance = BigInt(account.balance || 0n) - BigInt(tip);
  }
}
