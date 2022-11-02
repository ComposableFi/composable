import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Entity, Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { hexToU8a } from "@polkadot/util";
import { chain } from "./config";
import { encodeAccount, getAmountWithoutDecimals } from "./utils";
import {
  Account,
  Activity,
  Asset,
  Currency,
  Event,
  EventType,
  HistoricalLockedValue,
  HistoricalVolume,
  LockedSource,
  PabloPool,
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
    where: { poolId },
    order: { calculatedTimestamp: "DESC" },
    relations: {
      poolAssets: true,
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
    timestamp: BigInt(ctx.block.timestamp),
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
    timestamp: BigInt(ctx.block.timestamp),
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
 */
export async function storeHistoricalLockedValue(
  ctx: EventHandlerContext<Store>,
  amountsLocked: Record<string, bigint>,
  source: LockedSource
): Promise<void> {
  const wsProvider = new WsProvider(chain());
  const api = await ApiPromise.create({ provider: wsProvider });
  const oraclePrices: Record<string, bigint> = {};
  const assetDecimals: Record<string, number> = {};

  for (const assetId of Object.keys(amountsLocked)) {
    const asset = await ctx.store.get(Asset, {
      where: {
        id: assetId,
      },
    });

    if (asset?.decimals) {
      assetDecimals[assetId] = asset?.decimals || 12;
    }
  }

  try {
    for (const assetId of Object.keys(amountsLocked)) {
      const oraclePrice = await api.query.oracle.prices(assetId);
      if (!oraclePrice?.price) {
        return;
      }
      oraclePrices[assetId] = BigInt(oraclePrice.price.toString());
    }
  } catch (error) {
    console.error(error);
    return;
  }

  const netLockedValue = Object.keys(oraclePrices).reduce((agg, assetId) => {
    if (!assetDecimals[assetId]) {
      // Ignore assets for which we don't know decimals
      return agg;
    }
    const lockedValue =
      oraclePrices[assetId] *
      getAmountWithoutDecimals(amountsLocked[assetId], assetDecimals[assetId]);
    return BigInt(agg) + lockedValue;
  }, BigInt(0));

  const lastLockedValueAll = await getLastLockedValue(ctx, LockedSource.All);
  const lastLockedValueSource = await getLastLockedValue(ctx, source);
  const event = await ctx.store.get(Event, { where: { id: ctx.event.id } });

  if (!event) {
    return Promise.reject(new Error("Event not found"));
  }

  const historicalLockedValueAll = new HistoricalLockedValue({
    id: randomUUID(),
    event,
    amount: lastLockedValueAll + netLockedValue,
    currency: Currency.USD,
    timestamp: BigInt(new Date(ctx.block.timestamp).valueOf()),
    source: LockedSource.All,
  });

  const historicalLockedValueSource = new HistoricalLockedValue({
    id: randomUUID(),
    event,
    amount: lastLockedValueSource + netLockedValue,
    currency: Currency.USD,
    timestamp: BigInt(new Date(ctx.block.timestamp).valueOf()),
    source,
  });

  await ctx.store.save(historicalLockedValueAll);
  await ctx.store.save(historicalLockedValueSource);
}

/**
 * Stores a new HistoricalVolume for the specified quote asset
 * @param ctx
 * @param quoteAssetId
 * @param amount
 */
export async function storeHistoricalVolume(
  ctx: EventHandlerContext<Store>,
  quoteAssetId: string,
  amount: bigint
): Promise<void> {
  const wsProvider = new WsProvider(chain());
  const api = await ApiPromise.create({ provider: wsProvider });
  let assetPrice = 0n;

  try {
    const oraclePrice = await api.query.oracle.prices(quoteAssetId);
    if (!oraclePrice?.price) {
      // TODO: handle missing oracle price
      // NOTE: should we look at the latest known price for this asset?
      return;
    }
    assetPrice = BigInt(oraclePrice.price.toString());
  } catch (error) {
    console.error(error);
    return;
  }

  // TODO: get decimals for this asset
  // NOTE: normalize to 12 decimals for historical values?

  const volume = amount * assetPrice;

  const lastVolume = await getLastVolume(ctx, quoteAssetId);

  const event = await ctx.store.get(Event, { where: { id: ctx.event.id } });

  if (!event) {
    return Promise.reject(new Error("Event not found"));
  }

  const historicalVolume = new HistoricalVolume({
    id: randomUUID(),
    event,
    amount: lastVolume + volume,
    currency: Currency.USD,
    assetId: quoteAssetId,
    timestamp: BigInt(new Date(ctx.block.timestamp).valueOf()),
  });

  await ctx.store.save(historicalVolume);
}

/**
 * Get latest locked value
 */
export async function getLastLockedValue(
  ctx: EventHandlerContext<Store>,
  source: LockedSource
): Promise<bigint> {
  const lastLockedValue = await ctx.store.find(HistoricalLockedValue, {
    where: { source },
    order: { timestamp: "DESC" },
    relations: { event: true },
  });

  return BigInt(lastLockedValue.length > 0 ? lastLockedValue[0].amount : 0);
}

/**
 * Get latest volume
 */
export async function getLastVolume(
  ctx: EventHandlerContext<Store>,
  assetId: string
): Promise<bigint> {
  const lastVolume = await ctx.store.find(HistoricalVolume, {
    where: { assetId },
    order: { timestamp: "DESC" },
    relations: { event: true },
  });

  return BigInt(lastVolume.length > 0 ? lastVolume[0].amount : 0);
}
