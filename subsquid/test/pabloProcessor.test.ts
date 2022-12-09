import { assert, expect } from "chai";
import {
  anyOfClass,
  anything,
  capture,
  instance,
  mock,
  verify,
  when,
} from "ts-mockito";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import Big from "big.js";
import { PabloPool, PabloPoolAsset, Event, EventType } from "../src/model";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent,
  processPoolDeletedEvent,
  processSwappedEvent,
} from "../src/processors/pablo";
import {
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloPoolDeletedEvent,
  PabloSwappedEvent,
} from "../src/types/events";
import { Fee } from "../src/types/v2402";
import { createAccount, createCtx, encodeAccount } from "../src/utils";

const UNIT = 1_000_000_000_000;

function assertPool(
  poolArg: PabloPool,
  eventId: string,
  poolId: bigint,
  owner: string,
  blockNumber: bigint,
  transactionCount: number,
  baseAssetId: string,
  quoteAssetId: string,
  totalLiquidity: string,
  totalVolume: string,
  totalFees: string
) {
  expect(poolArg.eventId).eq(eventId);
  expect(poolArg.poolId).eq(poolId);
  expect(poolArg.owner).eq(owner);
  expect(poolArg.blockNumber).eq(blockNumber);
  expect(poolArg.transactionCount).eq(transactionCount);
  // expect(poolArg.baseAssetId).eq(baseAssetId);
  expect(poolArg.quoteAssetId).eq(quoteAssetId);
  expect(poolArg.totalLiquidity).eq(totalLiquidity);
  expect(poolArg.totalVolume).eq(totalVolume);
  expect(poolArg.totalFees).eq(totalFees);
}

function assertEvent(
  event: Event,
  eventId: string,
  accountId: string,
  eventType: EventType,
  spotPrice: string,
  baseAssetId: string,
  baseAssetAmount: bigint,
  quoteAssetId: string,
  quoteAssetAmount: bigint,
  fee: string
) {
  expect(event.id).eq(eventId);
  expect(event.eventType).eq(eventType);
  expect(event.accountId).eq(accountId);
  const pabloTransaction = event.pabloTransaction!;
  expect(pabloTransaction.spotPrice).eq(spotPrice);
  expect(pabloTransaction.baseAssetId).eq(baseAssetId);
  expect(pabloTransaction.baseAssetAmount).eq(baseAssetAmount);
  expect(pabloTransaction.quoteAssetId).eq(quoteAssetId);
  expect(pabloTransaction.quoteAssetAmount).eq(quoteAssetAmount);
  expect(pabloTransaction.fee).eq(fee);
}

function assertAsset(
  asset: PabloPoolAsset,
  id: string,
  assetId: string,
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
  when(eventMock.asV10003).thenReturn(evt);
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
  when(eventMock.asV10003).thenReturn(evt);
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
  when(eventMock.asV10003).thenReturn(evt);
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
  when(eventMock.asV10003).thenReturn(evt);
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
  when(eventMock.asV10003).thenReturn(evt);
  let event = instance(eventMock);
  return { who, event };
}

function createZeroAsset(id: string, assetId: string) {
  let asset = new PabloPoolAsset();
  asset.blockNumber = BigInt(1);
  asset.totalVolume = BigInt(0);
  asset.totalLiquidity = BigInt(0);
  asset.assetId = assetId;
  asset.id = id;
  return asset;
}

function createZeroPool() {
  let baseAsset = createZeroAsset("1-1", "1");
  let quoteAsset = createZeroAsset("1-4", "4");
  let pabloPool = new PabloPool();
  pabloPool.id = randomUUID();
  pabloPool.poolId = BigInt(1);
  pabloPool.owner = encodeAccount(createAccount());
  pabloPool.quoteAssetId = "4";
  pabloPool.totalLiquidity = "0.0";
  pabloPool.totalVolume = "0.0";
  pabloPool.totalFees = "0.0";
  pabloPool.transactionCount = 1;
  pabloPool.lpIssued = BigInt(0);
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

describe.skip("PoolCreated Tests", function () {
  it("Should create the Pool, Assets and Event correctly", async function () {
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
      "1",
      "4",
      "0.0",
      "0.0",
      "0.0"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      "1",
      BigInt(1),
      BigInt(0),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      "4",
      BigInt(1),
      BigInt(0),
      BigInt(0)
    );
    verify(storeMock.save(anyOfClass(Event))).once();
    const [txArg] = capture(storeMock.save).last();
    assertEvent(
      txArg as unknown as Event,
      ctx.event.id,
      encodeAccount(owner),
      EventType.CREATE_POOL,
      "0",
      "1",
      BigInt(0),
      "4",
      BigInt(0),
      "0.0"
    );
  });
});

