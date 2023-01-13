import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import { hexToU8a } from "@polkadot/util";
import { EntityManager, LessThan } from "typeorm";
import { divideBigInts, encodeAccount } from "./utils";
import {
  Account,
  Activity,
  Event,
  EventType,
  HistoricalLockedValue,
  LockedSource,
  PabloAssetWeight,
  PabloPool,
  PabloPoolAsset,
  PabloSwap
} from "./model";

export async function getLatestPoolByPoolId(store: Store, poolId: bigint): Promise<PabloPool | undefined> {
  return store.get<PabloPool>(PabloPool, {
    where: { id: poolId.toString() },
    order: { timestamp: "DESC" },
    relations: {
      poolAssets: true,
      poolAssetWeights: true
    }
  });
}

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
    where: { id: accId }
  });

  if (!account) {
    account = new Account();
  }

  account.id = accId;
  account.eventId = ctx.event.id;
  account.blockId = ctx.block.id;

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
export async function saveEvent(ctx: EventHandlerContext<Store>, eventType: EventType): Promise<Event> {
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
    blockId: ctx.block.hash
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
export async function saveActivity(ctx: EventHandlerContext<Store>, event: Event, accountId: string): Promise<string> {
  const activity = new Activity({
    id: randomUUID(),
    event,
    accountId,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash
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
  const accountIds: (string | undefined)[] = typeof accountId === "string" ? [accountId] : accountId || [];

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
  let event = await ctx.store.get(Event, { where: { id: ctx.event.id } });

  if (!event) {
    event = await saveEvent(ctx, EventType.SWAP);
  }

  for (const [assetId, amount] of amountsLocked) {
    const lastAccumulatedValue =
      (
        await ctx.store.findOne(HistoricalLockedValue, {
          where: {
            source,
            assetId,
            sourceEntityId
          },
          order: {
            timestamp: "DESC"
          }
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
      blockId: ctx.block.hash
    });

    await ctx.store.save(historicalLockedValue);
  }
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
        id: pool.id
      }
    }
  });
  if (!pabloAsset) {
    const weight = await ctx.store.get(PabloAssetWeight, {
      where: {
        assetId,
        pool: {
          id: pool.id
        }
      }
    });
    pabloAsset = new PabloPoolAsset({
      id: randomUUID(),
      assetId,
      pool,
      totalLiquidity: BigInt(0),
      totalVolume: BigInt(0),
      blockId: ctx.block.hash,
      weight: weight?.weight || 0
    });
  }
  return Promise.resolve(pabloAsset);
}

export async function getSpotPrice(
  ctx: EventHandlerContext<Store> | EntityManager,
  quoteAssetId: string,
  baseAssetId: string,
  poolId: string,
  timestamp?: number
): Promise<number> {
  const isRepository = ctx instanceof EntityManager;

  const time = timestamp || new Date().getTime();

  const swap1 = isRepository
    ? await ctx.getRepository(PabloSwap).findOne({
        where: {
          baseAssetId,
          quoteAssetId,
          pool: {
            id: poolId
          },
          timestamp: LessThan(new Date(time))
        },
        order: {
          timestamp: "DESC"
        }
      })
    : await ctx.store.get(PabloSwap, {
        where: {
          baseAssetId,
          quoteAssetId,
          pool: {
            id: poolId
          },
          timestamp: LessThan(new Date(time))
        },
        order: {
          timestamp: "DESC"
        }
      });

  const swap2 = isRepository
    ? await ctx.getRepository(PabloSwap).findOne({
        where: {
          baseAssetId: quoteAssetId,
          quoteAssetId: baseAssetId,
          pool: {
            id: poolId
          },
          timestamp: LessThan(new Date(time))
        },
        order: {
          timestamp: "DESC"
        }
      })
    : await ctx.store.get(PabloSwap, {
        where: {
          baseAssetId: quoteAssetId,
          quoteAssetId: baseAssetId,
          pool: {
            id: poolId
          },
          timestamp: LessThan(new Date(time))
        },
        order: {
          timestamp: "DESC"
        }
      });

  const timestamp1 = swap1?.timestamp;
  const timestamp2 = swap2?.timestamp;

  let swap: PabloSwap;

  if (timestamp1 && !timestamp2) {
    swap = swap1;
  } else if (!timestamp1 && timestamp2) {
    swap = swap2;
  } else if (timestamp1 && timestamp2) {
    swap = timestamp1 > timestamp2 ? swap1 : swap2;
  } else {
    // If no timestamp, we need to calculate the spot price using the liquidity
    const baseWhere = {
      assetId: baseAssetId,
      pool: {
        id: poolId
      }
    };
    const baseAsset = isRepository
      ? await ctx.getRepository(PabloPoolAsset).findOne({
          where: baseWhere
        })
      : await ctx.store.findOne(PabloPoolAsset, { where: baseWhere });

    const quoteWhere = {
      assetId: baseAssetId,
      pool: {
        id: poolId
      }
    };
    const quoteAsset = isRepository
      ? await ctx.getRepository(PabloPoolAsset).findOne({
          where: quoteWhere
        })
      : await ctx.store.findOne(PabloPoolAsset, { where: quoteWhere });

    if (!baseAsset || !quoteAsset) {
      throw new Error("No liquidity data for this pool. Can't compute spot price.");
    }

    const baseAssetWeight = isRepository
      ? await ctx.getRepository(PabloAssetWeight).findOne({
          where: baseWhere
        })
      : await ctx.store.findOne(PabloAssetWeight, { where: baseWhere });

    const quoteAssetWeight = isRepository
      ? await ctx.getRepository(PabloAssetWeight).findOne({
          where: baseWhere
        })
      : await ctx.store.findOne(PabloAssetWeight, { where: quoteWhere });

    const weightRatio =
      baseAssetWeight?.weight && quoteAssetWeight?.weight ? baseAssetWeight.weight / quoteAssetWeight.weight : 1;

    const quoteTotalLiquidity = (quoteAssetId === "130" ? 1_000_000n : 1n) * quoteAsset.totalLiquidity;
    const baseTotalLiquidity = (baseAssetId === "130" ? 1_000_000n : 1n) * baseAsset.totalLiquidity;

    return divideBigInts(quoteTotalLiquidity, baseTotalLiquidity) * weightRatio;
  }

  return baseAssetId === swap.baseAssetId ? Number(swap.spotPrice) : 1 / Number(swap.spotPrice);
}
