import { assert, expect } from "chai";
import { Store } from "@subsquid/substrate-processor";
import {
  anyOfClass,
  anything,
  capture,
  instance,
  mock,
  verify,
  when,
} from "ts-mockito";
import {
  PabloPool,
  PabloPoolAsset,
  PabloTransaction,
  PabloTransactionType,
} from "../src/model";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent,
  processPoolDeletedEvent,
  processSwappedEvent,
} from "../src/pabloProcessor";
import {
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloPoolDeletedEvent,
  PabloSwappedEvent,
} from "../src/types/events";
import { randomUUID } from "crypto";
import Big from "big.js";
import { Fee } from "../src/types/v2401";
import { createAccount, createCtx, encodeAccount } from "../src/utils";

const UNIT = 1_000_000_000_000;

function assertPool(
  poolArg: PabloPool,
  id: string,
  poolId: bigint,
  owner: string,
  blockNumber: bigint,
  transactionCount: number,
  quoteAssetId: bigint,
  totalLiquidity: string,
  totalVolume: string,
  totalFees: string
) {
  expect(poolArg.id).eq(id);
  expect(poolArg.poolId).eq(poolId);
  expect(poolArg.owner).eq(owner);
  expect(poolArg.blockNumber).eq(blockNumber);
  expect(poolArg.transactionCount).eq(transactionCount);
  expect(poolArg.quoteAssetId).eq(quoteAssetId);
  expect(poolArg.totalLiquidity).eq(totalLiquidity);
  expect(poolArg.totalVolume).eq(totalVolume);
  expect(poolArg.totalFees).eq(totalFees);
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
  quoteAssetAmount: bigint,
  fee: string
) {
  expect(txArg.id).eq(id);
  expect(txArg.transactionType).eq(transactionType);
  expect(txArg.who).eq(who);
  expect(txArg.spotPrice).eq(spotPrice);
  expect(txArg.baseAssetId).eq(baseAssetId);
  expect(txArg.baseAssetAmount).eq(baseAssetAmount);
  expect(txArg.quoteAssetId).eq(quoteAssetId);
  expect(txArg.quoteAssetAmount).eq(quoteAssetAmount);
  expect(txArg.fee).eq(fee);
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

function createPoolCreatedEvent() {
  let eventMock = mock(PabloPoolCreatedEvent);
  let owner = createAccount();
  let evt = {
    assets: { base: BigInt(1), quote: BigInt(4) },
    owner: owner,
    poolId: BigInt(1),
  };
  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);
  let event = instance(eventMock);
  return { owner, event };
}

function createLiquidityAddedEvent() {
  let eventMock = mock(PabloLiquidityAddedEvent);
  let who = createAccount();
  let evt = {
    who: who,
    poolId: BigInt(1),
    baseAmount: BigInt(10_000 * UNIT),
    quoteAmount: BigInt(10_000 * UNIT),
    mintedLp: BigInt(200),
  };
  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);
  let event = instance(eventMock);
  return { who, event };
}

function createLiquidityRemovedEvent() {
  let eventMock = mock(PabloLiquidityRemovedEvent);
  let who = createAccount();
  let evt = {
    who: who,
    poolId: BigInt(1),
    baseAmount: BigInt(10_000 * UNIT),
    quoteAmount: BigInt(10_000 * UNIT),
    totalIssuance: BigInt(10_000),
  };
  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);
  let event = instance(eventMock);
  return { who, event };
}

function createPoolDeletedEvent() {
  let eventMock = mock(PabloPoolDeletedEvent);
  let who = createAccount();
  let evt = {
    poolId: BigInt(1),
    baseAmount: BigInt(10_000 * UNIT),
    quoteAmount: BigInt(10_000 * UNIT),
  };
  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);
  let event = instance(eventMock);
  return { who, event };
}

function createSwappedEvent(
  baseAsset?: bigint,
  quoteAsset?: bigint,
  baseAmount?: bigint,
  quoteAmount?: bigint,
  fee?: Fee
) {
  let eventMock = mock(PabloSwappedEvent);
  let who = createAccount();
  let evt = {
    poolId: BigInt(1),
    who: who,
    baseAsset: baseAsset || BigInt(1),
    quoteAsset: quoteAsset || BigInt(4),
    baseAmount: baseAmount || BigInt(100 * UNIT),
    quoteAmount: quoteAmount || BigInt(25 * UNIT),
    fee: fee || {
      fee: BigInt(UNIT),
      lpFee: BigInt(UNIT),
      ownerFee: BigInt(0),
      protocolFee: BigInt(0),
      assetId: BigInt(4),
    },
  };
  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);
  let event = instance(eventMock);
  return { who, event };
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
  let baseAsset = createZeroAsset("1-1", BigInt(1));
  let quoteAsset = createZeroAsset("1-4", BigInt(4));
  let pabloPool = new PabloPool();
  pabloPool.id = randomUUID();
  pabloPool.poolId = BigInt(1);
  pabloPool.owner = encodeAccount(createAccount());
  pabloPool.quoteAssetId = BigInt(4);
  pabloPool.totalLiquidity = "0.0";
  pabloPool.totalVolume = "0.0";
  pabloPool.totalFees = "0.0";
  pabloPool.transactionCount = 1;
  pabloPool.blockNumber = BigInt(1);
  pabloPool.poolAssets = [baseAsset, quoteAsset];
  return pabloPool;
}

