import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  StakingRewardsRewardPoolCreatedEvent,
  StakingRewardsSplitPositionEvent,
  StakingRewardsStakeAmountExtendedEvent,
  StakingRewardsStakedEvent,
  StakingRewardsUnstakedEvent,
} from "../types/events";
import {
  getAssetIdFromRewardPoolId,
  saveAccountAndEvent,
  storeHistoricalLockedValue,
} from "../dbHelper";
import {
  RewardPool,
  StakingPosition,
  StakingSource,
  EventType,
  Event,
} from "../model";
import { encodeAccount } from "../utils";

interface RewardPoolCreatedEvent {
  poolId: bigint;
  owner: Uint8Array;
  endBlock: number;
  assetId: bigint;
}

interface StakedEvent {
  poolId: bigint;
  owner: Uint8Array;
  amount: bigint;
  durationPreset: bigint;
  positionId: bigint;
  keepAlive: boolean;
}

interface UnstakedEvent {
  owner: Uint8Array;
  positionId: bigint;
}

interface StakeAmountExtendedEvent {
  positionId: bigint;
  amount: bigint;
}

interface SplitPositionEvent {
  positions: [bigint, bigint][];
}

function getRewardPoolCreatedEvent(
  event: StakingRewardsRewardPoolCreatedEvent
): RewardPoolCreatedEvent {
  const { poolId, owner, endBlock, assetId } = event.asV2401 ?? event.asLatest;
  return { poolId, owner, endBlock, assetId };
}

function getStakedEvent(event: StakingRewardsStakedEvent): StakedEvent {
  const { poolId, owner, amount, durationPreset, positionId, keepAlive } =
    event.asV2401 ?? event.asLatest;
  return { poolId, owner, amount, durationPreset, positionId, keepAlive };
}

function getUnstakedEvent(event: StakingRewardsUnstakedEvent): UnstakedEvent {
  const { positionId, owner } = event.asV2401 ?? event.asLatest;
  return { positionId, owner };
}

function getStakeAmountExtendedEvent(
  event: StakingRewardsStakeAmountExtendedEvent
): StakeAmountExtendedEvent {
  const { positionId, amount } = event.asV2401 ?? event.asLatest;
  return { positionId, amount };
}

function getSplitPositionEvent(
  event: StakingRewardsSplitPositionEvent
): SplitPositionEvent {
  const { positions } = event.asV2401 ?? event.asLatest;
  return { positions };
}

export function createRewardPool(
  eventId: string,
  poolId: bigint,
  assetId: bigint
): RewardPool {
  return new RewardPool({
    id: randomUUID(),
    eventId,
    poolId: poolId.toString(),
    assetId: assetId.toString(),
  });
}

/**
 * Create new StakingPosition.
 * @param positionId
 * @param assetId
 * @param owner
 * @param amount
 * @param duration
 * @param event
 * @param startTimestamp
 */
export function createStakingPosition(
  positionId: string,
  assetId: string,
  owner: string,
  amount: bigint,
  duration: bigint,
  event: Event,
  startTimestamp: bigint
): StakingPosition {
  return new StakingPosition({
    id: randomUUID(),
    event,
    positionId: positionId.toString(),
    owner,
    amount,
    startTimestamp,
    endTimestamp: BigInt(startTimestamp + BigInt(duration * 1_000n)),
    assetId,
    source: StakingSource.StakingRewards,
  });
}

/**
 * Update position's amount in place.
 * @param position
 * @param newAmount
 * @param event
 */
