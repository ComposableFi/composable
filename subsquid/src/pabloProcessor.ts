import {PabloLiquidityAddedEvent, PabloPoolCreatedEvent} from "./types/events";
import {EventHandlerContext} from "@subsquid/substrate-processor";
import * as ss58 from "@subsquid/ss58";
import {get, getOrCreate} from "./dbHelper";
import {Account, PabloPool, PabloPoolAsset, PabloTransaction, PabloTransactionType} from "./model";
import Big from "big.js";
import {CurrencyPair} from "./types/v2100";

function createTransaction(
    ctx: EventHandlerContext,
    pool: PabloPool,
    transactionType: PabloTransactionType,
    priceInQuoteAsset: string,
    baseAssetId: bigint,
    baseAssetAmount: bigint,
    quoteAssetId: bigint,
    quoteAssetAmount: bigint,
) {
    let tx = new PabloTransaction();
    tx.id = ctx.event.id;
    tx.eventId = ctx.event.id;
    tx.pool = pool;
    tx.blockNumber = BigInt(ctx.block.height);
    tx.receivedTimestamp = BigInt(new Date().getTime());
    tx.transactionType = PabloTransactionType.CREATE_POOL;
    tx.priceInQuoteAsset = priceInQuoteAsset;
    tx.baseAssetId = baseAssetId;
    tx.baseAssetAmount = baseAssetAmount;
    tx.quoteAssetId = quoteAssetId;
    tx.quoteAssetAmount = quoteAssetAmount;
    return tx;
}

function createAsset(pool: PabloPool, assetId: bigint, ctx: EventHandlerContext, timestamp: bigint) {
    const asset = new PabloPoolAsset();
    asset.pool = pool;
    asset.id = createPoolAssetId(pool.poolId, assetId);
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
    assets: CurrencyPair
}

function getPoolCreatedEvent(event: PabloPoolCreatedEvent): PoolCreatedEvent {
    if (event.isV2100) {
        const {owner, poolId, assets} = event.asV2100;
        return {owner, poolId, assets};
    } else {
        const {owner, poolId, assets} = event.asLatest;
        return {owner, poolId, assets};
    }
}

export async function processPoolCreatedEvent(ctx: EventHandlerContext, event: PabloPoolCreatedEvent) {
    console.info('processing PoolCreatedEvent', event);
    const poolCreatedEvt = getPoolCreatedEvent(event);
    const owner = ss58.codec("picasso").encode(poolCreatedEvt.owner);
    const pool = await getOrCreate(ctx.store, PabloPool, poolCreatedEvt.poolId.toString());
    // only set values if the owner was missing, i.e a new pool
    if (pool.owner == null) {
        let timestamp = BigInt(new Date().getTime());
        pool.owner = owner;
        pool.poolId = poolCreatedEvt.poolId.toString();
        pool.quoteAssetId = poolCreatedEvt.assets.quote;
        pool.transactionCount = 1;
        pool.totalLiquidity = '0.0';
        pool.totalVolume = '0.0';
        pool.calculatedTimestamp = timestamp;
        pool.blockNumber = BigInt(ctx.block.height);

        let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
        if (tx != undefined) {
            console.error("Unexpected transaction in db", tx);
            throw new Error("Unexpected transaction in db");
        }
        tx = createTransaction(ctx, pool, PabloTransactionType.CREATE_POOL,
            // Following fields are irrelevant for CREATE_POOL
            '0',
            poolCreatedEvt.assets.base,
            BigInt(0),
            poolCreatedEvt.assets.quote,
            BigInt(0));

        let quoteAsset = await get(ctx.store, PabloPoolAsset, createPoolAssetId(pool.poolId, poolCreatedEvt.assets.quote));
        let baseAsset = await get(ctx.store, PabloPoolAsset, createPoolAssetId(pool.poolId, poolCreatedEvt.assets.base));
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

function createPoolAssetId(poolId: string, assetId: bigint): string {
    return poolId + '-' + assetId;
}

interface LiquidityAddedEvent {
    who: Uint8Array,
    poolId: bigint,
    baseAmount: bigint,
    quoteAmount: bigint,
    mintedLp: bigint
}

function getLiquidityAddedEvent(event: PabloLiquidityAddedEvent): LiquidityAddedEvent {
    if (event.isV2100) {
        const {who, poolId, baseAmount, quoteAmount, mintedLp} = event.asV2100;
        return {who, poolId, baseAmount, quoteAmount, mintedLp};
    } else {
        const {who, poolId, baseAmount, quoteAmount, mintedLp} = event.asLatest;
        return {who, poolId, baseAmount, quoteAmount, mintedLp};
    }
}

export async function processLiquidityAddedEvent(ctx: EventHandlerContext, event: PabloLiquidityAddedEvent) {
    console.info('processing LiquidityAddedEvent', event);
    const liquidityAddedEvt = getLiquidityAddedEvent(event);
    const who = ss58.codec("picasso").encode(liquidityAddedEvt.who);
    const pool = await get(ctx.store, PabloPool, liquidityAddedEvt.poolId.toString());
    // only set values if the owner was missing, i.e a new pool
    if (pool != undefined) {
        const timestamp = BigInt(new Date().getTime());
        pool.transactionCount += 1;
        pool.totalLiquidity = Big(pool.totalLiquidity)
            // multiplying by 2 to account for base amount being added
            .add(Big(liquidityAddedEvt.quoteAmount.toString()).mul(2))
            .toString();
        pool.calculatedTimestamp = timestamp;
        pool.blockNumber = BigInt(ctx.block.height);

        // find baseAsset: Following is only valid for dual asset pools
        const baseAsset = pool.poolAssets
            .find((asset) => asset.assetId != pool.quoteAssetId);
        if (baseAsset == undefined) {
            throw new Error('baseAsset not found');
        }
        baseAsset.totalLiquidity += liquidityAddedEvt.baseAmount;
        baseAsset.calculatedTimestamp = timestamp;
        baseAsset.blockNumber = BigInt(ctx.block.height);
        // find quoteAsset
        const quoteAsset = pool.poolAssets
            .find((asset) => asset.assetId == pool.quoteAssetId);
        if (quoteAsset == undefined) {
            throw new Error('quoteAsset not found');
        }
        quoteAsset.totalLiquidity += liquidityAddedEvt.quoteAmount;
        quoteAsset.calculatedTimestamp = timestamp;
        quoteAsset.blockNumber = BigInt(ctx.block.height);

        let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
        if (tx != undefined) {
            throw new Error("Unexpected transaction in db");
        }
        tx = createTransaction(ctx, pool, PabloTransactionType.ADD_LIQUIDITY,
            Big(liquidityAddedEvt.baseAmount.toString())
                .div(Big(liquidityAddedEvt.quoteAmount.toString())).toString(),
            BigInt(baseAsset.assetId),
            liquidityAddedEvt.baseAmount,
            pool.quoteAssetId,
            liquidityAddedEvt.quoteAmount);

        await ctx.store.save(pool);
        await ctx.store.save(baseAsset);
        await ctx.store.save(quoteAsset);
        await ctx.store.save(tx);
    } else {
        throw new Error("Pool not found");
    }
}