function addLiquidity(
  pool: PabloPool,
  baseAssetAmount: bigint,
  quoteAssetAmount: bigint
) {
  assert(
    baseAssetAmount == quoteAssetAmount,
    "base and quote amounts must be equal"
  );
  pool.transactionCount += 1;
  pool.totalLiquidity = Big(pool.totalLiquidity)
    .add(Big(quoteAssetAmount.toString()).mul(2))
    .toString();
  pool.poolAssets[0].totalLiquidity += baseAssetAmount;
  pool.poolAssets[1].totalLiquidity += quoteAssetAmount;
}

describe("PoolCreated Tests", function () {
  it("Should create the Pool, Assets and Transaction correctly", async function () {
    // given
    let pabloPool = new PabloPool();
    let storeMock: Store = mock<Store>();
    when(storeMock.get<PabloPool>(PabloPool, anything())).thenReturn(
      Promise.resolve(pabloPool)
    );
    let ctx = createCtx(storeMock, 1);
    let { owner, event } = createPoolCreatedEvent();

    // when
    await processPoolCreatedEvent(ctx, event);

    // then
    verify(storeMock.save(anyOfClass(PabloPool))).once();
    const [poolArg] = capture(storeMock.save).first();
    assertPool(
      poolArg as unknown as PabloPool,
      ctx.event.id,
      BigInt(1),
      encodeAccount(owner),
      BigInt(1),
      1,
      BigInt(4),
      "0.0",
      "0.0",
      "0.0"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      BigInt(1),
      BigInt(1),
      BigInt(0),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
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
      "0",
      BigInt(1),
      BigInt(0),
      BigInt(4),
      BigInt(0),
      "0.0"
    );
  });
});

describe("Liquidity Added & Removed Tests", function () {
  it("Should add liquidity to the Pool and record Assets and Transaction correctly", async function () {
    // given
    let pabloPool = createZeroPool();
    let storeMock: Store = mock<Store>();
    when(storeMock.get<PabloPool>(PabloPool, anything())).thenReturn(
      Promise.resolve(pabloPool)
    );
    let ctx = createCtx(storeMock, 1);
    let { who, event } = createLiquidityAddedEvent();

    // when
    await processLiquidityAddedEvent(ctx, event);

    // then
    verify(storeMock.save(anyOfClass(PabloPool))).once();
    const [poolArg] = capture(storeMock.save).first();
    assertPool(
      poolArg as unknown as PabloPool,
      ctx.event.id,
      BigInt(1),
      pabloPool.owner,
      BigInt(1),
      2,
      BigInt(4),
      "20000000000000000",
      "0.0",
      "0.0"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      BigInt(1),
      BigInt(1),
      BigInt(10_000 * UNIT),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
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
      "1",
      BigInt(1),
      BigInt(10_000 * UNIT),
      BigInt(4),
      BigInt(10_000 * UNIT),
      "0.0"
    );
  });

  it("Should remove liquidity from the Pool and record Assets and Transaction correctly", async function () {
    // given
    let pabloPool = createZeroPool();
    addLiquidity(
      pabloPool,
      BigInt(10_000_000 * UNIT),
      BigInt(10_000_000 * UNIT)
    );
    let storeMock: Store = mock<Store>();
    when(storeMock.get<PabloPool>(PabloPool, anything())).thenReturn(
      Promise.resolve(pabloPool)
    );
    let ctx = createCtx(storeMock, 1);
    let { who, event } = createLiquidityRemovedEvent();

    // when
    await processLiquidityRemovedEvent(ctx, event);

    // then
    verify(storeMock.save(anyOfClass(PabloPool))).once();
    const [poolArg] = capture(storeMock.save).first();
    assertPool(
      poolArg as unknown as PabloPool,
      ctx.event.id,
      BigInt(1),
      pabloPool.owner,
      BigInt(1),
      3,
      BigInt(4),
      "19980000000000000000",
      "0.0",
      "0.0"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      BigInt(1),
      BigInt(1),
      BigInt(9_990_000 * UNIT),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      BigInt(4),
      BigInt(1),
      BigInt(9_990_000 * UNIT),
      BigInt(0)
    );
    verify(storeMock.save(anyOfClass(PabloTransaction))).once();
    const [txArg] = capture(storeMock.save).last();
    assertTransaction(
      txArg as unknown as PabloTransaction,
      ctx.event.id,
      encodeAccount(who),
      PabloTransactionType.REMOVE_LIQUIDITY,
      "1",
      BigInt(1),
      BigInt(10_000 * UNIT),
      BigInt(4),
      BigInt(10_000 * UNIT),
      "0.0"
    );
  });
});

