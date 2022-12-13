import {
  PabloPool,
  PabloPoolAsset,
  EventType,
  LockedSource,
} from "subsquid/model";
import { Store } from "@subsquid/typeorm-store";
import { EventHandlerContext } from "@subsquid/substrate-processor";
import {
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloSwappedEvent,
} from "subsquid/types/events";
import { Fee } from "subsquid/types/v10003";
import { encodeAccount } from "subsquid/utils";
import {
  getLatestPoolByPoolId,
  saveAccountAndEvent,
  saveActivity,
  saveEvent,
  storeCurrentLockedValue,
  storeHistoricalLockedValue,
  storeHistoricalVolume,
} from "../dbHelper";

/**
 * Creates asset fpr Pablo pool
 * @param pool
 * @param assetId
 * @param weight
 * @param ctx
 * @param timestamp
 */
function createPabloAsset(
  pool: PabloPool,
  assetId: string,
  weight: bigint,
  ctx: EventHandlerContext<Store, { event: true }>,
  timestamp: Date
) {
  return new PabloPoolAsset({
    id: assetId,
    pool,
    blockNumber: BigInt(ctx.block.height),
    totalLiquidity: BigInt(0),
    totalVolume: BigInt(0),
    weight,
    timestamp,
  });
}

interface PoolCreatedEvent {
  owner: Uint8Array;
  poolId: bigint;
  assetWeights: [bigint, number][];
}

function getPoolCreatedEvent(event: PabloPoolCreatedEvent): PoolCreatedEvent {
  const { owner, poolId, assetWeights } = event.asV10003;
  return {
    owner,
    poolId,
    assetWeights,
  };
}

interface LiquidityAddedEvent {
  who: Uint8Array;
  poolId: bigint;
  assetAmounts: [bigint, bigint][];
  mintedLp: bigint;
}

function getLiquidityAddedEvent(
  event: PabloLiquidityAddedEvent
): LiquidityAddedEvent {
  const { who, poolId, assetAmounts, mintedLp } = event.asV10003;
  return {
    who,
    poolId,
    assetAmounts,
    mintedLp,
  };
}

interface LiquidityRemovedEvent {
  who: Uint8Array;
  poolId: bigint;
  assetAmounts: [bigint, bigint][];
}

function getLiquidityRemovedEvent(
  event: PabloLiquidityRemovedEvent
): LiquidityRemovedEvent {
  const { who, poolId, assetAmounts } = event.asV10003;
  return {
    who,
    poolId,
    assetAmounts,
  };
}

interface SwappedEvent {
  who: Uint8Array;
  poolId: bigint;
  baseAsset: bigint;
  baseAmount: bigint;
  quoteAsset: bigint;
  quoteAmount: bigint;
  fee: Fee;
}

function getSwappedEvent(event: PabloSwappedEvent): SwappedEvent {
  const { who, poolId, baseAsset, baseAmount, quoteAsset, quoteAmount, fee } =
    event.asV10003;
  return {
    who,
    poolId,
    baseAsset,
    baseAmount,
    quoteAsset,
    quoteAmount,
    fee,
  };
}

export async function processPoolCreatedEvent(
  ctx: EventHandlerContext<Store, { event: true }>
): Promise<void> {
  console.debug("processing PoolCreatedEvent", ctx.event.id);
  const pabloPoolCreatedEvent = new PabloPoolCreatedEvent(ctx);
  const poolCreatedEvent = getPoolCreatedEvent(pabloPoolCreatedEvent);
  const owner = encodeAccount(poolCreatedEvent.owner);
  const { poolId, assetWeights } = poolCreatedEvent;

  // Create and save event
  await saveEvent(ctx, EventType.CREATE_POOL);

  // Create pool
  const pool = new PabloPool({
    id: poolId.toString(),
    eventId: ctx.event.id,
    owner,
    lpIssued: BigInt(0),
    transactionCount: 0,
    totalLiquidity: BigInt(0),
    totalVolume: BigInt(0),
    totalFees: BigInt(0),
    blockNumber: BigInt(ctx.block.height),
    timestamp: new Date(ctx.block.timestamp),
  });

  // Create assets
  const poolAssets = assetWeights.map(([assetId, weight]) =>
    createPabloAsset(
      pool,
      assetId.toString(),
      BigInt(weight || 0),
      ctx,
      new Date(ctx.block.timestamp)
    )
  );

  // Store assets
  for (const asset of poolAssets) {
    await ctx.store.save(asset);
  }

  // TODO: Create and store Pablo Transaction

  // TODO: add activity? Maybe not because of Sudo?

  // Store pool
  await ctx.store.save(pool);
}

