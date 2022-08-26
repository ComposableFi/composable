import { EventHandlerContext } from "@subsquid/substrate-processor";
import Big from "big.js";
import {
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloPoolDeletedEvent,
  PabloSwappedEvent,
} from "./types/events";
import { get, getLatestPoolByPoolId, getOrCreate } from "./dbHelper";
import {
  PabloPool,
  PabloPoolAsset,
  PabloTransaction,
  PabloTransactionType,
} from "./model";
import { CurrencyPair, Fee } from "./types/v2401";
import { encodeAccount } from "./utils";

function createTransaction(
  ctx: EventHandlerContext,
  pool: PabloPool,
  who: string,
  transactionType: PabloTransactionType,
  spotPrice: string,
  baseAssetId: bigint,
  baseAssetAmount: bigint,
  quoteAssetId: bigint,
  quoteAssetAmount: bigint,
  fee?: string
) {
  const tx = new PabloTransaction();
  tx.id = ctx.event.id;
  tx.eventId = ctx.event.id;
  tx.pool = pool;
  tx.who = who;
  tx.blockNumber = BigInt(ctx.block.height);
  tx.receivedTimestamp = BigInt(new Date().getTime());
  tx.transactionType = transactionType;
  tx.spotPrice = spotPrice;
  tx.baseAssetId = baseAssetId;
  tx.baseAssetAmount = baseAssetAmount;
  tx.quoteAssetId = quoteAssetId;
  tx.quoteAssetAmount = quoteAssetAmount;
  tx.fee = fee || "0.0";
  return tx;
}

function createAsset(
  pool: PabloPool,
  assetId: bigint,
  ctx: EventHandlerContext,
  timestamp: bigint
) {
  const asset = new PabloPoolAsset();
  asset.pool = pool;
  asset.id = createPoolAssetId(ctx.event.id, pool.poolId, assetId);
  asset.assetId = assetId;
  asset.blockNumber = BigInt(ctx.block.height);
  asset.totalLiquidity = BigInt(0);
  asset.totalVolume = BigInt(0);
  asset.calculatedTimestamp = timestamp;
  return asset;
}

interface PoolCreatedEvent {
  owner: Uint8Array;
  poolId: bigint;
  assets: CurrencyPair;
}

function getPoolCreatedEvent(event: PabloPoolCreatedEvent): PoolCreatedEvent {
  const { owner, poolId, assets } = event.asV2401 ?? event.asLatest;
  return { owner, poolId, assets };
}

export async function processPoolCreatedEvent(
  ctx: EventHandlerContext,
  event: PabloPoolCreatedEvent
) {
  console.debug("processing PoolCreatedEvent", ctx.event.id);
  const poolCreatedEvt = getPoolCreatedEvent(event);
  const owner = encodeAccount(poolCreatedEvt.owner);
  const pool = await getOrCreate(ctx.store, PabloPool, ctx.event.id);
  // only set values if the owner was missing, i.e a new pool
  if (pool.owner == null) {
    const timestamp = BigInt(new Date().getTime());
    pool.id = ctx.event.id;
    pool.eventId = ctx.event.id;
    pool.owner = owner;
    pool.poolId = poolCreatedEvt.poolId;
    pool.quoteAssetId = poolCreatedEvt.assets.quote;
    pool.transactionCount = 1;
    pool.totalLiquidity = "0.0";
    pool.totalVolume = "0.0";
    pool.totalFees = "0.0";
    pool.calculatedTimestamp = timestamp;
    pool.blockNumber = BigInt(ctx.block.height);

    let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
    if (tx != undefined) {
      console.error("Unexpected transaction in db", tx);
      throw new Error("Unexpected transaction in db");
    }
    tx = createTransaction(
      ctx,
      pool,
      owner,
      PabloTransactionType.CREATE_POOL,
      // Following fields are irrelevant for CREATE_POOL
      "0",
      poolCreatedEvt.assets.base,
      BigInt(0),
      poolCreatedEvt.assets.quote,
      BigInt(0)
    );

    let quoteAsset = await get(
      ctx.store,
      PabloPoolAsset,
      createPoolAssetId(ctx.event.id, pool.poolId, poolCreatedEvt.assets.quote)
    );
    let baseAsset = await get(
      ctx.store,
      PabloPoolAsset,
      createPoolAssetId(ctx.event.id, pool.poolId, poolCreatedEvt.assets.base)
    );
    if (quoteAsset != undefined || baseAsset != undefined) {
      console.error("Unexpected assets for pool in db", quoteAsset, baseAsset);
      throw new Error("Unexpected assets found");
    }
    quoteAsset = createAsset(pool, poolCreatedEvt.assets.quote, ctx, timestamp);
    baseAsset = createAsset(pool, poolCreatedEvt.assets.base, ctx, timestamp);

    await ctx.store.save(pool);
    await ctx.store.save(baseAsset);
    await ctx.store.save(quoteAsset);
    await ctx.store.save(tx);
  }
}

