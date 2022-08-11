import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  BalancesSlashedEvent,
  BalancesTransferEvent,
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
  BondedFinanceOfferCancelledEvent,
  StakingRewardsRewardPoolCreatedEvent,
  StakingRewardsSplitPositionEvent,
  StakingRewardsStakeAmountExtendedEvent,
  StakingRewardsStakedEvent,
  StakingRewardsUnstakedEvent,
} from "./types/events";
import { get, getOrCreate } from "./dbHelper";
import {
  PicassoAccount,
  PicassoTransaction,
  PicassoTransactionType,
} from "./model";
import { encodeAccount } from "./utils";

/**
 * Creates PicassoTransaction
 * @param ctx
 * @param who
 * @param transactionType
 * @param fee
 */
function createTransaction(
  ctx: EventHandlerContext,
  who: PicassoAccount,
  transactionType: PicassoTransactionType,
  fee?: string
): PicassoTransaction {
  return new PicassoTransaction({
    id: randomUUID(),
    eventId: ctx.event.id,
    transactionId: ctx.event.id, // TODO: change
    who,
    transactionType,
    blockNumber: BigInt(ctx.block.height),
    fee: BigInt(fee || 0),
    timestamp: BigInt(new Date().getTime()),
  });
}

function createAccount(
  ctx: EventHandlerContext,
  balance: bigint
): PicassoAccount {
  return new PicassoAccount({
    id: randomUUID(),
    eventId: ctx.event.id,
    transactionId: ctx.event.id, // TODO: change,
    balance,
    // TODO: transactions?
  });
}

interface TransferEvent {
  from: Uint8Array;
  to: Uint8Array;
  amount: bigint;
}

interface SlashedEvent {
  who: Uint8Array;
  amount: bigint;
}

interface RewardPoolCreatedEvent {
  poolId: number;
  owner: Uint8Array;
  endBlock: number;
}

interface StakedEvent {
  poolId: number;
  owner: Uint8Array;
  amount: bigint;
  durationPreset: bigint;
  positionId: bigint;
  keepAlive: boolean;
}

interface StakeAmountExtendedEvent {
  positionId: bigint;
  amount: bigint;
}

interface UnstakedEvent {
  owner: Uint8Array;
  positionId: bigint;
}

interface SplitPositionEvent {
  positions: bigint[];
}

interface NewOfferEvent {
  offerId: bigint;
  beneficiary: Uint8Array;
}

interface NewBondEvent {
  offerId: bigint;
  nbOfBonds: bigint;
}

interface OfferCancelledEvent {
  offerId: bigint;
}

function getTransferEvent(event: BalancesTransferEvent): TransferEvent {
  const { from, to, amount } = event.asV2401 ?? event.asLatest;
  return { from, to, amount };
}

function getSlashedEvent(event: BalancesSlashedEvent): SlashedEvent {
  const { who, amount } = event.asV2401 ?? event.asLatest;
  return { who, amount };
}

function getRewardPoolCreatedEvent(
  event: StakingRewardsRewardPoolCreatedEvent
): RewardPoolCreatedEvent {
  const { poolId, owner, endBlock } = event.asV2401 ?? event.asLatest;
  return { poolId, owner, endBlock };
}

function getStakedEvent(event: StakingRewardsStakedEvent): StakedEvent {
  const { poolId, owner, amount, durationPreset, positionId, keepAlive } =
    event.asV2401 ?? event.asLatest;
  return { poolId, owner, amount, durationPreset, positionId, keepAlive };
}

function getStakeAmountExtendedEvent(
  event: StakingRewardsStakeAmountExtendedEvent
): StakeAmountExtendedEvent {
  const { positionId, amount } = event.asV2401 ?? event.asLatest;
  return { positionId, amount };
}

function getUnstakedEvent(event: StakingRewardsUnstakedEvent): UnstakedEvent {
  const { positionId, owner } = event.asV2401 ?? event.asLatest;
  return { positionId, owner };
}

function getSplitPositionEvent(
  event: StakingRewardsSplitPositionEvent
): SplitPositionEvent {
  const { positions } = event.asV2401 ?? event.asLatest;
  return { positions };
}

