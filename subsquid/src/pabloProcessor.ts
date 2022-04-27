import {PabloPoolCreatedEvent} from "./types/events";
import {EventHandlerContext} from "@subsquid/substrate-processor";
import * as ss58 from "@subsquid/ss58";
import {get, getOrCreate} from "./dbHelper";
import {Account, PabloPool, PabloTransaction, PabloTransactionType} from "./model";

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

export async function processPoolCreatedEvent(ctx: EventHandlerContext, event: PabloPoolCreatedEvent) {
    const poolCreatedEvt = getPoolCreatedEvent(event);
    const owner = ss58.codec("picasso").encode(poolCreatedEvt.owner);
    const ownerAcc = await getOrCreate(ctx.store, Account, owner);
    const pool = await getOrCreate(ctx.store, PabloPool, poolCreatedEvt.poolId.toString());
    // only set values if the owner was missing, i.e a new pool
    if (pool.owner == null) {
        pool.owner = owner;
        pool.poolId = poolCreatedEvt.poolId.toString();
        pool.quoteAssetId = BigInt(0);
        pool.transactionCount = 1;
        pool.totalLiquidity = '0.0';
        pool.totalVolume = '0.0';
        pool.calculatedTimestamp = BigInt(new Date().getTime());
        pool.blockNumber = BigInt(ctx.block.height);

        let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
        if (tx != undefined) {
            throw new Error("Unexpected transaction in db");
        }
        tx = createTransaction(ctx, pool, PabloTransactionType.CREATE_POOL,
            // Following fields are irrelevant for CREATE_POOL
            '0',
            BigInt(0),
            BigInt(0),
            BigInt(0),
            BigInt(0));
        await ctx.store.save(pool);
        await ctx.store.save(tx);
    }
}

interface PoolCreatedEvent {
    owner: Uint8Array;
    poolId: bigint;
}

function getPoolCreatedEvent(event: PabloPoolCreatedEvent): PoolCreatedEvent {
    if (event.isV2100) {
        const {owner, poolId} = event.asV2100;
        return {owner, poolId};
    } else {
        const {owner, poolId} = event.asLatest;
        return {owner, poolId};
    }
}