export function createPoolAssetId(
  eventId: string,
  poolId: bigint,
  assetId: bigint
): string {
  return `${eventId}-${poolId}-${assetId}`;
}

interface LiquidityAddedEvent {
  who: Uint8Array;
  poolId: bigint;
  baseAmount: bigint;
  quoteAmount: bigint;
  mintedLp: bigint;
}

function getLiquidityAddedEvent(
  event: PabloLiquidityAddedEvent
): LiquidityAddedEvent {
  const { who, poolId, baseAmount, quoteAmount, mintedLp } =
    event.asV2401 ?? event.asLatest;
  return { who, poolId, baseAmount, quoteAmount, mintedLp };
}

export async function processLiquidityAddedEvent(
  ctx: EventHandlerContext,
  event: PabloLiquidityAddedEvent
) {
  console.debug("processing LiquidityAddedEvent", ctx.event.id);
  const liquidityAddedEvt = getLiquidityAddedEvent(event);
  const who = encodeAccount(liquidityAddedEvt.who);
  const pool = await getLatestPoolByPoolId(ctx.store, liquidityAddedEvt.poolId);
  // only set values if the owner was missing, i.e a new pool
  if (pool != undefined) {
    const timestamp = BigInt(new Date().getTime());
    pool.id = ctx.event.id;
    pool.eventId = ctx.event.id;
    pool.transactionCount += 1;
    pool.totalLiquidity = Big(pool.totalLiquidity)
      // multiplying by 2 to account for base amount being added
      .add(Big(liquidityAddedEvt.quoteAmount.toString()).mul(2))
      .toString();
    pool.calculatedTimestamp = timestamp;
    pool.blockNumber = BigInt(ctx.block.height);

    // find baseAsset: Following is only valid for dual asset pools
    const baseAsset = pool.poolAssets.find(
      (asset) => asset.assetId != pool.quoteAssetId
    );
    if (baseAsset == undefined) {
      throw new Error("baseAsset not found");
    }
    baseAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      baseAsset.assetId
    );
    baseAsset.pool = pool;
    baseAsset.totalLiquidity += liquidityAddedEvt.baseAmount;
    baseAsset.calculatedTimestamp = timestamp;
    baseAsset.blockNumber = BigInt(ctx.block.height);
    // find quoteAsset
    const quoteAsset = pool.poolAssets.find(
      (asset) => asset.assetId == pool.quoteAssetId
    );
    if (quoteAsset == undefined) {
      throw new Error("quoteAsset not found");
    }
    quoteAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      quoteAsset.assetId
    );
    quoteAsset.pool = pool;
    quoteAsset.totalLiquidity += liquidityAddedEvt.quoteAmount;
    quoteAsset.calculatedTimestamp = timestamp;
    quoteAsset.blockNumber = BigInt(ctx.block.height);

    let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
    if (tx != undefined) {
      throw new Error("Unexpected transaction in db");
    }
    tx = createTransaction(
      ctx,
      pool,
      who,
      PabloTransactionType.ADD_LIQUIDITY,
      Big(liquidityAddedEvt.baseAmount.toString())
        .div(Big(liquidityAddedEvt.quoteAmount.toString()))
        .toString(),
      BigInt(baseAsset.assetId),
      liquidityAddedEvt.baseAmount,
      pool.quoteAssetId,
      liquidityAddedEvt.quoteAmount
    );

    await ctx.store.save(pool);
    await ctx.store.save(baseAsset);
    await ctx.store.save(quoteAsset);
    await ctx.store.save(tx);
  } else {
    throw new Error("Pool not found");
  }
}