function getNewOfferEvent(event: BondedFinanceNewOfferEvent): NewOfferEvent {
  const { offerId, beneficiary } = event.asV2401 ?? event.asLatest;

  return { offerId, beneficiary };
}

function getNewBondEvent(event: BondedFinanceNewBondEvent): NewBondEvent {
  const { offerId, nbOfBonds } = event.asV2401 ?? event.asLatest;
  return { offerId, nbOfBonds };
}

function getOfferCancelledEvent(
  event: BondedFinanceOfferCancelledEvent
): OfferCancelledEvent {
  const { offerId } = event.asV2401 ?? event.asLatest;
  return { offerId };
}

export async function processTransferEvent(ctx: EventHandlerContext) {
  console.log("Process transfer");
  const event = new BalancesTransferEvent(ctx);
  const transferEvent = getTransferEvent(event);
  const from = encodeAccount(transferEvent.from);
  const to = encodeAccount(transferEvent.from);
  const tip = ctx.extrinsic?.tip || 0n;
  const { amount } = transferEvent;

  const accountFrom = await getOrCreate(ctx.store, PicassoAccount, from);
  const accountTo = await getOrCreate(ctx.store, PicassoAccount, to);

  const txId = randomUUID();

  // Update event id
  accountFrom.eventId = ctx.event.id;
  accountTo.eventId = ctx.event.id;
  accountFrom.transactionId = txId;
  accountTo.transactionId = txId;

  // Update balance
  accountFrom.balance =
    BigInt(accountFrom.balance || 0n) - BigInt(amount) - BigInt(tip);
  accountTo.balance = BigInt(accountTo.balance || 0n) + BigInt(amount);

  // TODO: get correct initial balance

  // Create transaction
  const tx = createTransaction(
    ctx,
    accountFrom,
    PicassoTransactionType.BALANCES_TRANSFER
  ); // TODO: fee?

  // TODO: what to do with the transaction in terms of the `accountTo`?

  tx.id = txId;
  tx.eventId = ctx.event.id;

  await ctx.store.save(accountFrom);
  await ctx.store.save(accountTo);
  await ctx.store.save(tx);
}

export async function processSlashedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `slashed`");
  const event = new BalancesSlashedEvent(ctx);
  const slashedEvent = getSlashedEvent(event);
  const who = encodeAccount(slashedEvent.who);
  const tip = ctx.extrinsic?.tip || 0n;
  const { amount } = slashedEvent;

  const account = await getOrCreate(ctx.store, PicassoAccount, who);

  const txId = randomUUID();

  // Update event id
  account.eventId = ctx.event.id;
  account.transactionId = txId;

  // Create transaction
  const tx = createTransaction(
    ctx,
    account,
    PicassoTransactionType.BALANCES_SLASHED
  ); // TODO: fee?

  tx.id = txId;
  tx.eventId = ctx.event.id;

  await ctx.store.save(account);
  await ctx.store.save(tx);
  console.log("Finish processing `slashed`");
}

export async function processRewardPoolCreatedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `reward pool created`");
  const event = new StakingRewardsRewardPoolCreatedEvent(ctx);
  const rewardPoolCreatedEvent = getRewardPoolCreatedEvent(event);
  const owner = encodeAccount(rewardPoolCreatedEvent.owner);
  const tip = ctx.extrinsic?.tip || 0n;
  const { poolId, endBlock } = rewardPoolCreatedEvent;

  const account = await getOrCreate(ctx.store, PicassoAccount, owner);

  const txId = randomUUID();

  // Update event id
  account.eventId = ctx.event.id;
  account.transactionId = txId;

  // Create transaction
  const tx = createTransaction(
    ctx,
    account,
    PicassoTransactionType.STAKING_REWARDS_REWARD_POOL_CREATED
  ); // TODO: fee?

  tx.id = txId;
  tx.eventId = ctx.event.id;

  await ctx.store.save(account);
  await ctx.store.save(tx);
  console.log("Finish processing `reward pool created`");
}

// function getRewardPoolCreatedEvent(
//   event: StakingRewardsRewardPoolCreatedEvent
// ): RewardPoolCreatedEvent {
//   const { poolId, owner, endBlock } = event.asV2401 ?? event.asLatest;
//   return { poolId, owner, endBlock };
// }