export function extendStakingPosition(
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
 * @param newPositionId
 * @param event
 */
export function splitStakingPosition(
  position: StakingPosition,
  oldAmount: bigint,
  newAmount: bigint,
  newPositionId: bigint,
  event: Event
): StakingPosition {
  position.amount = oldAmount;
  position.event = event;

  return new StakingPosition({
    id: randomUUID(),
    event,
    positionId: newPositionId.toString(),
    owner: position.owner,
    amount: newAmount,
    startTimestamp: position.startTimestamp,
    endTimestamp: position.endTimestamp,
    assetId: position.assetId,
    source: StakingSource.StakingRewards,
  });
}

/**
 * Process `stakingRewards.RewardPoolCreated` event.
 *  - Create reward pool.
 *  - Update account and store event.
 * @param ctx
 */
export async function processRewardPoolCreatedEvent(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Start processing `reward pool created`");
  const evt = new StakingRewardsRewardPoolCreatedEvent(ctx);
  const event = getRewardPoolCreatedEvent(evt);
  const owner = encodeAccount(event.owner);

  const { assetId, poolId } = event;

  const stakingPool = createRewardPool(ctx.event.id, poolId, assetId);

  await ctx.store.save(stakingPool);

  await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_REWARD_POOL_CREATED,
    owner
  );
}

/**
 * Process `stakingRewards.Staked` event.
 *  - Create StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processStakedEvent(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Start processing `staked`");
  const evt = new StakingRewardsStakedEvent(ctx);
  const stakedEvent = getStakedEvent(evt);
  const owner = encodeAccount(stakedEvent.owner);
  const { poolId, positionId, amount, durationPreset } = stakedEvent;

  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_STAKED,
    owner
  );

  const assetId = await getAssetIdFromRewardPoolId(ctx, poolId);

  const stakingPosition = createStakingPosition(
    positionId.toString(),
    assetId,
    owner,
    amount,
    durationPreset,
    event,
    BigInt(ctx.block.timestamp)
  );

  await storeHistoricalLockedValue(ctx, amount, assetId);

  await ctx.store.save(stakingPosition);
}

/**
 * Process `stakingRewards.StakeAmountExtended` event.
 *  - Update amount for StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processStakeAmountExtendedEvent(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Start processing `StakeAmountExtended`");
  const evt = new StakingRewardsStakeAmountExtendedEvent(ctx);
  const stakeAmountExtendedevent = getStakeAmountExtendedEvent(evt);
  const { positionId, amount } = stakeAmountExtendedevent;

  const stakingPosition = await ctx.store.get(StakingPosition, {
    where: { positionId: positionId.toString() },
  });

  if (!stakingPosition) {
    // no-op
    return;
  }

  const oldAmount = stakingPosition.amount;
  const amountChanged = amount - oldAmount;

  const { event } = await saveAccountAndEvent(
    ctx,
    EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED,
    stakingPosition.owner
  );

  extendStakingPosition(stakingPosition, amount, event);

  await storeHistoricalLockedValue(ctx, amountChanged, stakingPosition.assetId);
}

/**
 * Process `stakingRewards.Unstaked` event.
 *  - Set amount for StakingPosition to 0.
 *  - Update account and store event.
 * @param ctx
 */
export async function processUnstakedEvent(
  ctx: EventHandlerContext
): Promise<void> {
  // TODO: when does this run?
  console.log("Start processing `Unstaked`");
  const evt = new StakingRewardsUnstakedEvent(ctx);
  const event = getUnstakedEvent(evt);
  const owner = encodeAccount(event.owner);
  const { positionId } = event;

  const position = await ctx.store.get(StakingPosition, {
    where: { positionId: positionId.toString() },
  });

  if (!position) {
    // no-op.
    return;
  }

  await saveAccountAndEvent(ctx, EventType.STAKING_REWARDS_UNSTAKE, owner);

  await storeHistoricalLockedValue(ctx, -position.amount, position.assetId);
}

/**
 * Process `stakingRewards.SplitPosition` event.
 *  - Update amount for existing StakingPosition.
 *  - Create new StakingPosition.
 *  - Update account and store event.
 * @param ctx
 */
export async function processSplitPositionEvent(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Start processing `SplitPosition`");
  const evt = new StakingRewardsSplitPositionEvent(ctx);
  const splitPositionevent = getSplitPositionEvent(evt);
  const { positions } = splitPositionevent;
  const [
    [oldPositionId, oldPositionAmount],
    [newPositionId, newPositionAmount],
  ] = positions;

  const position = await ctx.store.get(StakingPosition, {
    where: {
      positionId: oldPositionId.toString(),
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
    newPositionId,
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
