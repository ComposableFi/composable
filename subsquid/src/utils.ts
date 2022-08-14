import {
  EventHandlerContext,
  Store,
  SubstrateBlock,
  SubstrateEvent,
} from "@subsquid/substrate-processor";
import { instance, mock } from "ts-mockito";
import { randomUUID } from "crypto";
import * as ss58 from "@subsquid/ss58";
import {
  Account,
  Activity,
  PicassoTransaction,
  PicassoTransactionType,
} from "./model";
import { getOrCreate } from "./dbHelper";

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
 * Creates PicassoTransaction in database.
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

/**
 * Create or update account and store transaction in database.
 * @param ctx
 * @param transactionType
 */
export async function saveAccountAndTransaction(
  ctx: EventHandlerContext,
  transactionType: PicassoTransactionType
) {
  const signer = ctx.extrinsic?.signer;

  if (signer) {
    const account = await getOrCreate(ctx.store, Account, signer);

    const tx = createTransaction(ctx, signer, transactionType);

    await ctx.store.save(account);
    await ctx.store.save(tx);

    await saveActivity(ctx, signer);
  }
}

/**
 * Store Activity on the database.
 * @param ctx
 * @param transactionId
 * @param accountId
 */
export async function saveActivity(
  ctx: EventHandlerContext,
  transactionId: string,
  accountId?: string
) {
  const accId = accountId || ctx.extrinsic?.signer;

  if (accId) {
    const activity = new Activity({
      id: randomUUID(),
      eventId: ctx.event.id,
      transactionId,
      accountId: accId,
      timestamp: BigInt(ctx.block.timestamp),
    });

    await ctx.store.save(activity);
  }
}