interface LiquidityRemovedEvent {
  who: Uint8Array;
  poolId: bigint;
  baseAmount: bigint;
  quoteAmount: bigint;
  totalIssuance: bigint;
}

function getLiquidityRemovedEvent(
  event: PabloLiquidityRemovedEvent
): LiquidityRemovedEvent {
  const { who, poolId, baseAmount, quoteAmount, totalIssuance } =
    event.asV2401 ?? event.asLatest;
  return { who, poolId, baseAmount, quoteAmount, totalIssuance };
}

export async function processLiquidityRemovedEvent(
  ctx: EventHandlerContext,
  event: PabloLiquidityRemovedEvent
) {
  console.debug("processing LiquidityAddedEvent", ctx.event.id);
  const liquidityRemovedEvt = getLiquidityRemovedEvent(event);
  const who = encodeAccount(liquidityRemovedEvt.who);
  const pool = await getLatestPoolByPoolId(
    ctx.store,
    liquidityRemovedEvt.poolId
  );
  // only set values if the owner was missing, i.e a new pool
  if (pool != undefined) {
    const timestamp = BigInt(new Date().getTime());
    pool.id = ctx.event.id;
    pool.eventId = ctx.event.id;
    pool.transactionCount += 1;
    pool.totalLiquidity = Big(pool.totalLiquidity)
      // multiplying by 2 to account for base amount being removed
      .sub(Big(liquidityRemovedEvt.quoteAmount.toString()).mul(2))
      .toString();
    pool.calculatedTimestamp = timestamp;
    pool.blockNumber = BigInt(ctx.block.height);

    // find baseAsset: Following is only valid for dual asset pools
    const baseAsset = pool.poolAssets.find(
      (asset) => asset.assetId != pool.quoteAssetId
    );
    if (baseAsset == undefined) {
      throw new Error("baseAsset not found");
    }
    baseAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      baseAsset.assetId
    );
    baseAsset.pool = pool;
    baseAsset.totalLiquidity -= liquidityRemovedEvt.baseAmount;
    baseAsset.calculatedTimestamp = timestamp;
    baseAsset.blockNumber = BigInt(ctx.block.height);
    // find quoteAsset
    const quoteAsset = pool.poolAssets.find(
      (asset) => asset.assetId == pool.quoteAssetId
    );
    if (quoteAsset == undefined) {
      throw new Error("quoteAsset not found");
    }
    quoteAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      quoteAsset.assetId
    );
    quoteAsset.pool = pool;
    quoteAsset.totalLiquidity -= liquidityRemovedEvt.quoteAmount;
    quoteAsset.calculatedTimestamp = timestamp;
    quoteAsset.blockNumber = BigInt(ctx.block.height);

    let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
    if (tx != undefined) {
      throw new Error("Unexpected transaction in db");
    }
    tx = createTransaction(
      ctx,
      pool,
      who,
      PabloTransactionType.REMOVE_LIQUIDITY,
      Big(liquidityRemovedEvt.baseAmount.toString())
        .div(Big(liquidityRemovedEvt.quoteAmount.toString()))
        .toString(),
      BigInt(baseAsset.assetId),
      liquidityRemovedEvt.baseAmount,
      pool.quoteAssetId,
      liquidityRemovedEvt.quoteAmount
    );

    await ctx.store.save(pool);
    await ctx.store.save(baseAsset);
    await ctx.store.save(quoteAsset);
    await ctx.store.save(tx);
  } else {
    throw new Error("Pool not found");
  }
}

interface SwappedEvent {
  poolId: bigint;
  who: Uint8Array;
  baseAsset: bigint;
  quoteAsset: bigint;
  baseAmount: bigint;
  quoteAmount: bigint;
  fee: Fee;
}

