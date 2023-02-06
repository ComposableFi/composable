import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import {
  StakingRewardsRewardPoolCreatedEvent,
  StakingRewardsRewardPoolUpdatedEvent,
  StakingRewardsSplitPositionEvent,
  StakingRewardsStakeAmountExtendedEvent,
  StakingRewardsStakedEvent,
  StakingRewardsUnstakedEvent
} from "../types/events";
import { RewardPoolConfiguration } from "../types/v10005";
import { saveAccountAndEvent, storeHistoricalLockedValue } from "../dbHelper";
import { Event, EventType, LockedSource, Reward, RewardPool, RewardRatePeriod, StakingPosition } from "../model";
import { encodeAccount } from "../utils";
import * as v10005 from "subsquid/types/v10005";

interface RewardPoolCreatedEvent {
  poolId: bigint;
  owner: Uint8Array;
  poolConfig: RewardPoolConfiguration;
}

interface StakedEvent {
  poolId: bigint;
  owner: Uint8Array;
  amount: bigint;
  durationPreset: bigint;
  fnftCollectionId: bigint;
  fnftInstanceId: bigint;
  rewardMultiplier: bigint;
  keepAlive: boolean;
}

interface UnstakedEvent {
  owner: Uint8Array;
  fnftCollectionId: bigint;
  fnftInstanceId: bigint;
  slash?: bigint;
}

interface StakeAmountExtendedEvent {
  amount: bigint;
  fnftCollectionId: bigint;
  fnftInstanceId: bigint;
}

interface SplitPositionEvent {
  positions: [bigint, bigint, bigint][]; // [collectionId, instanceId, balance]
}

interface RewardPoolUpdatedEvent {
  poolId: bigint;
  rewardUpdates: [bigint, v10005.RewardUpdate][];
}

function getRewardPoolCreatedEvent(event: StakingRewardsRewardPoolCreatedEvent): RewardPoolCreatedEvent {
  const { poolId, owner, poolConfig } = event.asV10005;
  return { poolId, owner, poolConfig };
}

function getStakedEvent(event: StakingRewardsStakedEvent): StakedEvent {
  const { poolId, owner, amount, durationPreset, fnftCollectionId, fnftInstanceId, rewardMultiplier, keepAlive } =
    event.asV10005;
  return {
    poolId,
    owner,
    amount,
    durationPreset,
    fnftCollectionId,
    fnftInstanceId,
    rewardMultiplier,
    keepAlive
  };
}

function getUnstakedEvent(event: StakingRewardsUnstakedEvent): UnstakedEvent {
  const { owner, fnftCollectionId, fnftInstanceId, slash } = event.asV10005;
  return { owner, fnftCollectionId, fnftInstanceId, slash };
}

function getStakeAmountExtendedEvent(event: StakingRewardsStakeAmountExtendedEvent): StakeAmountExtendedEvent {
  const { fnftCollectionId, fnftInstanceId, amount } = event.asV10005;
  return { fnftCollectionId, fnftInstanceId, amount };
}

function getSplitPositionEvent(event: StakingRewardsSplitPositionEvent): SplitPositionEvent {
  const { positions } = event.asV10005;
  return { positions };
}

function getRewardPoolUpdatedEvent(event: StakingRewardsRewardPoolUpdatedEvent): RewardPoolUpdatedEvent {
  const { poolId, rewardUpdates } = event.asV10005;
  return { poolId, rewardUpdates };
}

export function createRewardPool(ctx: EventHandlerContext<Store>, event: Event, poolId: bigint): RewardPool {
  return new RewardPool({
    id: poolId.toString(),
    event,
    // Asset ID is used as pool ID on the pallet
    assetId: poolId.toString(),
    blockId: ctx.block.hash
  });
}

/**
 * Create new StakingPosition.
 * @param rewardPool
 * @param fnftCollectionId
 * @param fnftInstanceId
 * @param assetId
 * @param owner
 * @param amount
 * @param duration
 * @param rewardMultiplier
 * @param event
 * @param startTimestamp
 */
