import { Store } from "@subsquid/typeorm-store";
import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloSwappedEvent
} from "../types/events";
import {
  EventType,
  HistoricalLockedValue,
  HistoricalVolume,
  LockedSource,
  PabloAmount,
  PabloAssetWeight,
  PabloFee,
  PabloLiquidityAdded,
  PabloLiquidityRemoved,
  PabloPool,
  PabloPoolType,
  PabloSwap,
  PabloTransaction,
  PabloTx
} from "../model";
import { Fee } from "../types/v10005";
import { divideBigInts, encodeAccount } from "../utils";
import {
  getLatestPoolByPoolId,
  getOrCreateHistoricalAssetPrice,
  getOrCreateFeeApr,
  getOrCreatePabloAsset,
  getOrCreatePabloLpToken,
  saveAccountAndEvent,
  saveActivity,
  saveEvent
} from "../dbHelper";
import { chain } from "../config";

interface PoolCreatedEvent {
  owner: Uint8Array;
  poolId: bigint;
  assetWeights: [bigint, number][];
  lpTokenId: string;
}

async function getPoolCreatedEvent(event: PabloPoolCreatedEvent): Promise<PoolCreatedEvent> {
  if (event.isV10005) {
    const { owner, poolId, assetWeights } = event.asV10005;
    // TODO: get lpTokenId from the event
    // This is a temporary solution, and will be replaced by event data when runtime is upgraded
    const lpTokenId = (105 + Number(poolId)).toString();

    return Promise.resolve({
      owner,
      poolId,
      assetWeights,
      lpTokenId
    });
  }
  const { owner, poolId, assetWeights, lpTokenId } = event.asV10009;
  return Promise.resolve({
    owner,
    poolId,
    assetWeights,
    lpTokenId: lpTokenId.toString()
  });
}

interface LiquidityAddedEvent {
  who: Uint8Array;
  poolId: bigint;
  assetAmounts: [bigint, bigint][];
  mintedLp: bigint;
}

