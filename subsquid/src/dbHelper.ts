import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { SubstrateExtrinsicSignature } from "@subsquid/substrate-processor";
import BigNumber from "bignumber.js";
import { EntityManager, LessThan, MoreThan } from "typeorm";
import { isInstance } from "class-validator";
import { divideBigInts, fetch, fetchRetry, getAccountFromSignature } from "./utils";
import {
  Account,
  Activity,
  CallError,
  Currency,
  Event,
  EventType,
  HistoricalAssetPrice,
  HistoricalLockedValue,
  HistoricalPabloFeeApr,
  HistoricalStakingApr,
  LockedSource,
  PabloAssetWeight,
  PabloLpToken,
  PabloPool,
  PabloPoolAsset,
  PabloSwap,
  StakingRewardsPool
} from "./model";
import { Block, Context, EventItem } from "./processorTypes";
import { AssetId, AssetInfo, assetList, CoingeckoPrices, DAY_IN_MS } from "./constants";

const provider = new WsProvider("wss://rpc.composablenodes.tech");
const api = new ApiPromise({ provider });

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
 * @param block
 * @param eventItem
 * @param accountId
 *
 * @returns string | undefined
 */
export async function getOrCreateAccount(
  ctx: Context,
  block: Block,
  eventItem: EventItem,
  accountId?: string
): Promise<Account | undefined> {
  let signature: SubstrateExtrinsicSignature | undefined;
  if ("extrinsic" in eventItem.event) {
    signature = eventItem.event.extrinsic?.signature;
  }
  const accId = accountId || getAccountFromSignature(signature);

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
  account.eventId = eventItem.event.id;
  account.blockId = block.header.hash;

  await ctx.store.save(account);

  return account;
}

/**
 * Create and store Event on database.
 *
 * Returns the stored event id.
 * @param ctx
 * @param block
 * @param eventItem
 * @param eventType
 */
export async function saveEvent(
  ctx: Context,
  block: Block,
  eventItem: EventItem,
  eventType: EventType
): Promise<Event> {
  let signature: SubstrateExtrinsicSignature | undefined;
  let txHash: string | undefined;
  if ("extrinsic" in eventItem.event) {
    signature = eventItem.event.extrinsic?.signature;
    txHash = eventItem.event.extrinsic?.hash;
  }
  const accountId = getAccountFromSignature(signature);

  // Create event
  const newEvent = new Event({
    id: eventItem.event.id,
    accountId,
    eventType,
    blockNumber: BigInt(block.header.height),
    timestamp: new Date(block.header.timestamp),
    blockId: block.header.hash,
    txHash
  });

  // Store event
  await ctx.store.save(newEvent);

  return newEvent;
}

/**
 * Store Activity on the database.
 * @param ctx
 * @param block
 * @param event
 * @param accountId
 */
export async function saveActivity(ctx: Context, block: Block, event: Event, accountId: string): Promise<string> {
  const activity = new Activity({
    id: randomUUID(),
    event,
    accountId,
    timestamp: new Date(block.header.timestamp),
    blockId: block.header.hash
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
 * @param block
 * @param eventItem
 * @param eventType
 * @param accountId
 */
export async function saveAccountAndEvent(
  ctx: Context,
  block: Block,
  eventItem: EventItem,
  eventType: EventType,
  accountId?: string | string[]
): Promise<{ accounts: Account[]; event: Event }> {
  const accountIds: (string | undefined)[] = typeof accountId === "string" ? [accountId] : accountId || [];

  const savedEvent = await saveEvent(ctx, block, eventItem, eventType);

  const accounts: Account[] = [];

  for (let index = 0; index < accountIds.length; index += 1) {
    const id = accountIds[index];
    if (!id) {
      // no-op
      return Promise.reject(new Error("Missing account id"));
    }
    const account = await getOrCreateAccount(ctx, block, eventItem, id);
    if (account) {
      accounts.push(account);
      await saveActivity(ctx, block, savedEvent, id);
    }
  }

  return Promise.resolve({ accounts, event: savedEvent });
}

/**
 * Stores a new HistoricalLockedValue with current locked amount
 * for the specified source, and for the overall locked value
 * @param ctx
 * @param block
 * @param eventItem
 * @param amountsLocked
 * @param source
 * @param sourceEntityId
 */
export async function storeHistoricalLockedValue(
  ctx: Context,
  block: Block,
  eventItem: EventItem,
  amountsLocked: [string, bigint][], // [assetId, amountLocked]
  source: LockedSource,
  sourceEntityId: string
): Promise<void> {
  let event = await ctx.store.get(Event, { where: { id: eventItem.event.id } });

  if (!event) {
    event = await saveEvent(ctx, block, eventItem, EventType.SWAP);
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
      timestamp: new Date(block.header.timestamp),
      source,
      assetId,
      sourceEntityId,
      blockId: block.header.hash
    });

    await ctx.store.save(historicalLockedValue);
  }
}

/**
 * Get Pablo pool asset by asset id and pool id. If it doesn't exist, create it.
 * @param ctx
 * @param block
 * @param pool
 * @param assetId
 */
export async function getOrCreatePabloAsset(
  ctx: Context,
  block: Block,
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
      blockId: block.header.hash,
      weight: weight?.weight || 0
    });
  }
  return Promise.resolve(pabloAsset);
}

