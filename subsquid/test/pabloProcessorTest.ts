import {assert, expect} from 'chai';
import * as ss58 from "@subsquid/ss58";
import {EventHandlerContext, Store, SubstrateBlock, SubstrateEvent} from "@subsquid/substrate-processor";
import {anyOfClass, anything, capture, instance, mock, verify, when} from "ts-mockito";
import {PabloPool, PabloPoolAsset, PabloTransaction, PabloTransactionType} from "../src/model";
import {processLiquidityAddedEvent, processPoolCreatedEvent, processSwappedEvent} from "../src/pabloProcessor";
import {PabloLiquidityAddedEvent, PabloPoolCreatedEvent, PabloSwappedEvent} from "../src/types/events";
import {randomFill, randomUUID} from "crypto";
import Big from "big.js";

const UNIT = 1_000_000_000_000;

function assertPool(
    poolArg: PabloPool,
    poolId: string,
    owner: string,
    blockNumber: bigint,
    transactionCount: number,
    quoteAssetId: bigint,
    totalLiquidity: string,
    totalVolume: string
) {
    expect(poolArg.poolId).eq(poolId);
    expect(poolArg.owner)
        .eq(owner);
    expect(poolArg.blockNumber).eq(blockNumber);
    expect(poolArg.transactionCount).eq(transactionCount);
    expect(poolArg.quoteAssetId).eq(quoteAssetId);
    expect(poolArg.totalLiquidity).eq(totalLiquidity);
    expect(poolArg.totalVolume).eq(totalVolume);
}

function assertTransaction(
    txArg: PabloTransaction,
    id: string,
    who: string,
    transactionType: PabloTransactionType,
    spotPrice: string,
    baseAssetId: bigint,
    baseAssetAmount: bigint,
    quoteAssetId: bigint,
    quoteAssetAmount: bigint
) {
    expect(txArg.id).eq(id);
    expect(txArg.transactionType).eq(transactionType);
    expect(txArg.who).eq(who);
    expect(txArg.spotPrice).eq(spotPrice);
    expect(txArg.baseAssetId).eq(baseAssetId);
    expect(txArg.baseAssetAmount).eq(baseAssetAmount);
    expect(txArg.quoteAssetId).eq(quoteAssetId);
    expect(txArg.quoteAssetAmount).eq(quoteAssetAmount);
}

function assertAsset(
    asset: PabloPoolAsset,
    id: string,
    assetId: bigint,
    blockNumber: bigint,
    totalLiquidity: bigint,
    totalVolume: bigint
) {
    expect(asset.id).eq(id);
    expect(asset.blockNumber).eq(blockNumber);
    expect(asset.totalLiquidity).eq(totalLiquidity);
    expect(asset.totalVolume).eq(totalVolume);
    expect(asset.assetId).eq(assetId);
}

function createCtx(storeMock: Store, blockHeight: number) {
    let blockMock: SubstrateBlock = mock<SubstrateBlock>();
    blockMock.height = blockHeight;
    let event: SubstrateEvent = mock<SubstrateEvent>();
    event.id = randomUUID();
    let ctxMock: EventHandlerContext = mock<EventHandlerContext>();
    let ctx: EventHandlerContext = instance(ctxMock);
    ctx.store = instance(storeMock);
    ctx.block = blockMock;
    return ctx;
}

function createAccount() {
    let acc = Uint8Array.of(...new Array<any>(32));
    randomFill(acc, (err) => err != null ? console.log(err) : '');
    return acc;
}

function createPoolCreatedEvent() {
    let eventMock = mock(PabloPoolCreatedEvent);
    let owner = createAccount();
    let evt = {
        assets: {base: BigInt(1), quote: BigInt(4)},
        owner: owner,
        poolId: BigInt(1)
    }
    when(eventMock.asV2100).thenReturn(evt);
    when(eventMock.asLatest).thenReturn(evt);
    let event = instance(eventMock);
    return {owner, event};
}

function createLiquidityAddedEvent() {
    let eventMock = mock(PabloLiquidityAddedEvent);
    let who = createAccount();
    let evt = {
        who: who,
        poolId: BigInt(1),
        baseAmount: BigInt(10_000 * UNIT),
        quoteAmount: BigInt(10_000 * UNIT),
        mintedLp: BigInt(200)
    };
    when(eventMock.asV2100).thenReturn(evt);
    when(eventMock.asLatest).thenReturn(evt);
    let event = instance(eventMock);
    return {who, event};
}