function getLiquidityAddedEvent(event: PabloLiquidityAddedEvent): LiquidityAddedEvent {
  const { who, poolId, assetAmounts, mintedLp } = event.asV10005;
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
  const { who, poolId, assetAmounts } = event.asV10005;
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
  const { who, poolId, baseAsset, baseAmount, quoteAsset, quoteAmount, fee } = event.asV10005;
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
  const poolCreatedEvent = await getPoolCreatedEvent(pabloPoolCreatedEvent);
  const owner = encodeAccount(poolCreatedEvent.owner);
  const { poolId, assetWeights, lpTokenId } = poolCreatedEvent;

  // Create and save event
  await saveEvent(ctx, EventType.CREATE_POOL);

  const lpToken = await getOrCreatePabloLpToken(ctx, poolId.toString(), lpTokenId);

  // Create pool
  const pool = new PabloPool({
    id: poolId.toString(),
    eventId: ctx.event.id,
    owner,
    // Note: when we add more pool types, we can get this from the chain -> api.query.pablo.pool(poolId)
    poolType: PabloPoolType.DualAssetConstantProduct,
    lpToken,
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

  const pool = await ctx.store.get(PabloPool, {
    where: {
      id: poolId.toString()
    },
    relations: {
      poolAssets: true,
      poolAssetWeights: true,
      lpToken: true
    }
  });

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
  pool.blockId = ctx.block.hash;

  await ctx.store.save(pool);
  const { lpToken } = pool;

  lpToken.totalIssued += mintedLp;

  await ctx.store.save(lpToken);

  // Update or create assets
  for (const [assetId, amount] of assetAmounts) {
    const asset = await getOrCreatePabloAsset(ctx, pool, assetId.toString());

    asset.totalLiquidity += amount;
    asset.blockId = ctx.block.hash;

    await ctx.store.save(asset);

    const historicalLockedValue = new HistoricalLockedValue({
      id: randomUUID(),
      event,
      amount,
      accumulatedAmount: asset.totalLiquidity,
      timestamp: new Date(ctx.block.timestamp),
      source: LockedSource.Pablo,
      assetId: assetId.toString(),
      sourceEntityId: pool.id,
      blockId: ctx.block.hash
    });

    await ctx.store.save(historicalLockedValue);

    await getOrCreateHistoricalAssetPrice(ctx, assetId.toString(), ctx.block.timestamp);
  }

  const amounts: Array<PabloAmount> = [];

  for (const [assetId, amount] of assetAmounts) {
    const price = await getOrCreateHistoricalAssetPrice(ctx, assetId.toString(), ctx.block.timestamp);
    if (price) {
      const pabloAmount = new PabloAmount({ assetId: assetId.toString(), amount, price });
      amounts.push(pabloAmount);
    }
  }

  const pabloLiquidityAdded = new PabloLiquidityAdded({
    id: ctx.event.id,
    event,
    pool,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash,
    amounts
  });

  await ctx.store.save(pabloLiquidityAdded);

  const pabloTransaction = new PabloTransaction({
    id: ctx.event.id,
    pool,
    account: who,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash,
    event,
    liquidityAdded: pabloLiquidityAdded,
    txType: PabloTx.ADD_LIQUIDITY
  });

  await ctx.store.save(pabloTransaction);

  // Calculate and update APR
  await getOrCreateFeeApr(ctx, pool, undefined, new Date(ctx.block.timestamp), event);
}

export async function processLiquidityRemovedEvent(ctx: EventHandlerContext<Store, { event: true }>): Promise<void> {
  console.debug("processing LiquidityRemovedEvent", ctx.event.id);
  const pabloLiquidityRemovedEvent = new PabloLiquidityRemovedEvent(ctx);
  const liquidityRemovedEvent = getLiquidityRemovedEvent(pabloLiquidityRemovedEvent);
  const who = encodeAccount(liquidityRemovedEvent.who);
  const { poolId, assetAmounts } = liquidityRemovedEvent;

  const pool = await ctx.store.get(PabloPool, {
    where: {
      id: poolId.toString()
    },
    relations: {
      poolAssets: true,
      poolAssetWeights: true,
      lpToken: true
    }
  });

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
  pool.blockId = ctx.block.hash;

  await ctx.store.save(pool);

  // Update or create assets
  for (const [assetId, amount] of assetAmounts) {
    const asset = await getOrCreatePabloAsset(ctx, pool, assetId.toString());

    asset.totalLiquidity -= amount;
    asset.blockId = ctx.block.hash;

    await ctx.store.save(asset);

    const historicalLockedValue = new HistoricalLockedValue({
      id: randomUUID(),
      event,
      amount: -amount,
      accumulatedAmount: asset.totalLiquidity,
      timestamp: new Date(ctx.block.timestamp),
      source: LockedSource.Pablo,
      assetId: assetId.toString(),
      sourceEntityId: pool.id,
      blockId: ctx.block.hash
    });

    await ctx.store.save(historicalLockedValue);

    await getOrCreateHistoricalAssetPrice(ctx, assetId.toString(), ctx.block.timestamp);
  }

  const amounts: Array<PabloAmount> = [];

  for (const [assetId, amount] of assetAmounts) {
    const price = await getOrCreateHistoricalAssetPrice(ctx, assetId.toString(), ctx.block.timestamp);
    if (price) {
      const pabloAmount = new PabloAmount({ assetId: assetId.toString(), amount, price });
      amounts.push(pabloAmount);
    }
  }

  const pabloLiquidityRemoved = new PabloLiquidityRemoved({
    id: ctx.event.id,
    event,
    pool,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash,
    amounts
  });

  await ctx.store.save(pabloLiquidityRemoved);

  const pabloTransaction = new PabloTransaction({
    id: ctx.event.id,
    pool,
    account: who,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash,
    event,
    liquidityRemoved: pabloLiquidityRemoved,
    txType: PabloTx.REMOVE_LIQUIDITY
  });

  await ctx.store.save(pabloTransaction);

  // Calculate and update APR
  await getOrCreateFeeApr(ctx, pool, undefined, new Date(ctx.block.timestamp), event);
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
  pool.blockId = ctx.block.hash;

  await ctx.store.save(pool);

  const baseAsset = await getOrCreatePabloAsset(ctx, pool, baseAssetId.toString());

  const quoteAsset = await getOrCreatePabloAsset(ctx, pool, quoteAssetId.toString());

  baseAsset.totalVolume += baseAmount;
  baseAsset.totalLiquidity = baseAsset.totalLiquidity - baseAmount > 0n ? baseAsset.totalLiquidity - baseAmount : 0n;
  baseAsset.blockId = ctx.block.hash;

  await ctx.store.save(baseAsset);

  const baseHistoricalLockedValue = new HistoricalLockedValue({
    id: randomUUID(),
    event,
    amount: -baseAmount,
    accumulatedAmount: baseAsset.totalLiquidity,
    timestamp: new Date(ctx.block.timestamp),
    source: LockedSource.Pablo,
    assetId: baseAssetId.toString(),
    sourceEntityId: pool.id,
    blockId: ctx.block.hash
  });

  await ctx.store.save(baseHistoricalLockedValue);

  quoteAsset.totalVolume += quoteAmount;
  quoteAsset.totalLiquidity += quoteAmount;
  quoteAsset.blockId = ctx.block.hash;

  await ctx.store.save(quoteAsset);

  const quoteHistoricalLockedValue = new HistoricalLockedValue({
    id: randomUUID(),
    event,
    amount: quoteAmount,
    accumulatedAmount: quoteAsset.totalLiquidity,
    timestamp: new Date(ctx.block.timestamp),
    source: LockedSource.Pablo,
    assetId: quoteAssetId.toString(),
    sourceEntityId: pool.id,
    blockId: ctx.block.hash
  });

  await ctx.store.save(quoteHistoricalLockedValue);

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

  const pabloFee = new PabloFee({
    id: ctx.event.id,
    event,
    pool,
    assetId: fee.assetId.toString(),
    account: who,
    fee: fee.fee,
    lpFee: fee.lpFee,
    ownerFee: fee.ownerFee,
    protocolFee: fee.protocolFee,
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

  const pabloTransaction = new PabloTransaction({
    id: ctx.event.id,
    pool,
    account: who,
    timestamp: new Date(ctx.block.timestamp),
    blockId: ctx.block.hash,
    event,
    swap: pabloSwap,
    txType: PabloTx.SWAP
  });

  await ctx.store.save(pabloTransaction);

  const latestBaseAssetVolume =
    (
      await ctx.store.findOne(HistoricalVolume, {
        where: {
          assetId: baseAssetId.toString(),
          pool: {
            id: pool.id
          },
          source: LockedSource.Pablo
        },
        order: {
          timestamp: "DESC"
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
        },
        order: {
          timestamp: "DESC"
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
  await getOrCreateHistoricalAssetPrice(ctx, baseAssetId.toString(), ctx.block.timestamp);
  await getOrCreateHistoricalAssetPrice(ctx, quoteAssetId.toString(), ctx.block.timestamp);

  // Calculate and update APR
  await getOrCreateFeeApr(ctx, pool, undefined, new Date(ctx.block.timestamp), event);
}
