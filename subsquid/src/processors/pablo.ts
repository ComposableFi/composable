import { Store } from "@subsquid/typeorm-store";
import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import BigNumber from "bignumber.js";
import {
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloSwappedEvent
} from "../types/events";
import {
  EventType,
  HistoricalVolume,
  LockedSource,
  PabloAssetWeight,
  PabloFee,
  PabloPool,
  PabloPoolType,
  PabloSwap,
  PabloTransaction
} from "../model";
import { Fee } from "../types/v10002";
import { divideBigInts, encodeAccount } from "../utils";
import {
  getLatestPoolByPoolId,
  getOrCreatePabloAsset,
  saveAccountAndEvent,
  saveActivity,
  saveEvent,
  storeHistoricalLockedValue
} from "../dbHelper";

interface PoolCreatedEvent {
  owner: Uint8Array;
  poolId: bigint;
  assetWeights: [bigint, number][];
}

function getPoolCreatedEvent(event: PabloPoolCreatedEvent): PoolCreatedEvent {
  if (event.isV10002) {
    const { owner, poolId, assets } = event.asV10002;
    return {
      owner,
      poolId,
      assetWeights: [
        [assets.base, 0],
        [assets.quote, 0]
      ]
    };
  }
  const { owner, poolId, assetWeights } = event.asV10004;
  return {
    owner,
    poolId,
    assetWeights
  };
}

interface LiquidityAddedEvent {
  who: Uint8Array;
  poolId: bigint;
  assetAmounts: [bigint, bigint][];
  mintedLp: bigint;
}

function getLiquidityAddedEvent(event: PabloLiquidityAddedEvent): LiquidityAddedEvent {
  if (event.isV10002) {
    const { who, poolId, mintedLp } = event.asV10002;
    return {
      who,
      poolId,
      assetAmounts: [
        // This version should not be reached, but needs to be handled
        [0n, 0n],
        [0n, 0n]
      ],
      mintedLp
    };
  }
  const { who, poolId, assetAmounts, mintedLp } = event.asV10004;
  return {
    who,
    poolId,
    assetAmounts,
    mintedLp
  };
}

interface LiquidityRemovedEvent {
  who: Uint8Array;
  poolId: bigint;
  assetAmounts: [bigint, bigint][];
}

function getLiquidityRemovedEvent(event: PabloLiquidityRemovedEvent): LiquidityRemovedEvent {
  if (event.isV10002) {
    const { who, poolId } = event.asV10002;
    return {
      who,
      poolId,
      assetAmounts: [
        // This version should not be reached, but needs to be handled
        [0n, 0n],
        [0n, 0n]
      ]
    };
  }
  const { who, poolId, assetAmounts } = event.asV10004;
  return {
    who,
    poolId,
    assetAmounts
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
  const { who, poolId, baseAsset, baseAmount, quoteAsset, quoteAmount, fee } = event.asV10002;
  return {
    who,
    poolId,
    baseAsset,
    baseAmount,
    quoteAsset,
    quoteAmount,
    fee
  };
}

export async function processPoolCreatedEvent(ctx: EventHandlerContext<Store, { event: true }>): Promise<void> {
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
    // Note: when we add more pool types, we can get this from the chain -> api.query.pablo.pool(poolId)
    poolType: PabloPoolType.DualAssetConstantProduct,
    lpIssued: BigInt(0),
    transactionCount: 0,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash,
    quoteAssetId: assetWeights[0][0].toString()
  });

  // Store pool
  await ctx.store.save(pool);

  // Store weights
  for (const [assetId, weight] of assetWeights) {
    const pabloAssetWeight = new PabloAssetWeight({
      id: randomUUID(),
      pool,
      assetId: assetId.toString(),
      weight: weight / 1_000_000,
      blockId: ctx.block.hash
    });

    await ctx.store.save(pabloAssetWeight);
  }
}