function getSwappedEvent(event: PabloSwappedEvent): SwappedEvent {
  const { poolId, who, baseAsset, quoteAsset, baseAmount, quoteAmount, fee } =
    event.asV2401 ?? event.asLatest;
  return { poolId, who, baseAsset, quoteAsset, baseAmount, quoteAmount, fee };
}

export async function processSwappedEvent(
  ctx: EventHandlerContext,
  event: PabloSwappedEvent
) {
  console.debug("processing SwappedEvent", ctx.event.id);
  const swappedEvt = getSwappedEvent(event);
  const who = encodeAccount(swappedEvt.who);
  const pool = await getLatestPoolByPoolId(ctx.store, swappedEvt.poolId);
  // only set values if the owner was missing, i.e a new pool
  if (pool != undefined) {
    const isReverse: boolean = pool.quoteAssetId != swappedEvt.quoteAsset;
    const timestamp = BigInt(new Date().getTime());
    pool.id = ctx.event.id;
    pool.eventId = ctx.event.id;
    pool.transactionCount += 1;
    pool.calculatedTimestamp = timestamp;
    pool.blockNumber = BigInt(ctx.block.height);
    // find baseAsset: Following is only valid for dual asset pools
    const baseAsset = pool.poolAssets.find(
      (asset) => asset.assetId != pool.quoteAssetId
    );
    if (baseAsset == undefined) {
      throw new Error("baseAsset not found");
    }
    // find quoteAsset
    const quoteAsset = pool.poolAssets.find(
      (asset) => asset.assetId == pool.quoteAssetId
    );
    if (quoteAsset == undefined) {
      throw new Error("quoteAsset not found");
    }
    const feesLeavingPool = swappedEvt.fee.fee - swappedEvt.fee.lpFee;
    const spotPrice = isReverse
      ? Big(swappedEvt.baseAmount.toString()).div(
          Big(swappedEvt.quoteAmount.toString())
        )
      : Big(swappedEvt.quoteAmount.toString()).div(
          Big(swappedEvt.baseAmount.toString())
        );
    if (isReverse) {
      console.debug("Reverse swap");
      // volume
      pool.totalVolume = Big(pool.totalVolume)
        .add(Big(swappedEvt.baseAmount.toString()))
        .toString();
      baseAsset.totalVolume += swappedEvt.quoteAmount;
      quoteAsset.totalVolume += swappedEvt.baseAmount;

      // for reverse exchange "default quote" (included as the base amount in the evt) amount leaves the pool
      baseAsset.totalLiquidity += swappedEvt.quoteAmount;
      quoteAsset.totalLiquidity -= swappedEvt.baseAmount;
      quoteAsset.totalLiquidity -= feesLeavingPool;
    } else {
      console.debug("Normal swap");
      // volume
      pool.totalVolume = Big(pool.totalVolume)
        .add(Big(swappedEvt.quoteAmount.toString()))
        .toString();
      baseAsset.totalVolume += swappedEvt.baseAmount;
      quoteAsset.totalVolume += swappedEvt.quoteAmount;

      // for normal exchange "default quote" amount gets into the pool
      baseAsset.totalLiquidity -= swappedEvt.baseAmount;
      baseAsset.totalLiquidity -= feesLeavingPool;
      quoteAsset.totalLiquidity += swappedEvt.quoteAmount;
    }
    // fee and liquidity
    pool.totalLiquidity = Big(pool.totalLiquidity)
      .sub(
        calculateFeeInQuoteAsset(
          spotPrice,
          quoteAsset.assetId,
          swappedEvt.fee.assetId,
          feesLeavingPool
        )
      )
      .toString();
    const fee = calculateFeeInQuoteAsset(
      spotPrice,
      quoteAsset.assetId,
      swappedEvt.fee.assetId,
      swappedEvt.fee.fee
    );
    pool.totalFees = Big(pool.totalFees).add(fee).toString();
    baseAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      baseAsset.assetId
    );
    baseAsset.pool = pool;
    baseAsset.calculatedTimestamp = timestamp;
    baseAsset.blockNumber = BigInt(ctx.block.height);
    quoteAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      quoteAsset.assetId
    );
    quoteAsset.pool = pool;
    quoteAsset.calculatedTimestamp = timestamp;
    quoteAsset.blockNumber = BigInt(ctx.block.height);

    let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
    if (tx != undefined) {
      throw new Error("Unexpected transaction in db");
    }
    tx = createTransaction(
      ctx,
      pool,
      who,
      PabloTransactionType.SWAP,
      spotPrice.toString(),
      swappedEvt.baseAsset,
      swappedEvt.baseAmount,
      swappedEvt.quoteAsset,
      swappedEvt.quoteAmount,
      fee.toString()
    );

    await ctx.store.save(pool);
    await ctx.store.save(baseAsset);
    await ctx.store.save(quoteAsset);
    await ctx.store.save(tx);
  } else {
    throw new Error("Pool not found");
  }
}