export async function processLiquidityAddedEvent(
  ctx: EventHandlerContext<Store, { event: true }>
): Promise<void> {
  console.debug("processing LiquidityAddedEvent", ctx.event.id);
  const pabloLiquidityAddedEvent = new PabloLiquidityAddedEvent(ctx);
  const liquidityAddedEvent = getLiquidityAddedEvent(pabloLiquidityAddedEvent);
  const who = encodeAccount(liquidityAddedEvent.who);
  const { poolId, assetAmounts, mintedLp } = liquidityAddedEvent;

  // Get the latest pool
  const pool = await getLatestPoolByPoolId(ctx.store, poolId);

  if (!pool) {
    console.error("Pool not found");
    return;
  }

  // Create and save event
  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.ADD_LIQUIDITY,
    who
  );

  // Create and save activity
  await saveActivity(ctx, event, who);

  const addedLiquidity = assetAmounts.reduce(
    (acc, [, amount]) => acc + amount,
    BigInt(0)
  );

  // Update pool
  pool.eventId = ctx.event.id;
  pool.timestamp = new Date(ctx.block.timestamp);
  pool.blockNumber = BigInt(ctx.block.height);
  pool.transactionCount += 1;
  pool.lpIssued += mintedLp;
  pool.totalLiquidity += addedLiquidity;

  // Update assets
  for (const [assetId, amount] of assetAmounts) {
    const asset = pool.poolAssets.find(({ id }) => id === assetId.toString());
    if (asset) {
      asset.totalLiquidity += amount;
      asset.timestamp = new Date(ctx.block.timestamp);
      asset.blockNumber = BigInt(ctx.block.height);
      // Save asset
      await ctx.store.save(asset);
    }
  }

  // TODO: Create and store Pablo Transaction

  await ctx.store.save(pool);

  const amountsLocked = assetAmounts.reduce<Record<string, bigint>>(
    (acc, [assetId, amount]) => ({
      ...acc,
      [assetId.toString()]: amount,
    }),
    {}
  );

  // TODO: refactor following functions to expect array of [assetId, amount]
  await storeHistoricalLockedValue(ctx, amountsLocked, LockedSource.Pablo);
  await storeCurrentLockedValue(ctx, amountsLocked, LockedSource.Pablo);
}

export async function processLiquidityRemovedEvent(
  ctx: EventHandlerContext<Store, { event: true }>
): Promise<void> {
  console.debug("processing LiquidityRemovedEvent", ctx.event.id);
  const pabloLiquidityRemovedEvent = new PabloLiquidityRemovedEvent(ctx);
  const liquidityRemovedEvent = getLiquidityRemovedEvent(
    pabloLiquidityRemovedEvent
  );
  const who = encodeAccount(liquidityRemovedEvent.who);
  const { poolId, assetAmounts } = liquidityRemovedEvent;

  // Get the latest pool
  const pool = await getLatestPoolByPoolId(ctx.store, poolId);

  if (!pool) {
    console.error("Pool not found");
    return;
  }

  // Create and save account and event
  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.REMOVE_LIQUIDITY,
    who
  );

  // Create and save activity
  await saveActivity(ctx, event, who);

  const removedLiquidity = assetAmounts.reduce(
    (acc, [, amount]) => acc + amount,
    BigInt(0)
  );

  // Update pool
  pool.eventId = ctx.event.id;
  pool.timestamp = new Date(ctx.block.timestamp);
  pool.blockNumber = BigInt(ctx.block.height);
  pool.transactionCount += 1;
  pool.totalLiquidity -= removedLiquidity;

  // Update assets
  for (const [assetId, amount] of assetAmounts) {
    const asset = pool.poolAssets.find(({ id }) => id === assetId.toString());
    if (asset) {
      asset.totalLiquidity -= amount;
      asset.timestamp = new Date(ctx.block.timestamp);
      asset.blockNumber = BigInt(ctx.block.height);
      // Save asset
      await ctx.store.save(asset);
    }
  }

  // TODO: Create and store Pablo Transaction

  await ctx.store.save(pool);

  const amountsLocked = assetAmounts.reduce<Record<string, bigint>>(
    (acc, [assetId, amount]) => ({
      ...acc,
      [assetId.toString()]: -amount,
    }),
    {}
  );

  // TODO: refactor following functions to expect array of [assetId, amount]
  await storeHistoricalLockedValue(ctx, amountsLocked, LockedSource.Pablo);
  await storeCurrentLockedValue(ctx, amountsLocked, LockedSource.Pablo);
}

