import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import {
  StakingRewardsRewardPoolCreatedEvent,
  StakingRewardsSplitPositionEvent,
  StakingRewardsStakeAmountExtendedEvent,
  StakingRewardsStakedEvent,
  StakingRewardsUnstakedEvent,
} from "../types/events";
import { saveAccountAndEvent, storeHistoricalLockedValue } from "../dbHelper";
import {
  Event,
  EventType,
  LockedSource,
  RewardPool,
  StakingPosition,
} from "../model";
import { encodeAccount } from "../utils";

interface RewardPoolCreatedEvent {
  poolId: bigint;
  owner: Uint8Array;
  endBlock: number;
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

function getRewardPoolCreatedEvent(
  event: StakingRewardsRewardPoolCreatedEvent
): RewardPoolCreatedEvent {
  const { poolId, owner, endBlock } = event.asV2402;
  return { poolId, owner, endBlock };
}

function getStakedEvent(event: StakingRewardsStakedEvent): StakedEvent {
  const {
    poolId,
    owner,
    amount,
    durationPreset,
    fnftCollectionId,
    fnftInstanceId,
    rewardMultiplier,
    keepAlive,
  } = event.asV2402;
  return {
    poolId,
    owner,
    amount,
    durationPreset,
    fnftCollectionId,
    fnftInstanceId,
    rewardMultiplier,
    keepAlive,
  };
}

function getUnstakedEvent(event: StakingRewardsUnstakedEvent): UnstakedEvent {
  const { owner, fnftCollectionId, fnftInstanceId, slash } = event.asV2402;
  return { owner, fnftCollectionId, fnftInstanceId, slash };
}

function getStakeAmountExtendedEvent(
  event: StakingRewardsStakeAmountExtendedEvent
): StakeAmountExtendedEvent {
  const { fnftCollectionId, fnftInstanceId, amount } = event.asV2402;
  return { fnftCollectionId, fnftInstanceId, amount };
}

function getSplitPositionEvent(
  event: StakingRewardsSplitPositionEvent
): SplitPositionEvent {
  const { positions } = event.asV2402;
  return { positions };
}

export function createRewardPool(eventId: string, poolId: bigint): RewardPool {
  return new RewardPool({
    id: randomUUID(),
    eventId,
    poolId: poolId.toString(),
  });
}

/**
 * Create new StakingPosition.
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
    owner,
    amount,
    startTimestamp,
    duration,
    endTimestamp: BigInt(startTimestamp + BigInt(duration * 1_000n)),
    rewardMultiplier,
    assetId,
    source: LockedSource.StakingRewards,
  });
}

/**
 * Update position's amount in place.
 * @param position
 * @param newAmount
 * @param event
 */
export function updateStakingPositionAmount(
  position: StakingPosition,
  newAmount: bigint,
  event: Event
): void {
  position.amount = newAmount;
  position.event = event;
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
  });
}

/**
 * Process `StakingRewards.RewardPoolCreated` event.
 *  - Create reward pool.
 *  - Update account and store event.
 * @param ctx
 */
export async function processRewardPoolCreatedEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Processing `StakingRewards.RewardPoolCreated`");
  const evt = new StakingRewardsRewardPoolCreatedEvent(ctx);
  const event = getRewardPoolCreatedEvent(evt);
  const owner = encodeAccount(event.owner);

  const { poolId } = event;

  const stakingPool = createRewardPool(ctx.event.id, poolId);

  await ctx.store.save(stakingPool);

  await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_REWARD_POOL_CREATED,
    owner
  );
}