export async function processLiquidityAddedEvent(ctx: EventHandlerContext<Store, { event: true }>): Promise<void> {
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
  const { event } = await saveAccountAndEvent(ctx, EventType.ADD_LIQUIDITY, who);

  // Create and save activity
  await saveActivity(ctx, event, who);

  // Update pool
  pool.eventId = ctx.event.id;
  pool.timestamp = new Date(ctx.block.timestamp);
  pool.transactionCount += 1;
  pool.lpIssued += mintedLp;
  pool.blockId = ctx.block.id;

  // Update or create assets
  for (const [assetId, amount] of assetAmounts) {
    const asset = await getOrCreatePabloAsset(ctx, pool, assetId.toString());

    asset.totalLiquidity += amount;
    asset.blockId = ctx.block.id;

    await ctx.store.save(asset);
  }

  const pabloTransaction = new PabloTransaction({
    id: ctx.event.id,
    pool,
    account: who,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash
  });

  await ctx.store.save(pabloTransaction);

  await ctx.store.save(pool);

  for (const [assetId, amount] of assetAmounts) {
    await storeHistoricalLockedValue(ctx, [[assetId.toString(), amount]], LockedSource.Pablo, pool.id);
  }
}

export async function processLiquidityRemovedEvent(ctx: EventHandlerContext<Store, { event: true }>): Promise<void> {
  console.debug("processing LiquidityRemovedEvent", ctx.event.id);
  const pabloLiquidityRemovedEvent = new PabloLiquidityRemovedEvent(ctx);
  const liquidityRemovedEvent = getLiquidityRemovedEvent(pabloLiquidityRemovedEvent);
  const who = encodeAccount(liquidityRemovedEvent.who);
  const { poolId, assetAmounts } = liquidityRemovedEvent;

  // Get the latest pool
  const pool = await getLatestPoolByPoolId(ctx.store, poolId);

  if (!pool) {
    console.error("Pool not found");
    return;
  }

  // Create and save account and event
  const { event } = await saveAccountAndEvent(ctx, EventType.REMOVE_LIQUIDITY, who);

  // Create and save activity
  await saveActivity(ctx, event, who);

  // Update pool
  pool.eventId = ctx.event.id;
  pool.timestamp = new Date(ctx.block.timestamp);
  pool.transactionCount += 1;
  pool.blockId = ctx.block.id;

  // Update or create assets
  for (const [assetId, amount] of assetAmounts) {
    const asset = await getOrCreatePabloAsset(ctx, pool, assetId.toString());

    asset.totalLiquidity -= amount;
    asset.blockId = ctx.block.id;

    await ctx.store.save(asset);
  }

  const pabloTransaction = new PabloTransaction({
    id: ctx.event.id,
    pool,
    account: who,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash
  });

  await ctx.store.save(pabloTransaction);

  await ctx.store.save(pool);

  for (const [assetId, amount] of assetAmounts) {
    await storeHistoricalLockedValue(ctx, [[assetId.toString(), -amount]], LockedSource.Pablo, pool.id);
  }
}