export async function processSwappedEvent(
  ctx: EventHandlerContext<Store, { event: true }>
): Promise<void> {
  console.debug("processing SwappedEvent", ctx.event.id);
  const pabloSwappedEvent = new PabloSwappedEvent(ctx);
  const swappedEvent = getSwappedEvent(pabloSwappedEvent);
  const who = encodeAccount(swappedEvent.who);
  const {
    poolId,
    fee,
    baseAsset: baseAssetId,
    baseAmount,
    quoteAsset: quoteAssetId,
    quoteAmount,
  } = swappedEvent;
  const feesLeavingPool = fee.fee - fee.lpFee;
  const spotPrice = quoteAmount / baseAmount; // TODO: check

  // Get the latest pool
  const pool = await getLatestPoolByPoolId(ctx.store, poolId);

  if (!pool) {
    console.error("Pool not found");
    return;
  }

  // Create and save account and event
  const { event } = await saveAccountAndEvent(ctx, EventType.SWAP, who);

  // Create and save activity
  await saveActivity(ctx, event, who);

  // Update pool
  pool.eventId = ctx.event.id;
  pool.timestamp = new Date(ctx.block.timestamp);
  pool.blockNumber = BigInt(ctx.block.height);
  pool.transactionCount += 1;
  pool.totalVolume += quoteAmount; // TODO: check if this is correct in the case of reversed, if exists
  pool.totalLiquidity -= calculateFeeInQuoteAsset(
    spotPrice,
    quoteAssetId,
    fee.assetId,
    feesLeavingPool
  );

  const quoteAssetFee = calculateFeeInQuoteAsset(
    spotPrice,
    quoteAssetId,
    fee.assetId,
    fee.fee
  );
  pool.totalFees += quoteAssetFee;

  const baseAsset = pool.poolAssets.find(
    ({ id }) => id === baseAssetId.toString()
  );

  if (baseAsset) {
    baseAsset.timestamp = new Date(ctx.block.timestamp);
    baseAsset.blockNumber = BigInt(ctx.block.height);
    baseAsset.totalVolume += baseAmount; // TODO: check
    baseAsset.totalLiquidity -= baseAmount; // TODO: check
    baseAsset.totalLiquidity -= feesLeavingPool; // TODO: check

    await ctx.store.save(baseAsset);
  }

  const quoteAsset = pool.poolAssets.find(
    ({ id }) => id === quoteAssetId.toString()
  );

  if (quoteAsset) {
    quoteAsset.timestamp = new Date(ctx.block.timestamp);
    quoteAsset.blockNumber = BigInt(ctx.block.height);
    quoteAsset.totalVolume += quoteAmount; // TODO: check
    quoteAsset.totalLiquidity += quoteAmount; // TODO: check

    await ctx.store.save(quoteAsset);
  }

  // TODO: Create and store Pablo Transaction

  // TODO: reverse swap??

  await ctx.store.save(pool);

  await storeHistoricalVolume(ctx, quoteAssetId.toString(), quoteAmount);
}

function calculateFeeInQuoteAsset(
  spotPrice: bigint,
  quoteAsset: bigint,
  feeAsset: bigint,
  fee: bigint
): bigint {
  // calculate the quote amount based on the exchange rate if the fees are in the base asset
  return feeAsset === quoteAsset ? fee : spotPrice * fee;
}