export async function getSpotPrice(
  ctx: Context | EntityManager,
  quoteAssetId: string,
  baseAssetId: string,
  poolId: string,
  timestamp?: number
): Promise<number> {
  if (quoteAssetId === baseAssetId) {
    return 1;
  }

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
          timestamp: LessThan(new Date(time)),
          success: true
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
          timestamp: LessThan(new Date(time)),
          success: true
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
          timestamp: LessThan(new Date(time)),
          success: true
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
          timestamp: LessThan(new Date(time)),
          success: true
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
      assetId: quoteAssetId,
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

/**
 * Searches for HistoricalAssetPrices and returns the asset price for a given timestamp.
 * If price does not exist, it creates it.
 * If a base asset and pool are provided, it calculates the spot price, and uses it to
 * calculate the actual asset price.
 * It does not store this calculated price, as the function does not know if it is
 * accurate for the provided timestamp (the processor might still be many blocks behind)
 * @param ctx
 * @param assetId
 * @param timestamp
 */
export async function getOrCreateHistoricalAssetPrice(
  ctx: Context | EntityManager,
  assetId: string,
  timestamp: number
): Promise<number> {
  const assetInfo = assetList.find(asset => asset.assetId === assetId);
  if (!assetInfo) {
    throw new Error(`Asset ${assetId} not found.`);
  }

  // When asset has a base asset, use it to calculate the price
  const baseAssetId = assetInfo.spotPriceBaseAsset?.assetId;
  const assetIdToUse = baseAssetId || assetId;
  const poolId = assetInfo.spotPriceBaseAsset?.poolId;

  const time = new Date(timestamp);
  // Use 00:00hs as date
  const date = new Date(time.getFullYear(), time.getMonth(), time.getDate());

  let price: number | undefined;

  // Check if the price already exists
  price = await findHistoricalAssetPrice(ctx, assetIdToUse, date);

  if (price === undefined) {
    // If price does not exist, query all prices for the given asset and store the
    // missing ones on the DB.
    await getAllHistoricalCoingeckoPrices(ctx, assetIdToUse);

    // Retry finding the recently stored price
    price = await findHistoricalAssetPrice(ctx, assetIdToUse, date);

    if (price === undefined) {
      throw new Error(`Cannot find asset price for ${assetIdToUse}`);
    }
  }

  if (baseAssetId && poolId) {
    const spotPrice = await getSpotPrice(ctx, assetId, baseAssetId, poolId, timestamp);
    return price / spotPrice;
  }

  return price;
}

/**
 * Looks for a historical price stored for a given assetId and timestamp.
 * If it does not exist, returns undefined.
 * @param ctx
 * @param assetId
 * @param timestamp
 */
export async function findHistoricalAssetPrice(
  ctx: Context | EntityManager,
  assetId: string,
  timestamp: Date
): Promise<number | undefined> {
  const isRepository = ctx instanceof EntityManager;

  const where = {
    assetId,
    timestamp
  };

  try {
    // Look for the price in the DB
    const assetPrice = isRepository
      ? await ctx.getRepository(HistoricalAssetPrice).findOne({ where })
      : await ctx.store.findOne(HistoricalAssetPrice, {
          where
        });
    return assetPrice?.price;
  } catch {
    return undefined;
  }
}

/**
 * Gets current prices from DB or Coingecko
 * @param ctx
 */
export async function getCurrentAssetPrices(ctx: Context | EntityManager): Promise<Record<string, number> | undefined> {
  const isRepository = ctx instanceof EntityManager;

  const now = new Date();
  // Round time to the nearest minute.
  const date = new Date(now.getFullYear(), now.getMonth(), now.getDate(), now.getHours(), now.getMinutes());

  // Current prices, that can be obtained from DB and/or Coingecko.
  const currentPrices: { [K in AssetId]?: number } = {};
  // Prices that need to be stored. Use Set to avoid duplications when multiple
  // assets rely on the same base asset.
  const updatePrices: Set<AssetId> = new Set();
  // New HistoricalAssetPrices to be stored.
  const newHistoricalPrices: Array<HistoricalAssetPrice> = [];

  // Check if all prices are in DB, and flag the missing ones for updating
  // This includes the ones that are required to calculate the spot price for
  // another asset.
  for (const { assetId, spotPriceBaseAsset } of assetList.values()) {
    // Use the asset ID of the base asset if it is provided, as this asset relies
    // on another price to calculate its own.
    const assetIdToUse = spotPriceBaseAsset?.assetId || assetId;
    // Search for price in DB.
    const price = await findHistoricalAssetPrice(ctx, assetIdToUse, date);

    if (price) {
      // If price is found, keep it for returning
      currentPrices[assetIdToUse] = price;
    } else {
      // If price is not found, flag it for updating
      updatePrices.add(assetIdToUse);
    }
  }

  // If there are assets for updating, query Coingecko
  if (updatePrices.size > 0) {
    // Create query url
    const queryIds = Array.from(updatePrices)
      .map(assetId => assetList.find(asset => asset.assetId === assetId)?.coingeckoId)
      .filter(Boolean)
      .join("%2C");
    const endpoint = `https://api.coingecko.com/api/v3/simple/price?ids=${queryIds}&vs_currencies=usd`;
    // Fetch from Coingecko
    const res = await fetch<CoingeckoPrices>(endpoint);

    for (const assetId of updatePrices) {
      const assetInfo = assetList.find(asset => asset.assetId === assetId);
      // Store prices from Coingecko in DB
      if (assetInfo?.coingeckoId && res[assetInfo.coingeckoId].usd) {
        // Keep price for returning
        currentPrices[assetId] = res[assetInfo.coingeckoId].usd;
        // Add HistoricalAssetPrice for storing in DB
        newHistoricalPrices.push(
          new HistoricalAssetPrice({
            id: randomUUID(),
            assetId,
            price: res[assetInfo.coingeckoId].usd,
            timestamp: date,
            currency: Currency.USD
          })
        );
      }
    }

    // Store new prices
    if (isRepository) {
      await ctx.getRepository(HistoricalAssetPrice).save(newHistoricalPrices);
    } else {
      await ctx.store.save(newHistoricalPrices);
    }
  }

  // Get prices that require to use another asset and the spot price
  for (const { assetId, spotPriceBaseAsset } of assetList.values()) {
    if (spotPriceBaseAsset && !currentPrices[assetId]) {
      const basePrice = currentPrices[spotPriceBaseAsset.assetId];
      if (basePrice) {
        const spotPrice = await getSpotPrice(
          ctx,
          assetId,
          spotPriceBaseAsset.assetId,
          spotPriceBaseAsset.poolId,
          date.getTime()
        );
        currentPrices[assetId] = basePrice / spotPrice;
      }
    }
  }

  return currentPrices;
}

export async function getNormalizedPoolTVL(ctx: Context | EntityManager, poolId: string): Promise<bigint> {
  const isRepository = ctx instanceof EntityManager;

  const poolOptions = {
    where: {
      id: poolId
    },
    relations: {
      poolAssets: true
    }
  };

  const pool = isRepository
    ? await ctx.getRepository(PabloPool).findOne(poolOptions)
    : await ctx.store.get(PabloPool, poolOptions);

  if (!pool) {
    throw new Error("Pool not found");
  }

  const { poolAssets, quoteAssetId } = pool;

  let normalizedTvl = 0n;

  for (const asset of poolAssets) {
    const assetPrice = BigNumber(await getSpotPrice(ctx, quoteAssetId, asset.assetId, poolId));
    const assetTVL = BigInt(BigNumber(asset.totalLiquidity.toString()).multipliedBy(assetPrice).toFixed(0));
    normalizedTvl += assetTVL;
  }

  return Promise.resolve(normalizedTvl);
}

/**
 * Get LP Token by id If it doesn't exist, create it.
 * @param ctx
 * @param block
 * @param poolId
 * @param lpTokenId
 */
export async function getOrCreatePabloLpToken(
  ctx: Context,
  block: Block,
  poolId: string,
  lpTokenId: string
): Promise<PabloLpToken> {
  let lpToken = await ctx.store.get(PabloLpToken, {
    where: {
      id: lpTokenId,
      poolId
    }
  });
  if (!lpToken) {
    lpToken = new PabloLpToken({
      id: lpTokenId,
      totalIssued: 0n,
      poolId,
      blockId: block.header.hash,
      timestamp: new Date(block.header.timestamp)
    });
    await ctx.store.save(lpToken);
  }
  return Promise.resolve(lpToken);
}

export async function getOrCreateFeeApr(
  ctx: Context | EntityManager,
  pool: PabloPool,
  swapFee = 0.003,
  timestamp = new Date(),
  block?: Block,
  event?: Event
): Promise<number> {
  const isRepository = ctx instanceof EntityManager;

  const { quoteAssetId } = pool;

  const options = {
    where: {
      pool: {
        id: pool.id
      },
      timestamp: MoreThan(new Date(timestamp.getTime() - DAY_IN_MS)),
      success: true
    }
  };

  const latestSwaps = isRepository
    ? await ctx.getRepository(PabloSwap).find(options)
    : await ctx.store.find(PabloSwap, options);

  const dailyVolume = latestSwaps.reduce((acc, swap) => {
    if (swap.baseAssetId === quoteAssetId) {
      return acc + swap.baseAssetAmount;
    }
    if (swap.quoteAssetId === quoteAssetId) {
      return acc + swap.quoteAssetAmount;
    }
    return acc;
  }, 0n);

  const normalizedTvl = await getNormalizedPoolTVL(ctx, pool.id);

  if (normalizedTvl === 0n) {
    console.error(`TVL for pool ${pool.id} is 0. Ignoring.`);
    return 0;
  }

  const tradingFee = BigNumber(dailyVolume.toString())
    .multipliedBy(BigNumber(swapFee))
    .multipliedBy(365)
    .dividedBy(normalizedTvl.toString())
    .toNumber();

  if (!isRepository && event && block) {
    const historicalFeeApr = new HistoricalPabloFeeApr({
      id: randomUUID(),
      event,
      pool,
      timestamp: new Date(block.header.timestamp),
      blockId: block.header.hash,
      tradingFee
    });

    await ctx.store.save(historicalFeeApr);
  }

  return tradingFee;
}

export async function getOrCreateStakingApr(
  ctx: Context | EntityManager,
  block: Block,
  pool: PabloPool,
  timestamp = new Date(),
  event?: Event
): Promise<number> {
  const isRepository = ctx instanceof EntityManager;

  const { lpToken } = pool;

  const options = {
    where: {
      assetId: lpToken.id,
      timestamp: LessThan(new Date(timestamp.getTime()))
    },
    sort: {
      timestamp: "DESC"
    }
  };
  const rewardsPool = isRepository
    ? await ctx.getRepository(StakingRewardsPool).findOne(options)
    : await ctx.store.get(StakingRewardsPool, options);

  if (!rewardsPool) {
    throw new Error("No rewards pool found for this pool's LP token");
  }

  const normalizedTvl = await getNormalizedPoolTVL(ctx, pool.id);

  const stakingApr = BigNumber(rewardsPool.rewardRateAmount.toString())
    .multipliedBy(365 * 24 * 60 * 60)
    .dividedBy(normalizedTvl.toString())
    .toNumber();

  if (!isRepository && event) {
    const historicalStakingApr = new HistoricalStakingApr({
      id: randomUUID(),
      event,
      assetId: lpToken.id,
      stakingApr,
      timestamp: new Date(block.header.timestamp),
      blockId: block.header.hash
    });

    await ctx.store.save(historicalStakingApr);
  }

  return stakingApr;
}

export async function getHistoricalCoingeckoPrice(
  ctx: Context | EntityManager,
  assetInfo: AssetInfo,
  date?: Date
): Promise<number> {
  let time = new Date();
  if (date && isInstance(date, Date)) {
    time = date;
  } else if (date) {
    time = new Date(date);
  }

  const month = time.getMonth() + 1;
  const day = time.getDate();
  const year = time.getFullYear();

  const queryDate = `${day < 10 ? "0" : ""}${day}-${month < 10 ? "0" : ""}${month}-${year}`;

  let coinId: string;
  let spotPrice: number | undefined;
  if (assetInfo.coingeckoId) {
    coinId = assetInfo.coingeckoId;
  } else if (assetInfo.spotPriceBaseAsset) {
    coinId = assetInfo.spotPriceBaseAsset.coingeckoId;
    spotPrice = await getSpotPrice(
      ctx,
      assetInfo.assetId,
      assetInfo.spotPriceBaseAsset.assetId,
      assetInfo.spotPriceBaseAsset.poolId,
      time.getTime()
    );
  } else {
    throw new Error("No Coingecko ID found");
  }

  const endpoint = `https://api.coingecko.com/api/v3/coins/${coinId}/history?date=${queryDate}&localization=en`;
  try {
    const res = await fetchRetry<{ market_data: { current_price: { usd: number } } }>(endpoint);
    if (res) {
      let price = res.market_data?.current_price?.usd;
      if (spotPrice) {
        price /= spotPrice;
      }
      return price;
    }
  } catch {
    console.log("error fetching", endpoint);
  }

  throw new Error("Failed to fetch historical price");
}

export async function getAllHistoricalCoingeckoPrices(
  ctx: Context | EntityManager,
  assetId: string
): Promise<Array<[Date, number]>> {
  const assetInfo = assetList.find(asset => asset.assetId === assetId);
  if (!assetInfo) {
    throw new Error(`Asset ${assetId} not found in asset list.`);
  }

  const coinId = assetInfo.coingeckoId || assetInfo.spotPriceBaseAsset?.coingeckoId;
  if (!coinId) {
    throw new Error("No Coingecko ID found");
  }

  const endpoint = `https://api.coingecko.com/api/v3/coins/${coinId}/market_chart?vs_currency=usd&days=max&interval=daily`;
  try {
    const res = await fetchRetry<{
      prices: [number, number][];
      market_caps: [number, number][];
      total_volumes: [number, number][];
    }>(endpoint);
    // If prices are obtained, store the missing ones on the DB
    if (res) {
      // Keep track of used dates to avoid duplicates
      const duplicates: Set<string> = new Set();
      // HistoricalAssetPrices that need to be stored
      const missingPrices: Array<HistoricalAssetPrice> = [];

      // Map prices to get proper Date values
      const prices: Array<[Date, number]> = res.prices.map(([timestamp, price]) => {
        const day = new Date(timestamp);
        const date = new Date(day.getFullYear(), day.getMonth(), day.getDate());
        return [date, price];
      });

      for (const price of prices) {
        try {
          // Look for the price in the DB
          const date = price[0];
          const priceValue = price[1];

          if (!duplicates.has(date.getTime().toString())) {
            duplicates.add(date.getTime().toString());
            const assetPrice = await findHistoricalAssetPrice(ctx, assetId, date);

            if (assetPrice === undefined) {
              missingPrices.push(
                new HistoricalAssetPrice({
                  id: randomUUID(),
                  assetId,
                  price: priceValue,
                  timestamp: date,
                  currency: Currency.USD
                })
              );
            }
          }
        } catch (err) {
          console.log(err);
        }
      }

      const isRepository = ctx instanceof EntityManager;
      if (isRepository) {
        await ctx.getRepository(HistoricalAssetPrice).save(missingPrices);
      } else {
        await ctx.store.save(missingPrices);
      }

      return prices;
    }
  } catch {
    console.log("error fetching", endpoint);
  }

  throw new Error("Failed to fetch historical price");
}

export async function getOrCreateCallError(ctx: Context, err: any): Promise<CallError | null> {
  try {
    const value: { error: string; index: number } = err?.value;
    const errorCode = parseInt(value.error.slice(0, 4), 16);
    const res = api.findError(new Uint8Array([value.index, errorCode]));

    let callError = await ctx.store.findOne<CallError>(CallError, {
      where: {
        section: res.section,
        name: res.name
      }
    });

    if (!callError) {
      callError = new CallError({
        id: `${res.section}-${res.name}`,
        section: res.section,
        name: res.name,
        description: res.docs?.[0] || undefined
      });

      await ctx.store.save(callError);
    }

    return callError;
  } catch {
    return null;
  }
}