export async function processSwappedEvent(ctx: EventHandlerContext<Store, { event: true }>): Promise<void> {
  console.debug("processing SwappedEvent", ctx.event.id);
  const pabloSwappedEvent = new PabloSwappedEvent(ctx);
  const swappedEvent = getSwappedEvent(pabloSwappedEvent);
  const who = encodeAccount(swappedEvent.who);
  const { poolId, fee, baseAsset: baseAssetId, baseAmount, quoteAsset: quoteAssetId, quoteAmount } = swappedEvent;

  // Get the latest pool
  const pool = await getLatestPoolByPoolId(ctx.store, poolId);

  if (!pool) {
    console.error("Pool not found");
    return;
  }

  const { poolType } = pool;

  if (poolType !== PabloPoolType.DualAssetConstantProduct) {
    throw new Error("Only DualAssetConstantProduct pools are currently supported.");
  }

  // Create and save account and event
  const { event } = await saveAccountAndEvent(ctx, EventType.SWAP, who);

  // Create and save activity
  await saveActivity(ctx, event, who);

  // Update pool
  pool.eventId = ctx.event.id;
  pool.timestamp = new Date(ctx.block.timestamp);
  pool.transactionCount += 1;
  pool.blockId = ctx.block.id;

  const baseAsset = await getOrCreatePabloAsset(ctx, pool, baseAssetId.toString());

  const quoteAsset = await getOrCreatePabloAsset(ctx, pool, quoteAssetId.toString());

  baseAsset.totalVolume += baseAmount;
  baseAsset.totalLiquidity -= baseAmount;
  baseAsset.blockId = ctx.block.id;

  await ctx.store.save(baseAsset);

  quoteAsset.totalVolume += quoteAmount;
  quoteAsset.totalLiquidity += quoteAmount;
  quoteAsset.blockId = ctx.block.id;

  await ctx.store.save(quoteAsset);

  const pabloTransaction = new PabloTransaction({
    id: ctx.event.id,
    pool,
    account: who,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash
  });

  await ctx.store.save(pabloTransaction);

  // Get weights
  const baseAssetWeight = pool.poolAssetWeights.find(({ assetId }) => assetId === baseAssetId.toString());
  const quoteAssetWeight = pool.poolAssetWeights.find(({ assetId }) => assetId === quoteAssetId.toString());

  if (!baseAssetWeight || !quoteAssetWeight) {
    console.error("Asset weights not found");
    return;
  }

  const weightRatio = baseAssetWeight.weight / quoteAssetWeight.weight;

  const normalizedQuoteAmount = (quoteAssetId === 130n ? 1_000_000n : 1n) * quoteAmount;
  const normalizedBaseAmount = (baseAssetId === 130n ? 1_000_000n : 1n) * baseAmount;

  const spotPrice = divideBigInts(normalizedQuoteAmount, normalizedBaseAmount) * weightRatio;

  const feeSpotPrice = BigNumber(fee.assetId.toString() === pool.quoteAssetId ? 1 : spotPrice);

  const pabloFee = new PabloFee({
    id: ctx.event.id,
    event,
    pool,
    assetId: pool.quoteAssetId,
    account: who,
    fee: BigInt(BigNumber(fee.fee.toString()).div(feeSpotPrice).toFixed(0)),
    lpFee: BigInt(BigNumber(fee.lpFee.toString()).div(feeSpotPrice).toFixed(0)),
    ownerFee: BigInt(BigNumber(fee.ownerFee.toString()).div(feeSpotPrice).toFixed(0)),
    protocolFee: BigInt(BigNumber(fee.protocolFee.toString()).div(feeSpotPrice).toFixed(0)),
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash
  });

  await ctx.store.save(pabloFee);

  const pabloSwap = new PabloSwap({
    id: randomUUID(),
    event,
    pool,
    baseAssetId: baseAssetId.toString(),
    baseAssetAmount: baseAmount,
    quoteAssetId: quoteAssetId.toString(),
    quoteAssetAmount: quoteAmount,
    spotPrice: spotPrice.toString(),
    fee: pabloFee,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash
  });

  await ctx.store.save(pabloSwap);

  await ctx.store.save(pool);

  const latestBaseAssetVolume =
    (
      await ctx.store.findOne(HistoricalVolume, {
        where: {
          assetId: baseAssetId.toString(),
          pool: {
            id: pool.id
          },
          source: LockedSource.Pablo
        }
      })
    )?.accumulatedAmount || 0n;

  const latestQuoteAssetVolume =
    (
      await ctx.store.findOne(HistoricalVolume, {
        where: {
          assetId: quoteAssetId.toString(),
          pool: {
            id: pool.id
          },
          source: LockedSource.Pablo
        }
      })
    )?.accumulatedAmount || 0n;

  const historicalVolumeBaseAsset = new HistoricalVolume({
    id: randomUUID(),
    event,
    amount: baseAmount,
    accumulatedAmount: latestBaseAssetVolume + baseAmount,
    assetId: baseAssetId.toString(),
    pool,
    timestamp: new Date(ctx.block.timestamp),
    source: LockedSource.Pablo,
    blockId: ctx.block.hash
  });

  const historicalVolumeQuoteAsset = new HistoricalVolume({
    id: randomUUID(),
    event,
    amount: quoteAmount,
    accumulatedAmount: latestQuoteAssetVolume + quoteAmount,
    assetId: quoteAssetId.toString(),
    pool,
    timestamp: new Date(ctx.block.timestamp),
    source: LockedSource.Pablo,
    blockId: ctx.block.hash
  });

  await ctx.store.save(historicalVolumeBaseAsset);
  await ctx.store.save(historicalVolumeQuoteAsset);
}
