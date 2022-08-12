import { EventHandlerContext } from "@subsquid/substrate-processor";
import {
  StakingRewardsRewardPoolCreatedEvent,
  StakingRewardsSplitPositionEvent,
  StakingRewardsStakeAmountExtendedEvent,
  StakingRewardsStakedEvent,
  StakingRewardsUnstakedEvent,
} from "./types/events";
import { getOrCreate } from "./dbHelper";
import { Account, PicassoTransactionType } from "./model";
import { createTransaction, encodeAccount, updateBalance } from "./utils";

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

export async function processRewardPoolCreatedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `reward pool created`");
  const evt = new StakingRewardsRewardPoolCreatedEvent(ctx);
  const event = getRewardPoolCreatedEvent(evt);
  const owner = encodeAccount(event.owner);

  const account = await getOrCreate(ctx.store, Account, owner);
  updateBalance(account, ctx);

  // Create transaction
  const tx = createTransaction(
    ctx,
    owner,
    PicassoTransactionType.STAKING_REWARDS_REWARD_POOL_CREATED
  );

  await ctx.store.save(account);
  await ctx.store.save(tx);
  console.log("Finish processing `reward pool created`");
}

export async function processStakedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `staked`");
  const evt = new StakingRewardsStakedEvent(ctx);
  const event = getStakedEvent(evt);
  const owner = encodeAccount(event.owner);

  const account = await getOrCreate(ctx.store, Account, owner);
  updateBalance(account, ctx);

  // Create transaction
  const tx = createTransaction(
    ctx,
    owner,
    PicassoTransactionType.STAKING_REWARDS_STAKED
  );

  // TODO: when staking is balanced changed or just locked?

  await ctx.store.save(account);
  await ctx.store.save(tx);
  console.log("Finish processing `staked`");
}

export async function processStakeAmountExtendedEvent(
  ctx: EventHandlerContext
) {
  console.log("Start processing `StakeAmountExtended`");
  const evt = new StakingRewardsStakeAmountExtendedEvent(ctx);
  const event = getStakeAmountExtendedEvent(evt);
  // const owner = encodeAccount(event.); // TODO: owner?

  // const account = await getOrCreate(ctx.store, Account, owner);
  // updateBalance(account, ctx);

  // Create transaction
  // const tx = createTransaction(
  //   ctx,
  //   account,
  //   PicassoTransactionType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED
  // );
  //
  // await ctx.store.save(account);
  // await ctx.store.save(tx);

  // TODO?
  console.log("Finish processing `StakeAmountExtended`");
}

export async function processUnstakedEvent(ctx: EventHandlerContext) {
  console.log("Start processing `Unstaked`");
  const evt = new StakingRewardsUnstakedEvent(ctx);
  const event = getUnstakedEvent(evt);
  const owner = encodeAccount(event.owner);

  const account = await getOrCreate(ctx.store, Account, owner);
  updateBalance(account, ctx);

  // Create transaction
  const tx = createTransaction(
    ctx,
    owner,
    PicassoTransactionType.STAKING_REWARDS_UNSTAKE
  );

  // TODO: when staking is balanced changed or just unlocked?

  await ctx.store.save(account);
  await ctx.store.save(tx);

  console.log("Finish processing `Unstaked`");
}

export async function processSplitPositionEvent(ctx: EventHandlerContext) {
  console.log("Start processing `SplitPosition`");
  const evt = new StakingRewardsSplitPositionEvent(ctx);
  const event = getSplitPositionEvent(evt);
  // TODO? need account
  console.log("Finish processing `SplitPosition`");
}
