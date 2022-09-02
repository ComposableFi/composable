import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import { ApiPromise, WsProvider } from "@polkadot/api";
import {
  Account,
  Activity,
  HistoricalLockedValue,
  PabloPool,
  PicassoPool,
  StakingPosition,
  Transaction,
  TransactionType,
} from "./model";
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
 * Create and store Transaction on database.
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
  transactionType: TransactionType,
  id: string
): Promise<string> {
  // Create transaction
  const tx = new Transaction({
    id,
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

/**
 * Saves the given Accounts, a Transaction for the first account, and
 * Activities for every account.
 * If no account id is provided, it will try to create an account using the
 * signer of the underlying extrinsic.
 * If no account is created, it will NOT create any Transaction or Activity
 * @param ctx
 * @param transactionType
 * @param accountId
 */
export async function saveAccountAndTransaction(
  ctx: EventHandlerContext,
  transactionType: TransactionType,
  accountId?: string | string[]
): Promise<{ transactionId: string }> {
  const accountIds: (string | undefined)[] =
    typeof accountId === "string" ? [accountId] : accountId || [undefined];

  const transactionId = randomUUID();

  for (let index = 0; index < accountIds.length; index += 1) {
    const id = accountIds[index];
    if (!id) {
      // no-op
      return Promise.reject();
    }
    const isSaved = await trySaveAccount(ctx, id);
    if (isSaved) {
      if (index === 0) {
        await saveTransaction(ctx, id, transactionType, transactionId);
      }
      await saveActivity(ctx, transactionId, id);
    }
  }

  return Promise.resolve({ transactionId });
}

/**
 * Stores a new HistoricalLockedValue with current locked amount
 * @param ctx
 * @param amountLocked
 * @param eventId
 * @param assetId
 */
export async function storeHistoricalLockedValue(
  ctx: EventHandlerContext,
  amountLocked: bigint,
  eventId: string,
  assetId: string
): Promise<void> {
  const wsProvider = new WsProvider("ws://127.0.0.1:9988");
  const api = await ApiPromise.create({ provider: wsProvider });

  const oraclePrice = await api.query.oracle.prices(assetId);

  if (!oraclePrice?.price) {
    // no-op.
    return;
  }

  const assetPrice = BigInt(oraclePrice.price.toString());

  const lastLockedValue = await getLastLockedValue(ctx);

  const historicalLockedValue = new HistoricalLockedValue({
    id: randomUUID(),
    eventId,
    amount: lastLockedValue + amountLocked * assetPrice,
    timestamp: BigInt(new Date().valueOf()),
  });

  await ctx.store.save(historicalLockedValue);
}

/**
 * Get asset id and price id from Picasso pool id
 * @param ctx
 * @param poolId
 */
export async function getAssetIdFromPicassoPoolId(
  ctx: EventHandlerContext,
  poolId: bigint
): Promise<string> {
  const picassoPool = await ctx.store.get(PicassoPool, {
    where: { poolId: poolId.toString() },
  });

  if (!picassoPool) {
    return Promise.reject(new Error(`Pool ${poolId} does not exist.`));
  }

  return Promise.resolve(picassoPool.assetId);
}

/**
 * Get latest locked value
 */
export async function getLastLockedValue(
  ctx: EventHandlerContext
): Promise<bigint> {
  const lastLockedValue: { amount: bigint }[] = await ctx.store.query(
    `
      SELECT amount
      FROM historical_locked_value
      ORDER BY timestamp DESC
      LIMIT 1
      `
  );

  let lastAmount = 0n;

  if (lastLockedValue?.[0]) {
    lastAmount = BigInt(lastLockedValue[0].amount);
  }

  return Promise.resolve(lastAmount);
}

export async function mockData(ctx: EventHandlerContext) {
  // await ctx.store.save(StakingPosition);
}