/**
 * Process `StakingRewards.Staked` event.
 *  - Create StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processStakedEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Processing `StakingRewards.Staked`");
  const evt = new StakingRewardsStakedEvent(ctx);
  const stakedEvent = getStakedEvent(evt);
  const owner = encodeAccount(stakedEvent.owner);
  const {
    poolId,
    fnftCollectionId,
    fnftInstanceId,
    amount,
    durationPreset,
    rewardMultiplier,
  } = stakedEvent;

  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_STAKED,
    owner
  );

  const stakingPosition = createStakingPosition(
    fnftCollectionId,
    fnftInstanceId,
    poolId.toString(), // assetId is used as poolId on the staking pallet
    owner,
    amount,
    durationPreset,
    rewardMultiplier,
    event,
    BigInt(ctx.block.timestamp)
  );

  await storeHistoricalLockedValue(
    ctx,
    {
      [poolId.toString()]: amount,
    },
    LockedSource.StakingRewards
  );

  await ctx.store.save(stakingPosition);
}

/**
 * Process `StakingRewards.StakeAmountExtended` event.
 *  - Update amount for StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processStakeAmountExtendedEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Processing `StakeAmountExtended`");
  const evt = new StakingRewardsStakeAmountExtendedEvent(ctx);
  const stakeAmountExtendedEvent = getStakeAmountExtendedEvent(evt);
  const { fnftCollectionId, fnftInstanceId, amount } = stakeAmountExtendedEvent;

  const stakingPosition = await ctx.store.get(StakingPosition, {
    where: {
      fnftCollectionId: fnftCollectionId.toString(),
      fnftInstanceId: fnftInstanceId.toString(),
    },
    relations: {
      event: true,
    },
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

  updateStakingPositionAmount(
    stakingPosition,
    stakingPosition.amount + amount,
    event
  );

  await ctx.store.save(stakingPosition);

  await storeHistoricalLockedValue(
    ctx,
    {
      [stakingPosition.assetId]: amount,
    },
    LockedSource.StakingRewards
  );
}

/**
 * Process `StakingRewards.Unstaked` event.
 *  - Set amount for StakingPosition to 0.
 *  - Update account and store event.
 * @param ctx
 */
export async function processUnstakedEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Processing `StakingRewards.Unstaked`");
  const evt = new StakingRewardsUnstakedEvent(ctx);
  const { owner, fnftCollectionId, fnftInstanceId, slash } =
    getUnstakedEvent(evt);

  const position = await ctx.store.get(StakingPosition, {
    where: {
      fnftCollectionId: fnftCollectionId.toString(),
      fnftInstanceId: fnftInstanceId.toString(),
    },
    relations: {
      event: true,
    },
  });

  if (!position) {
    // no-op.
    return;
  }

  const unstakedAmount = BigInt(position.amount) - BigInt(slash || 0);

  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_UNSTAKE,
    encodeAccount(owner)
  );

  updateStakingPositionAmount(
    position,
    position.amount - unstakedAmount,
    event
  );

  await ctx.store.save(position);

  await storeHistoricalLockedValue(
    ctx,
    {
      [position.assetId]: -unstakedAmount,
    },
    LockedSource.StakingRewards
  );
}

/**
 * Process `StakingRewards.SplitPosition` event.
 *  - Update amount for existing StakingPosition.
 *  - Create new StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processSplitPositionEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Processing `StakingRewards.SplitPosition`");
  const evt = new StakingRewardsSplitPositionEvent(ctx);
  const splitPositionEvent = getSplitPositionEvent(evt);
  const { positions } = splitPositionEvent;
  const [
    [fnftCollectionId, oldFnftInstanceId, oldPositionAmount],
    [_, newFnftInstanceId, newPositionAmount],
  ] = positions;

  const position = await ctx.store.get(StakingPosition, {
    where: {
      fnftCollectionId: fnftCollectionId.toString(),
      fnftInstanceId: oldFnftInstanceId.toString(),
    },
    relations: {
      event: true,
    },
  });

  if (!position) {
    // no-op.
    return;
  }

  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_SPLIT_POSITION,
    position.owner
  );

  const newPosition = splitStakingPosition(
    position,
    oldPositionAmount,
    newPositionAmount,
    newFnftInstanceId,
    event
  );

  if (!newPosition) {
    // no-op.
    return;
  }

  await ctx.store.save(position);
  await ctx.store.save(newPosition);

  // TODO: add data about new positions
}
