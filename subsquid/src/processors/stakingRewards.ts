import { EventHandlerContext } from "@subsquid/substrate-processor";
import {
  StakingRewardsRewardPoolCreatedEvent,
  StakingRewardsStakedEvent,
  StakingRewardsUnstakedEvent,
} from "../types/events";
import { saveAccountAndTransaction } from "../dbHelper";
import { PicassoTransactionType } from "../model";
import { encodeAccount } from "../utils";

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

interface UnstakedEvent {
  owner: Uint8Array;
  positionId: bigint;
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

function getUnstakedEvent(event: StakingRewardsUnstakedEvent): UnstakedEvent {
  const { positionId, owner } = event.asV2401 ?? event.asLatest;
  return { positionId, owner };
}

export async function processRewardPoolCreatedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `reward pool created`");
  const evt = new StakingRewardsRewardPoolCreatedEvent(ctx);
  const event = getRewardPoolCreatedEvent(evt);
  const owner = encodeAccount(event.owner);

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.STAKING_REWARDS_REWARD_POOL_CREATED,
    owner
  );
}

export async function processStakedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `staked`");
  const evt = new StakingRewardsStakedEvent(ctx);
  const event = getStakedEvent(evt);
  const owner = encodeAccount(event.owner);

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.STAKING_REWARDS_STAKED,
    owner
  );
}

export async function processStakeAmountExtendedEvent(
  ctx: EventHandlerContext
) {
  console.log("Start processing `StakeAmountExtended`");

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED
  );
}

export async function processUnstakedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `Unstaked`");
  const evt = new StakingRewardsUnstakedEvent(ctx);
  const event = getUnstakedEvent(evt);
  const owner = encodeAccount(event.owner);

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.STAKING_REWARDS_UNSTAKE,
    owner
  );
}

export async function processSplitPositionEvent(ctx: EventHandlerContext) {
  console.log("Start processing `SplitPosition`");

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.STAKING_REWARDS_SPLIT_POSITION
  );
}