describe("PoolDeleted Tests", function () {
  it("Should updated the Pool, Assets and Transaction correctly", async function () {
    // given
    let pabloPool = createZeroPool();
    addLiquidity(pabloPool, BigInt(10_000 * UNIT), BigInt(10_000 * UNIT));
    let storeMock: Store = mock<Store>();
    when(storeMock.get<PabloPool>(PabloPool, anything())).thenReturn(
      Promise.resolve(pabloPool)
    );
    let ctx = createCtx(storeMock, 1);
    let { who, event } = createPoolDeletedEvent();

    // when
    await processPoolDeletedEvent(ctx, event);

    // then
    verify(storeMock.save(anyOfClass(PabloPool))).once();
    const [poolArg] = capture(storeMock.save).first();
    assertPool(
      poolArg as unknown as PabloPool,
      ctx.event.id,
      BigInt(1),
      pabloPool.owner,
      BigInt(1),
      3,
      BigInt(4),
      "0.0",
      "0.0",
      "0.0"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      BigInt(1),
      BigInt(1),
      BigInt(0),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
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
      pabloPool.owner,
      PabloTransactionType.DELETE_POOL,
      "1",
      BigInt(1),
      BigInt(10_000 * UNIT),
      BigInt(4),
      BigInt(10_000 * UNIT),
      "0.0"
    );
  });
});

describe("Swapped Tests", function () {
  it("Should record Pool, Assets and Transaction correctly for normal swap", async function () {
    // given
    let pabloPool = createZeroPool();
    addLiquidity(pabloPool, BigInt(10_000 * UNIT), BigInt(10_000 * UNIT));
    let storeMock: Store = mock<Store>();
    when(storeMock.get<PabloPool>(PabloPool, anything())).thenReturn(
      Promise.resolve(pabloPool)
    );
    let ctx = createCtx(storeMock, 1);
    let { who, event } = createSwappedEvent();

    // when
    await processSwappedEvent(ctx, event);

    // then
    verify(storeMock.save(anyOfClass(PabloPool))).once();
    const [poolArg] = capture(storeMock.save).first();
    assertPool(
      poolArg as unknown as PabloPool,
      ctx.event.id,
      BigInt(1),
      pabloPool.owner,
      BigInt(1),
      3,
      BigInt(4),
      "20000000000000000",
      "25000000000000",
      "1000000000000"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      BigInt(1),
      BigInt(1),
      BigInt(9900 * UNIT),
      BigInt(100 * UNIT)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
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
      "0.25",
      BigInt(1),
      BigInt(100 * UNIT),
      BigInt(4),
      BigInt(25 * UNIT),
      "1000000000000"
    );
  });

  it("Should record Pool, Assets and Transaction correctly for reverse swap", async function () {
    // given
    let pabloPool = createZeroPool();
    addLiquidity(pabloPool, BigInt(10_000 * UNIT), BigInt(10_000 * UNIT));
    let storeMock: Store = mock<Store>();
    when(storeMock.get<PabloPool>(PabloPool, anything())).thenReturn(
      Promise.resolve(pabloPool)
    );
    let ctx = createCtx(storeMock, 1);
    let { who, event } = createSwappedEvent(
      BigInt(4),
      BigInt(1),
      BigInt(12 * UNIT),
      BigInt(25 * UNIT),
      {
        fee: BigInt(UNIT),
        lpFee: BigInt(UNIT),
        ownerFee: BigInt(0),
        protocolFee: BigInt(0),
        assetId: BigInt(1),
      }
    );

    // when
    await processSwappedEvent(ctx, event);

    // then
    verify(storeMock.save(anyOfClass(PabloPool))).once();
    const [poolArg] = capture(storeMock.save).first();
    assertPool(
      poolArg as unknown as PabloPool,
      ctx.event.id,
      BigInt(1),
      pabloPool.owner,
      BigInt(1),
      3,
      BigInt(4),
      "20000000000000000",
      "12000000000000",
      "480000000000"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      BigInt(1),
      BigInt(1),
      BigInt(10_025 * UNIT),
      BigInt(25 * UNIT)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      BigInt(4),
      BigInt(1),
      BigInt(9988 * UNIT),
      BigInt(12 * UNIT)
    );
    verify(storeMock.save(anyOfClass(PabloTransaction))).once();
    const [txArg] = capture(storeMock.save).last();
    assertTransaction(
      txArg as unknown as PabloTransaction,
      ctx.event.id,
      encodeAccount(who),
      PabloTransactionType.SWAP,
      "0.48",
      BigInt(4),
      BigInt(12 * UNIT),
      BigInt(1),
      BigInt(25 * UNIT),
      "480000000000"
    );
  });
});