export function createStakingPosition(
  rewardPool: RewardPool,
  fnftCollectionId: bigint,
  fnftInstanceId: bigint,
  assetId: string,
  owner: string,
  amount: bigint,
  duration: bigint,
  rewardMultiplier: bigint,
  event: Event,
  startTimestamp: bigint
): StakingPosition {
  return new StakingPosition({
    id: randomUUID(),
    event,
    fnftCollectionId: fnftCollectionId.toString(),
    fnftInstanceId: fnftInstanceId.toString(),
    rewardPool,
    owner,
    amount,
    startTimestamp,
    duration,
    endTimestamp: BigInt(startTimestamp + BigInt(duration * 1_000n)),
    rewardMultiplier,
    assetId,
    source: LockedSource.StakingRewards,
    removed: false
  });
}

/**
 * Split StakingPosition in 2.
 * Updates existing position in place, and returns new additional position.
 * @param position
 * @param oldAmount
 * @param newAmount
 * @param newFnftInstanceId
 * @param event
 */
export function splitStakingPosition(
  position: StakingPosition,
  oldAmount: bigint,
  newAmount: bigint,
  newFnftInstanceId: bigint,
  event: Event
): StakingPosition {
  position.amount = oldAmount;

  return new StakingPosition({
    id: randomUUID(),
    event,
    fnftCollectionId: position.fnftCollectionId,
    fnftInstanceId: newFnftInstanceId.toString(),
    owner: position.owner,
    amount: newAmount,
    startTimestamp: position.startTimestamp,
    duration: position.duration,
    endTimestamp: position.endTimestamp,
    rewardMultiplier: position.rewardMultiplier,
    assetId: position.assetId,
    source: LockedSource.StakingRewards,
    rewardPool: position.rewardPool,
    removed: false
  });
}

/**
 * Process `StakingRewards.RewardPoolCreated` event.
 *  - Create reward pool.
 *  - Update account and store event.
 * @param ctx
 */
export async function processRewardPoolCreatedEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Processing `StakingRewards.RewardPoolCreated`");
  const evt = getRewardPoolCreatedEvent(new StakingRewardsRewardPoolCreatedEvent(ctx));
  const owner = encodeAccount(evt.owner);

  const { poolId, poolConfig } = evt;

  const { event } = await saveAccountAndEvent(ctx, EventType.STAKING_REWARDS_REWARD_POOL_CREATED, owner);

  const rewardPool = createRewardPool(ctx, event, poolId);

  await ctx.store.save(rewardPool);

  for (const [rewardAssetId, config] of poolConfig.rewardConfigs) {
    const reward = new Reward({
      id: rewardAssetId.toString(),
      rewardPool,
      rewardRatePeriod: RewardRatePeriod.PER_SECOND,
      rewardRateAmount: config.rewardRate.amount
    });

    await ctx.store.save(reward);
  }
}

/**
 * Process `StakingRewards.Staked` event.
 *  - Create StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processStakedEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Processing `StakingRewards.Staked`");
  const evt = new StakingRewardsStakedEvent(ctx);
  const stakedEvent = getStakedEvent(evt);
  const owner = encodeAccount(stakedEvent.owner);
  const { poolId, fnftCollectionId, fnftInstanceId, amount, durationPreset, rewardMultiplier } = stakedEvent;
  const assetId = poolId.toString(); // assetId is used as poolId on the staking pallet

  const rewardPool = await ctx.store.get(RewardPool, poolId.toString());

  if (!rewardPool) {
    throw new Error(`Reward pool ${poolId} not found`);
  }

  const { event } = await saveAccountAndEvent(ctx, EventType.STAKING_REWARDS_STAKED, owner);

  const stakingPosition = createStakingPosition(
    rewardPool,
    fnftCollectionId,
    fnftInstanceId,
    assetId,
    owner,
    amount,
    durationPreset,
    rewardMultiplier,
    event,
    BigInt(ctx.block.timestamp)
  );

  await storeHistoricalLockedValue(ctx, [[assetId, amount]], LockedSource.StakingRewards, poolId.toString());

  await ctx.store.save(stakingPosition);
}

/**
 * Process `StakingRewards.StakeAmountExtended` event.
 *  - Update amount for StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processStakeAmountExtendedEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Processing `StakeAmountExtended`");
  const evt = new StakingRewardsStakeAmountExtendedEvent(ctx);
  const stakeAmountExtendedEvent = getStakeAmountExtendedEvent(evt);
  const { fnftCollectionId, fnftInstanceId, amount } = stakeAmountExtendedEvent;

  const stakingPosition = await ctx.store.get(StakingPosition, {
    where: {
      fnftCollectionId: fnftCollectionId.toString(),
      fnftInstanceId: fnftInstanceId.toString()
    },
    relations: {
      event: true
    }
  });

  if (!stakingPosition) {
    // no-op
    return;
  }

  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED,
    stakingPosition.owner
  );

  stakingPosition.event = event;
  stakingPosition.amount += amount;
  await ctx.store.save(stakingPosition);

  await storeHistoricalLockedValue(
    ctx,
    [[stakingPosition.assetId, amount]],
    LockedSource.StakingRewards,
    stakingPosition.assetId // assetId is used as poolId on the staking pallet
  );
}

/**
 * Process `StakingRewards.Unstaked` event.
 *  - Set amount for StakingPosition to 0.
 *  - Update account and store event.
 * @param ctx
 */
