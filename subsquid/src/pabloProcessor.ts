import {BalancesTransferEvent, PabloPoolCreatedEvent} from "./types/events";
import {EventHandlerContext} from "@subsquid/substrate-processor";
import * as ss58 from "@subsquid/ss58";
import {get, getOrCreate} from "./dbHelper";
import {Account, PabloPool} from "./model";

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
        pool.transactionCount = 0;
        pool.totalLiquidity = '0.0';
        pool.totalVolume = '0.0';
        pool.calculatedTimestamp = BigInt(new Date().getTime());
        pool.blockNumber = BigInt(ctx.block.height);
        await ctx.store.save(pool);
    }
}

interface PoolCreatedEvent {
    owner: Uint8Array;
    poolId: bigint;
}

function getPoolCreatedEvent(event: PabloPoolCreatedEvent): PoolCreatedEvent {
    if (event.isV2100) {
        const {owner, poolId} = event.asV2100;
        return { owner, poolId };
    } else {
        const {owner, poolId} = event.asLatest;
        return { owner, poolId };
    }
}
