import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Entity, Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";
import BigNumber from "bignumber.js";
import { chain } from "./config";
import { encodeAccount, getAmountWithoutDecimals } from "./utils";
import {
  Account,
  Activity,
  Asset,
  Currency,
  CurrentLockedValue,
  Event,
  EventType,
  HistoricalLockedValue,
  HistoricalVolume,
  LockedSource,
  PabloPool,
  PabloPoolAsset,
} from "./model";

export async function get<T extends { id: string }>(
  store: Store,
  EntityConstructor: EntityConstructor<T>,
  id: string
): Promise<T | undefined> {
  return store.get<T>(EntityConstructor, id);
}

export async function getLatestPoolByPoolId<T extends Entity>(
  store: Store,
  poolId: bigint
): Promise<PabloPool | undefined> {
  return store.get<PabloPool>(PabloPool, {
    where: { id: poolId.toString() },
    order: { timestamp: "DESC" },
    relations: {
      poolAssets: true,
      poolAssetWeights: true,
    },
  });
}

export async function getOrCreate<T extends Entity>(
  store: Store,
  EntityConstructor: EntityConstructor<T>,
  id: string
): Promise<T> {
  let entity = await store.get<T>(EntityConstructor, id);

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
 * Create or update account and store event in database.
 * When `accountId` is not defined, signer of extrinsic will be used.
 * When the extrinsic is not signed, it will be a noop.
 * Returns the `accountId` stored, or undefined if nothing is stored.
 * @param ctx
 * @param accountId
 *
 * @returns string | undefined
 */
export async function getOrCreateAccount(
  ctx: EventHandlerContext<Store>,
  accountId?: string
): Promise<Account | undefined> {
  const accId = accountId || ctx.event.extrinsic?.signature?.address;

  if (!accId) {
    // no-op
    return undefined;
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

  return account;
}

/**
 * Create and store Event on database.
 *
 * Returns the stored event id.
 * @param ctx
 * @param eventType
 */
export async function saveEvent(
  ctx: EventHandlerContext<Store>,
  eventType: EventType
): Promise<Event> {
  const accountId: string = ctx.event.extrinsic?.signature?.address.value
    ? encodeAccount(hexToU8a(ctx.event.extrinsic?.signature?.address.value))
    : ctx.event.extrinsic?.signature?.address;

  // Create event
  const event = new Event({
    id: ctx.event.id,
    accountId,
    eventType,
    blockNumber: BigInt(ctx.block.height),
    timestamp: new Date(ctx.block.timestamp),
  });

  // Store event
  await ctx.store.save(event);

  return event;
}

/**
 * Store Activity on the database.
 * @param ctx
 * @param event
 * @param accountId
 */
export async function saveActivity(
  ctx: EventHandlerContext<Store>,
  event: Event,
  accountId: string
): Promise<string> {
  const activity = new Activity({
    id: randomUUID(),
    event,
    accountId,
    timestamp: new Date(ctx.block.timestamp),
  });

  await ctx.store.save(activity);

  return activity.id;
}

/**
 * Saves the given Accounts, an Event for the first account, and
 * Activities for every account.
 * If no account id is provided, it will try to create an account using the
 * signer of the underlying extrinsic.
 * If no account is created, it will NOT create any Event or Activity
 * @param ctx
 * @param eventType
 * @param accountId
 */
export async function saveAccountAndEvent(
  ctx: EventHandlerContext<Store>,
  eventType: EventType,
  accountId?: string | string[]
): Promise<{ accounts: Account[]; event: Event }> {
  const accountIds: (string | undefined)[] =
    typeof accountId === "string" ? [accountId] : accountId || [];

  const event = await saveEvent(ctx, eventType);

  const accounts: Account[] = [];

  for (let index = 0; index < accountIds.length; index += 1) {
    const id = accountIds[index];
    if (!id) {
      // no-op
      return Promise.reject("Missing account id");
    }
    const account = await getOrCreateAccount(ctx, id);
    if (account) {
      accounts.push(account);
      await saveActivity(ctx, event, id);
    }
  }

  return Promise.resolve({ accounts, event });
}

/**
 * Stores a new HistoricalLockedValue with current locked amount
 * for the specified source, and for the overall locked value
 * @param ctx
 * @param amountsLocked
 * @param source
 * @param sourceEntityId
 */
export async function storeHistoricalLockedValue(
  ctx: EventHandlerContext<Store>,
  amountsLocked: [string, bigint][], // [assetId, amountLocked]
  source: LockedSource,
  sourceEntityId: string
): Promise<void> {
  const event = await ctx.store.get(Event, { where: { id: ctx.event.id } });

  if (!event) {
    // no-op
    return;
  }

  for (const [assetId, amount] of amountsLocked) {
    const lastAccumulatedValue =
      (
        await ctx.store.findOne(HistoricalLockedValue, {
          where: {
            source: LockedSource.Pablo,
            assetId: assetId.toString(),
            sourceEntityId,
          },
        })
      )?.accumulatedAmount || 0n;

    const historicalLockedValue = new HistoricalLockedValue({
      id: randomUUID(),
      event,
      amount,
      accumulatedAmount: lastAccumulatedValue + amount,
      timestamp: new Date(ctx.block.timestamp),
      source,
      assetId,
      sourceEntityId,
    });

    await ctx.store.save(historicalLockedValue);
  }
}

/**
 * Creates asset fpr Pablo pool
 * @param pool
 * @param assetId
 */
export function createPabloAsset(
  pool: PabloPool,
  assetId: string
): PabloPoolAsset {
  return new PabloPoolAsset({
    id: randomUUID(),
    assetId,
    pool,
    totalLiquidity: BigInt(0),
    totalVolume: BigInt(0),
  });
}

/**
 * Get Pablo pool asset by asset id and pool id. If it doesn't exist, create it.
 * @param ctx
 * @param pool
 * @param assetId
 */
export async function getOrCreatePabloAsset(
  ctx: EventHandlerContext<Store>,
  pool: PabloPool,
  assetId: string
): Promise<PabloPoolAsset> {
  let pabloAsset = await ctx.store.get(PabloPoolAsset, {
    where: {
      assetId,
      pool: {
        id: pool.id,
      },
    },
  });
  if (!pabloAsset) {
    pabloAsset = createPabloAsset(pool, assetId);
  }
  return Promise.resolve(pabloAsset);
}