interface PoolDeletedEvent {
  poolId: bigint;
  baseAmount: bigint;
  quoteAmount: bigint;
}

function getPoolDeletedEvent(event: PabloPoolDeletedEvent): PoolDeletedEvent {
  const { poolId, baseAmount, quoteAmount } = event.asV2401 ?? event.asLatest;
  return { poolId, baseAmount, quoteAmount };
}

export async function processPoolDeletedEvent(
  ctx: EventHandlerContext,
  event: PabloPoolDeletedEvent
) {
  console.debug("processing LiquidityAddedEvent", ctx.event.id);
  const poolDeletedEvent = getPoolDeletedEvent(event);
  const pool = await getLatestPoolByPoolId(ctx.store, poolDeletedEvent.poolId);
  // only set values if the owner was missing, i.e a new pool
  if (pool != undefined) {
    const who = pool.owner;
    const timestamp = BigInt(new Date().getTime());
    pool.id = ctx.event.id;
    pool.eventId = ctx.event.id;
    pool.transactionCount += 1;
    pool.totalLiquidity = "0.0";
    pool.calculatedTimestamp = timestamp;
    pool.blockNumber = BigInt(ctx.block.height);

    // find baseAsset: Following is only valid for dual asset pools
    const baseAsset = pool.poolAssets.find(
      (asset) => asset.assetId != pool.quoteAssetId
    );
    if (baseAsset == undefined) {
      throw new Error("baseAsset not found");
    }
    baseAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      baseAsset.assetId
    );
    baseAsset.pool = pool;
    baseAsset.totalLiquidity -= poolDeletedEvent.baseAmount;
    baseAsset.calculatedTimestamp = timestamp;
    baseAsset.blockNumber = BigInt(ctx.block.height);
    // find quoteAsset
    const quoteAsset = pool.poolAssets.find(
      (asset) => asset.assetId == pool.quoteAssetId
    );
    if (quoteAsset == undefined) {
      throw new Error("quoteAsset not found");
    }
    quoteAsset.id = createPoolAssetId(
      ctx.event.id,
      pool.poolId,
      quoteAsset.assetId
    );
    quoteAsset.pool = pool;
    quoteAsset.totalLiquidity -= poolDeletedEvent.quoteAmount;
    quoteAsset.calculatedTimestamp = timestamp;
    quoteAsset.blockNumber = BigInt(ctx.block.height);

    let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
    if (tx != undefined) {
      throw new Error("Unexpected transaction in db");
    }
    tx = createTransaction(
      ctx,
      pool,
      who,
      PabloTransactionType.DELETE_POOL,
      Big(poolDeletedEvent.baseAmount.toString())
        .div(Big(poolDeletedEvent.quoteAmount.toString()))
        .toString(),
      BigInt(baseAsset.assetId),
      poolDeletedEvent.baseAmount,
      pool.quoteAssetId,
      poolDeletedEvent.quoteAmount
    );

    await ctx.store.save(pool);
    await ctx.store.save(baseAsset);
    await ctx.store.save(quoteAsset);
    await ctx.store.save(tx);
  } else {
    throw new Error("Pool not found");
  }
}

function calculateFeeInQuoteAsset(
  spotPrice: Big,
  quoteAsset: bigint,
  feeAsset: bigint,
  fee: bigint
): Big {
  // calculate the quote amount based on the exchange rate if the fees are in the base asset
  return feeAsset == quoteAsset
    ? Big(fee.toString())
    : spotPrice.mul(fee.toString());
}