// export function createPoolAssetId(
//   eventId: string,
//   poolId: bigint,
//   assetId: bigint
// ): string {
//   return `${eventId}-${poolId}-${assetId}`;
// }
//
// interface LiquidityAddedEvent {
//   who: Uint8Array;
//   poolId: bigint;
//   baseAmount: bigint;
//   quoteAmount: bigint;
//   mintedLp: bigint;
// }
//
// function getLiquidityAddedEvent(
//   event: PabloLiquidityAddedEvent
// ): LiquidityAddedEvent {
//   const { who, poolId, baseAmount, quoteAmount, mintedLp } =
//     event.asV2401 ?? event.asLatest;
//   return { who, poolId, baseAmount, quoteAmount, mintedLp };
// }
//
// export async function processLiquidityAddedEvent(
//   ctx: EventHandlerContext,
//   event: PabloLiquidityAddedEvent
// ) {
//   console.debug("processing LiquidityAddedEvent", ctx.event.id);
//   const liquidityAddedEvt = getLiquidityAddedEvent(event);
//   const who = encodeAccount(liquidityAddedEvt.who);
//   const pool = await getLatestPoolByPoolId(ctx.store, liquidityAddedEvt.poolId);
//   // only set values if the owner was missing, i.e a new pool
//   if (pool != undefined) {
//     const timestamp = BigInt(new Date().getTime());
//     pool.id = ctx.event.id;
//     pool.eventId = ctx.event.id;
//     pool.transactionCount += 1;
//     pool.totalLiquidity = Big(pool.totalLiquidity)
//       // multiplying by 2 to account for base amount being added
//       .add(Big(liquidityAddedEvt.quoteAmount.toString()).mul(2))
//       .toString();
//     pool.calculatedTimestamp = timestamp;
//     pool.blockNumber = BigInt(ctx.block.height);
//
//     // find baseAsset: Following is only valid for dual asset pools
//     const baseAsset = pool.poolAssets.find(
//       (asset) => asset.assetId != pool.quoteAssetId
//     );
//     if (baseAsset == undefined) {
//       throw new Error("baseAsset not found");
//     }
//     baseAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       baseAsset.assetId
//     );
//     baseAsset.pool = pool;
//     baseAsset.totalLiquidity += liquidityAddedEvt.baseAmount;
//     baseAsset.calculatedTimestamp = timestamp;
//     baseAsset.blockNumber = BigInt(ctx.block.height);
//     // find quoteAsset
//     const quoteAsset = pool.poolAssets.find(
//       (asset) => asset.assetId == pool.quoteAssetId
//     );
//     if (quoteAsset == undefined) {
//       throw new Error("quoteAsset not found");
//     }
//     quoteAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       quoteAsset.assetId
//     );
//     quoteAsset.pool = pool;
//     quoteAsset.totalLiquidity += liquidityAddedEvt.quoteAmount;
//     quoteAsset.calculatedTimestamp = timestamp;
//     quoteAsset.blockNumber = BigInt(ctx.block.height);
//
//     let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
//     if (tx != undefined) {
//       throw new Error("Unexpected transaction in db");
//     }
//     tx = createTransaction(
//       ctx,
//       pool,
//       who,
//       PabloTransactionType.ADD_LIQUIDITY,
//       Big(liquidityAddedEvt.baseAmount.toString())
//         .div(Big(liquidityAddedEvt.quoteAmount.toString()))
//         .toString(),
//       BigInt(baseAsset.assetId),
//       liquidityAddedEvt.baseAmount,
//       pool.quoteAssetId,
//       liquidityAddedEvt.quoteAmount
//     );
//
//     await ctx.store.save(pool);
//     await ctx.store.save(baseAsset);
//     await ctx.store.save(quoteAsset);
//     await ctx.store.save(tx);
//   } else {
//     throw new Error("Pool not found");
//   }
// }
//
// interface LiquidityRemovedEvent {
//   who: Uint8Array;
//   poolId: bigint;
//   baseAmount: bigint;
//   quoteAmount: bigint;
//   totalIssuance: bigint;
// }
//
// function getLiquidityRemovedEvent(
//   event: PabloLiquidityRemovedEvent
// ): LiquidityRemovedEvent {
//   const { who, poolId, baseAmount, quoteAmount, totalIssuance } =
//     event.asV2401 ?? event.asLatest;
//   return { who, poolId, baseAmount, quoteAmount, totalIssuance };
// }
//
// export async function processLiquidityRemovedEvent(
//   ctx: EventHandlerContext,
//   event: PabloLiquidityRemovedEvent
// ) {
//   console.debug("processing LiquidityAddedEvent", ctx.event.id);
//   const liquidityRemovedEvt = getLiquidityRemovedEvent(event);
//   const who = encodeAccount(liquidityRemovedEvt.who);
//   const pool = await getLatestPoolByPoolId(
//     ctx.store,
//     liquidityRemovedEvt.poolId
//   );
//   // only set values if the owner was missing, i.e a new pool
//   if (pool != undefined) {
//     const timestamp = BigInt(new Date().getTime());
//     pool.id = ctx.event.id;
//     pool.eventId = ctx.event.id;
//     pool.transactionCount += 1;
//     pool.totalLiquidity = Big(pool.totalLiquidity)
//       // multiplying by 2 to account for base amount being removed
//       .sub(Big(liquidityRemovedEvt.quoteAmount.toString()).mul(2))
//       .toString();
//     pool.calculatedTimestamp = timestamp;
//     pool.blockNumber = BigInt(ctx.block.height);
//
//     // find baseAsset: Following is only valid for dual asset pools
//     const baseAsset = pool.poolAssets.find(
//       (asset) => asset.assetId != pool.quoteAssetId
//     );
//     if (baseAsset == undefined) {
//       throw new Error("baseAsset not found");
//     }
//     baseAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       baseAsset.assetId
//     );
//     baseAsset.pool = pool;
//     baseAsset.totalLiquidity -= liquidityRemovedEvt.baseAmount;
//     baseAsset.calculatedTimestamp = timestamp;
//     baseAsset.blockNumber = BigInt(ctx.block.height);
//     // find quoteAsset
//     const quoteAsset = pool.poolAssets.find(
//       (asset) => asset.assetId == pool.quoteAssetId
//     );
//     if (quoteAsset == undefined) {
//       throw new Error("quoteAsset not found");
//     }
//     quoteAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       quoteAsset.assetId
//     );
//     quoteAsset.pool = pool;
//     quoteAsset.totalLiquidity -= liquidityRemovedEvt.quoteAmount;
//     quoteAsset.calculatedTimestamp = timestamp;
//     quoteAsset.blockNumber = BigInt(ctx.block.height);
//
//     let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
//     if (tx != undefined) {
//       throw new Error("Unexpected transaction in db");
//     }
//     tx = createTransaction(
//       ctx,
//       pool,
//       who,
//       PabloTransactionType.REMOVE_LIQUIDITY,
//       Big(liquidityRemovedEvt.baseAmount.toString())
//         .div(Big(liquidityRemovedEvt.quoteAmount.toString()))
//         .toString(),
//       BigInt(baseAsset.assetId),
//       liquidityRemovedEvt.baseAmount,
//       pool.quoteAssetId,
//       liquidityRemovedEvt.quoteAmount
//     );
//
//     await ctx.store.save(pool);
//     await ctx.store.save(baseAsset);
//     await ctx.store.save(quoteAsset);
//     await ctx.store.save(tx);
//   } else {
//     throw new Error("Pool not found");
//   }
// }
//
// interface SwappedEvent {
//   poolId: bigint;
//   who: Uint8Array;
//   baseAsset: bigint;
//   quoteAsset: bigint;
//   baseAmount: bigint;
//   quoteAmount: bigint;
//   fee: Fee;
// }
//
// function getSwappedEvent(event: PabloSwappedEvent): SwappedEvent {
//   const { poolId, who, baseAsset, quoteAsset, baseAmount, quoteAmount, fee } =
//     event.asV2401 ?? event.asLatest;
//   return { poolId, who, baseAsset, quoteAsset, baseAmount, quoteAmount, fee };
// }
//
// export async function processSwappedEvent(
//   ctx: EventHandlerContext,
//   event: PabloSwappedEvent
// ) {
//   console.debug("processing SwappedEvent", ctx.event.id);
//   const swappedEvt = getSwappedEvent(event);
//   const who = encodeAccount(swappedEvt.who);
//   const pool = await getLatestPoolByPoolId(ctx.store, swappedEvt.poolId);
//   // only set values if the owner was missing, i.e a new pool
//   if (pool != undefined) {
//     const isReverse: boolean = pool.quoteAssetId != swappedEvt.quoteAsset;
//     const timestamp = BigInt(new Date().getTime());
//     pool.id = ctx.event.id;
//     pool.eventId = ctx.event.id;
//     pool.transactionCount += 1;
//     pool.calculatedTimestamp = timestamp;
//     pool.blockNumber = BigInt(ctx.block.height);
//     // find baseAsset: Following is only valid for dual asset pools
//     const baseAsset = pool.poolAssets.find(
//       (asset) => asset.assetId != pool.quoteAssetId
//     );
//     if (baseAsset == undefined) {
//       throw new Error("baseAsset not found");
//     }
//     // find quoteAsset
//     const quoteAsset = pool.poolAssets.find(
//       (asset) => asset.assetId == pool.quoteAssetId
//     );
//     if (quoteAsset == undefined) {
//       throw new Error("quoteAsset not found");
//     }
//     const feesLeavingPool = swappedEvt.fee.fee - swappedEvt.fee.lpFee;
//     const spotPrice = isReverse
//       ? Big(swappedEvt.baseAmount.toString()).div(
//           Big(swappedEvt.quoteAmount.toString())
//         )
//       : Big(swappedEvt.quoteAmount.toString()).div(
//           Big(swappedEvt.baseAmount.toString())
//         );
//     if (isReverse) {
//       console.debug("Reverse swap");
//       // volume
//       pool.totalVolume = Big(pool.totalVolume)
//         .add(Big(swappedEvt.baseAmount.toString()))
//         .toString();
//       baseAsset.totalVolume += swappedEvt.quoteAmount;
//       quoteAsset.totalVolume += swappedEvt.baseAmount;
//
//       // for reverse exchange "default quote" (included as the base amount in the evt) amount leaves the pool
//       baseAsset.totalLiquidity += swappedEvt.quoteAmount;
//       quoteAsset.totalLiquidity -= swappedEvt.baseAmount;
//       quoteAsset.totalLiquidity -= feesLeavingPool;
//     } else {
//       console.debug("Normal swap");
//       // volume
//       pool.totalVolume = Big(pool.totalVolume)
//         .add(Big(swappedEvt.quoteAmount.toString()))
//         .toString();
//       baseAsset.totalVolume += swappedEvt.baseAmount;
//       quoteAsset.totalVolume += swappedEvt.quoteAmount;
//
//       // for normal exchange "default quote" amount gets into the pool
//       baseAsset.totalLiquidity -= swappedEvt.baseAmount;
//       baseAsset.totalLiquidity -= feesLeavingPool;
//       quoteAsset.totalLiquidity += swappedEvt.quoteAmount;
//     }
//     // fee and liquidity
//     pool.totalLiquidity = Big(pool.totalLiquidity)
//       .sub(
//         calculateFeeInQuoteAsset(
//           spotPrice,
//           quoteAsset.assetId,
//           swappedEvt.fee.assetId,
//           feesLeavingPool
//         )
//       )
//       .toString();
//     const fee = calculateFeeInQuoteAsset(
//       spotPrice,
//       quoteAsset.assetId,
//       swappedEvt.fee.assetId,
//       swappedEvt.fee.fee
//     );
//     pool.totalFees = Big(pool.totalFees).add(fee).toString();
//     baseAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       baseAsset.assetId
//     );
//     baseAsset.pool = pool;
//     baseAsset.calculatedTimestamp = timestamp;
//     baseAsset.blockNumber = BigInt(ctx.block.height);
//     quoteAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       quoteAsset.assetId
//     );
//     quoteAsset.pool = pool;
//     quoteAsset.calculatedTimestamp = timestamp;
//     quoteAsset.blockNumber = BigInt(ctx.block.height);
//
//     let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
//     if (tx != undefined) {
//       throw new Error("Unexpected transaction in db");
//     }
//     tx = createTransaction(
//       ctx,
//       pool,
//       who,
//       PabloTransactionType.SWAP,
//       spotPrice.toString(),
//       swappedEvt.baseAsset,
//       swappedEvt.baseAmount,
//       swappedEvt.quoteAsset,
//       swappedEvt.quoteAmount,
//       fee.toString()
//     );
//
//     await ctx.store.save(pool);
//     await ctx.store.save(baseAsset);
//     await ctx.store.save(quoteAsset);
//     await ctx.store.save(tx);
//   } else {
//     throw new Error("Pool not found");
//   }
// }
//
// interface PoolDeletedEvent {
//   poolId: bigint;
//   baseAmount: bigint;
//   quoteAmount: bigint;
// }
//
// function getPoolDeletedEvent(event: PabloPoolDeletedEvent): PoolDeletedEvent {
//   const { poolId, baseAmount, quoteAmount } = event.asV2401 ?? event.asLatest;
//   return { poolId, baseAmount, quoteAmount };
// }
//
// export async function processPoolDeletedEvent(
//   ctx: EventHandlerContext,
//   event: PabloPoolDeletedEvent
// ) {
//   console.debug("processing LiquidityAddedEvent", ctx.event.id);
//   const poolDeletedEvent = getPoolDeletedEvent(event);
//   const pool = await getLatestPoolByPoolId(ctx.store, poolDeletedEvent.poolId);
//   // only set values if the owner was missing, i.e a new pool
//   if (pool != undefined) {
//     const who = pool.owner;
//     const timestamp = BigInt(new Date().getTime());
//     pool.id = ctx.event.id;
//     pool.eventId = ctx.event.id;
//     pool.transactionCount += 1;
//     pool.totalLiquidity = "0.0";
//     pool.calculatedTimestamp = timestamp;
//     pool.blockNumber = BigInt(ctx.block.height);
//
//     // find baseAsset: Following is only valid for dual asset pools
//     const baseAsset = pool.poolAssets.find(
//       (asset) => asset.assetId != pool.quoteAssetId
//     );
//     if (baseAsset == undefined) {
//       throw new Error("baseAsset not found");
//     }
//     baseAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       baseAsset.assetId
//     );
//     baseAsset.pool = pool;
//     baseAsset.totalLiquidity -= poolDeletedEvent.baseAmount;
//     baseAsset.calculatedTimestamp = timestamp;
//     baseAsset.blockNumber = BigInt(ctx.block.height);
//     // find quoteAsset
//     const quoteAsset = pool.poolAssets.find(
//       (asset) => asset.assetId == pool.quoteAssetId
//     );
//     if (quoteAsset == undefined) {
//       throw new Error("quoteAsset not found");
//     }
//     quoteAsset.id = createPoolAssetId(
//       ctx.event.id,
//       pool.poolId,
//       quoteAsset.assetId
//     );
//     quoteAsset.pool = pool;
//     quoteAsset.totalLiquidity -= poolDeletedEvent.quoteAmount;
//     quoteAsset.calculatedTimestamp = timestamp;
//     quoteAsset.blockNumber = BigInt(ctx.block.height);
//
//     let tx = await get(ctx.store, PabloTransaction, ctx.event.id);
//     if (tx != undefined) {
//       throw new Error("Unexpected transaction in db");
//     }
//     tx = createTransaction(
//       ctx,
//       pool,
//       who,
//       PabloTransactionType.DELETE_POOL,
//       Big(poolDeletedEvent.baseAmount.toString())
//         .div(Big(poolDeletedEvent.quoteAmount.toString()))
//         .toString(),
//       BigInt(baseAsset.assetId),
//       poolDeletedEvent.baseAmount,
//       pool.quoteAssetId,
//       poolDeletedEvent.quoteAmount
//     );
//
//     await ctx.store.save(pool);
//     await ctx.store.save(baseAsset);
//     await ctx.store.save(quoteAsset);
//     await ctx.store.save(tx);
//   } else {
//     throw new Error("Pool not found");
//   }
// }
//
// function calculateFeeInQuoteAsset(
//   spotPrice: Big,
//   quoteAsset: bigint,
//   feeAsset: bigint,
//   fee: bigint
// ): Big {
//   // calculate the quote amount based on the exchange rate if the fees are in the base asset
//   return feeAsset == quoteAsset
//     ? Big(fee.toString())
//     : spotPrice.mul(fee.toString());
// }