function createSwappedEvent(
    baseAsset?: bigint,
    quoteAsset?: bigint,
    feeAsset?: bigint,
    baseAmount?: bigint,
    quoteAmount?: bigint,
    fee?: bigint
) {
    let eventMock = mock(PabloSwappedEvent);
    let who = createAccount();
    let evt = {
        poolId: BigInt(1),
        who: who,
        baseAsset: baseAsset || BigInt(1),
        quoteAsset: quoteAsset || BigInt(4),
        feeAsset: feeAsset || BigInt(4),
        baseAmount: baseAmount || BigInt(100 * UNIT),
        quoteAmount: quoteAmount || BigInt(25 * UNIT),
        fee: fee || BigInt(UNIT)
    };
    when(eventMock.asV2100).thenReturn(evt);
    when(eventMock.asLatest).thenReturn(evt);
    let event = instance(eventMock);
    return {who, event};
}

function encodeAccount(owner: Uint8Array) {
    return ss58.codec("picasso").encode(owner);
}

function createZeroAsset(id: string, assetId: bigint) {
    let asset = new PabloPoolAsset();
    asset.blockNumber = BigInt(1);
    asset.totalVolume = BigInt(0);
    asset.totalLiquidity = BigInt(0);
    asset.assetId = assetId;
    asset.id = id;
    return asset;
}

function createZeroPool() {
    let baseAsset = createZeroAsset('1-1', BigInt(1));
    let quoteAsset = createZeroAsset('1-4', BigInt(4));
    let pabloPool = new PabloPool();
    pabloPool.poolId = '1';
    pabloPool.owner = encodeAccount(createAccount());
    pabloPool.quoteAssetId = BigInt(4);
    pabloPool.totalLiquidity = '0.0';
    pabloPool.totalVolume = '0.0';
    pabloPool.transactionCount = 1;
    pabloPool.blockNumber = BigInt(1);
    pabloPool.poolAssets = [
        baseAsset,
        quoteAsset
    ];
    return pabloPool;
}

function addLiquidity(pool: PabloPool, baseAssetAmount: bigint, quoteAssetAmount: bigint) {
    assert(baseAssetAmount == quoteAssetAmount, 'base and quote amounts must be equal')
    pool.transactionCount += 1;
    pool.totalLiquidity = Big(pool.totalLiquidity)
        .add(Big(quoteAssetAmount.toString()).mul(2))
        .toString();
    pool.poolAssets[0].totalLiquidity += baseAssetAmount;
    pool.poolAssets[1].totalLiquidity += quoteAssetAmount;
}

describe('PoolCreated Tests', function () {

    it('Should create the Pool, Assets and Transaction correctly', async function () {
        // given
        let pabloPool = new PabloPool();
        let storeMock: Store = mock<Store>();
        when(storeMock.get<PabloPool>(PabloPool, anything()))
            .thenReturn(Promise.resolve(pabloPool));
        let ctx = createCtx(storeMock, 1);
        let {owner, event} = createPoolCreatedEvent();

        // when
        await processPoolCreatedEvent(ctx, event);

        // then
        verify(storeMock.save(anyOfClass(PabloPool))).once();
        const [poolArg] = capture(storeMock.save).first();
        assertPool(
            poolArg as unknown as PabloPool,
            '1',
            encodeAccount(owner),
            BigInt(1),
            1,
            BigInt(4),
            '0.0',
            '0.0'
        );
        verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
        const [assetOneArg] = capture(storeMock.save).second();
        assertAsset(assetOneArg as unknown as PabloPoolAsset,
            '1-1',
            BigInt(1),
            BigInt(1),
            BigInt(0),
            BigInt(0)
        );
        const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
        assertAsset(assetTwoArg as unknown as PabloPoolAsset,
            '1-4',
            BigInt(4),
            BigInt(1),
            BigInt(0),
            BigInt(0)
        );
        verify(storeMock.save(anyOfClass(PabloTransaction))).once();
        const [txArg] = capture(storeMock.save).last();
        assertTransaction(
            txArg as unknown as PabloTransaction,
            ctx.event.id,
            encodeAccount(owner),
            PabloTransactionType.CREATE_POOL,
            '0',
            BigInt(1),
            BigInt(0),
            BigInt(4),
            BigInt(0)
        );
    });
});

describe('Liquidity Added Tests', function () {

    it('Should add liquidity to the Pool and record Assets and Transaction correctly', async function () {
        // given
        let pabloPool = createZeroPool();
        pabloPool.poolAssets[0]
        let storeMock: Store = mock<Store>();
        when(storeMock.get<PabloPool>(PabloPool, anything()))
            .thenReturn(Promise.resolve(pabloPool));
        let ctx = createCtx(storeMock, 1);
        let {who, event} = createLiquidityAddedEvent();

        // when
        await processLiquidityAddedEvent(ctx, event);

        // then
        verify(storeMock.save(anyOfClass(PabloPool))).once();
        const [poolArg] = capture(storeMock.save).first();
        assertPool(
            poolArg as unknown as PabloPool,
            '1',
            pabloPool.owner,
            BigInt(1),
            2,
            BigInt(4),
            '20000000000000000',
            '0.0'
        );
        verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
        const [assetOneArg] = capture(storeMock.save).second();
        assertAsset(assetOneArg as unknown as PabloPoolAsset,
            '1-1',
            BigInt(1),
            BigInt(1),
            BigInt(10_000 * UNIT),
            BigInt(0)
        );
        const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
        assertAsset(assetTwoArg as unknown as PabloPoolAsset,
            '1-4',
            BigInt(4),
            BigInt(1),
            BigInt(10_000 * UNIT),
            BigInt(0)
        );
        verify(storeMock.save(anyOfClass(PabloTransaction))).once();
        const [txArg] = capture(storeMock.save).last();
        assertTransaction(
            txArg as unknown as PabloTransaction,
            ctx.event.id,
            encodeAccount(who),
            PabloTransactionType.ADD_LIQUIDITY,
            '1',
            BigInt(1),
            BigInt(10_000 * UNIT),
            BigInt(4),
            BigInt(10_000 * UNIT)
        );
    });
});


