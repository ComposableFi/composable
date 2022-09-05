import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import {
  Account,
  Activity,
  PabloPool,
  PicassoTransaction,
  PicassoTransactionType,
} from "./model";
import { randomUUID } from "crypto";
import { BOB } from "./utils";

export async function get<T extends { id: string }>(
  store: Store,
  EntityConstructor: EntityConstructor<T>,
  id: string
): Promise<T | undefined> {
  return store.get<T>(EntityConstructor, {
    where: { id },
  });
}

export async function getLatestPoolByPoolId<T extends { id: string }>(
  store: Store,
  poolId: bigint
): Promise<PabloPool | undefined> {
  return store.get<PabloPool>(PabloPool, {
    where: { poolId },
    order: { calculatedTimestamp: "DESC" },
    relations: ["poolAssets"],
  });
}

export async function getOrCreate<T extends { id: string }>(
  store: Store,
  EntityConstructor: EntityConstructor<T>,
  id: string
): Promise<T> {
  let entity = await store.get<T>(EntityConstructor, {
    where: { id },
  });

  if (entity === undefined) {
    entity = new EntityConstructor();
    entity.id = id;
  }

  return entity;
}

export type EntityConstructor<T> = {
  new (...args: any[]): T;
};

/**
 * Create or update account and store transaction in database.
 * When `accountId` is not defined, signer of extrinsic will be used.
 * When the extrinsic is not signed, it will be a noop.
 * Returns the `accountId` stored, or undefined if nothing is stored.
 * @param ctx
 * @param accountId
 *
 * @returns string | undefined
 */
export async function trySaveAccount(
  ctx: EventHandlerContext,
  accountId?: string
): Promise<string | undefined> {
  let accId = accountId || ctx.extrinsic?.signer;

  if (process.env.npm_lifecycle_event === "test") {
    accId = BOB;
  }

  if (!accId) {
    // no-op
    return;
  }

  let account: Account | undefined = await ctx.store.get(Account, {
    where: { id: accId },
  });

  if (!account) {
    account = new Account();
  }

  account.id = accId;
  account.eventId = ctx.event.id;

  await ctx.store.save(account);

  return accId;
}

/**
 * Create and store PicassoTransaction on database.
 * If `id` is not defined, a random id will be generated.
 *
 * Returns the stored transaction id.
 * @param ctx
 * @param accountId
 * @param transactionType
 * @param id
 */
export async function saveTransaction(
  ctx: EventHandlerContext,
  accountId: string,
  transactionType: PicassoTransactionType,
  id?: string
): Promise<string> {
  // Create transaction
  const tx = new PicassoTransaction({
    id: id || randomUUID(),
    eventId: ctx.event.id,
    accountId,
    transactionType,
    blockNumber: BigInt(ctx.block.height),
    timestamp: BigInt(ctx.block.timestamp),
  });

  // Store transaction
  await ctx.store.save(tx);

  return tx.id;
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
  accountId: string
): Promise<string> {
  const activity = new Activity({
    id: randomUUID(),
    eventId: ctx.event.id,
    transactionId,
    accountId,
    timestamp: BigInt(ctx.block.timestamp),
  });

  await ctx.store.save(activity);

  return activity.id;
}