export async function processUnstakedEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Processing `StakingRewards.Unstaked`");
  const evt = new StakingRewardsUnstakedEvent(ctx);
  const { owner, fnftCollectionId, fnftInstanceId, slash } = getUnstakedEvent(evt);

  const position = await ctx.store.get(StakingPosition, {
    where: {
      fnftCollectionId: fnftCollectionId.toString(),
      fnftInstanceId: fnftInstanceId.toString()
    },
    relations: {
      event: true
    }
  });

  if (!position) {
    // no-op.
    return;
  }

  const { event } = await saveAccountAndEvent(ctx, EventType.STAKING_REWARDS_UNSTAKE, encodeAccount(owner));

  position.event = event;
  position.removed = true;
  await ctx.store.save(position);

  await storeHistoricalLockedValue(
    ctx,
    [[position.assetId, -position.amount]],
    LockedSource.StakingRewards,
    position.assetId // assetId is used as poolId on the staking pallet
  );
}

/**
 * Process `StakingRewards.SplitPosition` event.
 *  - Update amount for existing StakingPosition.
 *  - Create new StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processSplitPositionEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Processing `StakingRewards.SplitPosition`");
  const evt = new StakingRewardsSplitPositionEvent(ctx);
  const splitPositionEvent = getSplitPositionEvent(evt);
  const { positions } = splitPositionEvent;
  const [[fnftCollectionId, oldFnftInstanceId, oldPositionAmount], [_, newFnftInstanceId, newPositionAmount]] =
    positions;

  const position = await ctx.store.get(StakingPosition, {
    where: {
      fnftCollectionId: fnftCollectionId.toString(),
      fnftInstanceId: oldFnftInstanceId.toString()
    },
    relations: {
      event: true
    }
  });

  if (!position) {
    // no-op.
    return;
  }

  const { event } = await saveAccountAndEvent(ctx, EventType.STAKING_REWARDS_SPLIT_POSITION, position.owner);

  const newPosition = splitStakingPosition(position, oldPositionAmount, newPositionAmount, newFnftInstanceId, event);

  if (!newPosition) {
    // no-op.
    return;
  }

  await ctx.store.save(position);
  await ctx.store.save(newPosition);
}

/**
 * Process `StakingRewards.RewardPoolUpdated` event.
 *  - Update config for existing rewards.
 * @param ctx
 */
export async function processRewardPoolUpdatedEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Processing `StakingRewards.RewardPoolUpdated`");
  const evt = new StakingRewardsRewardPoolUpdatedEvent(ctx);
  const rewardPoolUpdatedEvent = getRewardPoolUpdatedEvent(evt);
  const { poolId, rewardUpdates } = rewardPoolUpdatedEvent;

  for (const [assetId, update] of rewardUpdates) {
    const reward = await ctx.store.get(Reward, {
      where: {
        id: assetId.toString(),
        rewardPool: {
          id: poolId.toString()
        }
      }
    });

    if (reward) {
      reward.rewardRateAmount = update.rewardRate.amount;
      await ctx.store.save(reward);
    }
  }
}