describe('Swapped Tests', function () {

    it('Should record Pool, Assets and Transaction correctly for normal swap', async function () {
        // given
        let pabloPool = createZeroPool();
        addLiquidity(pabloPool, BigInt(10_000 * UNIT), BigInt(10_000 * UNIT));
        let storeMock: Store = mock<Store>();
        when(storeMock.get<PabloPool>(PabloPool, anything()))
            .thenReturn(Promise.resolve(pabloPool));
        let ctx = createCtx(storeMock, 1);
        let {who, event} = createSwappedEvent();

        // when
        await processSwappedEvent(ctx, event);

        // then
        verify(storeMock.save(anyOfClass(PabloPool))).once();
        const [poolArg] = capture(storeMock.save).first();
        assertPool(
            poolArg as unknown as PabloPool,
            '1',
            pabloPool.owner,
            BigInt(1),
            3,
            BigInt(4),
            '19999000000000000',
            '25000000000000'
        );
        verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
        const [assetOneArg] = capture(storeMock.save).second();
        assertAsset(assetOneArg as unknown as PabloPoolAsset,
            '1-1',
            BigInt(1),
            BigInt(1),
            BigInt(9899 * UNIT),
            BigInt(100 * UNIT)
        );
        const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
        assertAsset(assetTwoArg as unknown as PabloPoolAsset,
            '1-4',
            BigInt(4),
            BigInt(1),
            BigInt(10_025 * UNIT),
            BigInt(25 * UNIT)
        );
        verify(storeMock.save(anyOfClass(PabloTransaction))).once();
        const [txArg] = capture(storeMock.save).last();
        assertTransaction(
            txArg as unknown as PabloTransaction,
            ctx.event.id,
            encodeAccount(who),
            PabloTransactionType.SWAP,
            '0.25',
            BigInt(1),
            BigInt(100 * UNIT),
            BigInt(4),
            BigInt(25 * UNIT)
        );
    });

    it('Should record Pool, Assets and Transaction correctly for reverse swap', async function () {
        // given
        let pabloPool = createZeroPool();
        addLiquidity(pabloPool, BigInt(10_000 * UNIT), BigInt(10_000 * UNIT));
        let storeMock: Store = mock<Store>();
        when(storeMock.get<PabloPool>(PabloPool, anything()))
            .thenReturn(Promise.resolve(pabloPool));
        let ctx = createCtx(storeMock, 1);
        let {who, event} = createSwappedEvent(
            BigInt(4),
            BigInt(1),
            BigInt(1),
            BigInt(12 * UNIT),
            BigInt(25 * UNIT),
            BigInt(UNIT)
        );

        // when
        await processSwappedEvent(ctx, event);

        // then
        verify(storeMock.save(anyOfClass(PabloPool))).once();
        const [poolArg] = capture(storeMock.save).first();
        assertPool(
            poolArg as unknown as PabloPool,
            '1',
            pabloPool.owner,
            BigInt(1),
            3,
            BigInt(4),
            '19999520000000000',
            '12000000000000'
        );
        verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
        const [assetOneArg] = capture(storeMock.save).second();
        assertAsset(assetOneArg as unknown as PabloPoolAsset,
            '1-1',
            BigInt(1),
            BigInt(1),
            BigInt(10_025 * UNIT),
            BigInt(25 * UNIT)
        );
        const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
        assertAsset(assetTwoArg as unknown as PabloPoolAsset,
            '1-4',
            BigInt(4),
            BigInt(1),
            BigInt(9987 * UNIT),
            BigInt(12 * UNIT)
        );
        verify(storeMock.save(anyOfClass(PabloTransaction))).once();
        const [txArg] = capture(storeMock.save).last();
        assertTransaction(
            txArg as unknown as PabloTransaction,
            ctx.event.id,
            encodeAccount(who),
            PabloTransactionType.SWAP,
            '0.48',
            BigInt(4),
            BigInt(12 * UNIT),
            BigInt(1),
            BigInt(25 * UNIT)
        );
    });
});