describe.skip("Liquidity Added & Removed Tests", function () {
  it("Should add liquidity to the Pool and record Assets and Event correctly", async function () {
    // given
    let pabloPool = createZeroPool();
    let storeMock: Store = mock<Store>();
    // when(storeMock.get<PabloPool>(PabloPool, anything())).thenReturn(
    //   Promise.resolve(pabloPool)
    // );
    when(storeMock.get<PabloPool>(PabloPool, anything())).thenCall((x) => {
      return Promise.resolve(pabloPool);
    });
    let ctx = createCtx(storeMock, 1);
    let { who, event } = createLiquidityAddedEvent();

    // when
    await processLiquidityAddedEvent(ctx, event);

    // then
    verify(storeMock.save(anyOfClass(PabloPool))).once();
    const [poolArg] = capture(storeMock.save).first();
    // TODO: refactor Pablo tests
    // assertPool(
    //   poolArg as unknown as PabloPool,
    //   ctx.event.id,
    //   BigInt(1),
    //   pabloPool.owner,
    //   BigInt(1),
    //   2,
    //   "4",
    //   "4",
    //   "20000000000000000",
    //   "0.0",
    //   "0.0"
    // );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      "1",
      BigInt(1),
      BigInt(10_000 * UNIT),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      "4",
      BigInt(1),
      BigInt(10_000 * UNIT),
      BigInt(0)
    );
    verify(storeMock.save(anyOfClass(Event))).once();
    const [txArg] = capture(storeMock.save).last();
    assertEvent(
      txArg as unknown as Event,
      ctx.event.id,
      encodeAccount(who),
      EventType.ADD_LIQUIDITY,
      "1",
      "1",
      BigInt(10_000 * UNIT),
      "4",
      BigInt(10_000 * UNIT),
      "0.0"
    );
  });

  it("Should remove liquidity from the Pool and record Assets and Event correctly", async function () {
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
      "4",
      "4",
      "19980000000000000000",
      "0.0",
      "0.0"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      "1",
      BigInt(1),
      BigInt(9_990_000 * UNIT),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      "4",
      BigInt(1),
      BigInt(9_990_000 * UNIT),
      BigInt(0)
    );
    verify(storeMock.save(anyOfClass(Event))).once();
    const [txArg] = capture(storeMock.save).last();
    assertEvent(
      txArg as unknown as Event,
      ctx.event.id,
      encodeAccount(who),
      EventType.REMOVE_LIQUIDITY,
      "1",
      "1",
      BigInt(10_000 * UNIT),
      "4",
      BigInt(10_000 * UNIT),
      "0.0"
    );
  });
});

describe.skip("PoolDeleted Tests", function () {
  it("Should updated the Pool, Assets and Event correctly", async function () {
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
      "4",
      "4",
      "0.0",
      "0.0",
      "0.0"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      "1",
      BigInt(1),
      BigInt(0),
      BigInt(0)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      "4",
      BigInt(1),
      BigInt(0),
      BigInt(0)
    );
    verify(storeMock.save(anyOfClass(Event))).once();
    const [txArg] = capture(storeMock.save).last();
    assertEvent(
      txArg as unknown as Event,
      ctx.event.id,
      pabloPool.owner,
      EventType.DELETE_POOL,
      "1",
      "1",
      BigInt(10_000 * UNIT),
      "4",
      BigInt(10_000 * UNIT),
      "0.0"
    );
  });
});

describe.skip("Swapped Tests", function () {
  it("Should record Pool, Assets and Event correctly for normal swap", async function () {
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
      "4",
      "4",
      "20000000000000000",
      "25000000000000",
      "1000000000000"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      "1",
      BigInt(1),
      BigInt(9900 * UNIT),
      BigInt(100 * UNIT)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      "4",
      BigInt(1),
      BigInt(10_025 * UNIT),
      BigInt(25 * UNIT)
    );
    verify(storeMock.save(anyOfClass(Event))).once();
    const [txArg] = capture(storeMock.save).last();
    assertEvent(
      txArg as unknown as Event,
      ctx.event.id,
      encodeAccount(who),
      EventType.SWAP,
      "0.25",
      "1",
      BigInt(100 * UNIT),
      "4",
      BigInt(25 * UNIT),
      "1000000000000"
    );
  });

  it("Should record Pool, Assets and Event correctly for reverse swap", async function () {
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
      "4",
      "4",
      "20000000000000000",
      "12000000000000",
      "480000000000"
    );
    verify(storeMock.save(anyOfClass(PabloPoolAsset))).twice();
    const [assetOneArg] = capture(storeMock.save).second();
    assertAsset(
      assetOneArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-1",
      "1",
      BigInt(1),
      BigInt(10_025 * UNIT),
      BigInt(25 * UNIT)
    );
    const [assetTwoArg] = capture(storeMock.save).byCallIndex(2);
    assertAsset(
      assetTwoArg as unknown as PabloPoolAsset,
      ctx.event.id + "-" + "1-4",
      "4",
      BigInt(1),
      BigInt(9988 * UNIT),
      BigInt(12 * UNIT)
    );
    verify(storeMock.save(anyOfClass(Event))).once();
    const [txArg] = capture(storeMock.save).last();
    assertEvent(
      txArg as unknown as Event,
      ctx.event.id,
      encodeAccount(who),
      EventType.SWAP,
      "0.48",
      "4",
      BigInt(12 * UNIT),
      "1",
      BigInt(25 * UNIT),
      "480000000000"
    );
  });
});
