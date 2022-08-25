import { EventHandlerContext } from "@subsquid/substrate-processor";
import {
  VestingSchedule as VestingScheduleType,
  VestingScheduleIdSet,
} from "../types/v2401";
import {
  PicassoTransactionType,
  Schedule,
  ScheduleWindow,
  VestingSchedule,
} from "../model";
import {
  VestingClaimedEvent,
  VestingVestingScheduleAddedEvent,
} from "../types/events";
import { encodeAccount } from "../utils";
import { saveAccountAndTransaction } from "../dbHelper";
import { randomUUID } from "crypto";

interface VestingScheduleAddedEvent {
  from: Uint8Array;
  to: Uint8Array;
  asset: bigint;
  schedule: VestingScheduleType;
}

/**
 * Extracts information about a VestingScheduleAdded event
 * @param event
 */
function getVestingScheduleAddedEvent(
  event: VestingVestingScheduleAddedEvent
): VestingScheduleAddedEvent {
  return event.asV2401 ?? event.asLatest;
}

/**
 * Creates Schedule
 * @param vestingSchedule
 */
export function createVestingSchedule(
  vestingSchedule: VestingScheduleType
): Schedule {
  const vestingWindow = new ScheduleWindow();
  vestingWindow.start = BigInt(vestingSchedule.window.start);
  vestingWindow.period = BigInt(vestingSchedule.window.period);
  vestingWindow.kind = vestingSchedule.window.__kind;

  const schedule = new Schedule();
  schedule.vestingScheduleId = vestingSchedule.vestingScheduleId;
  schedule.window = vestingWindow;
  schedule.periodCount = BigInt(vestingSchedule.periodCount);
  schedule.perPeriod = BigInt(vestingSchedule.perPeriod);
  schedule.alreadyClaimed = vestingSchedule.alreadyClaimed;

  return schedule;
}

/**
 * Based on the event, return a new VestingSchedule.
 * @param ctx
 * @param event
 */
export function getNewVestingSchedule(
  ctx: EventHandlerContext,
  event: VestingVestingScheduleAddedEvent
): VestingSchedule {
  const { from, to, asset, schedule } = getVestingScheduleAddedEvent(event);

  const fromAccount = encodeAccount(from);
  const toAccount = encodeAccount(to);

  return new VestingSchedule({
    id: randomUUID(),
    scheduleId: schedule.vestingScheduleId,
    from: fromAccount,
    eventId: ctx.event.id,
    to: toAccount,
    asset,
    schedule: createVestingSchedule(schedule),
    fullyClaimed: false,
  });
}

/**
 * Handle `vesting.VestingScheduleAdded` event.
 *  - Create and store VestingSchedule.
 *  - Create/update account.
 *  - Create transaction.
 * @param ctx
 */
export async function processVestingScheduleAddedEvent(
  ctx: EventHandlerContext
): Promise<void> {
  const event = new VestingVestingScheduleAddedEvent(ctx);

  const vestingSchedule = getNewVestingSchedule(ctx, event);

  await ctx.store.save(vestingSchedule);

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.VESTING_SCHEDULES_VESTING_SCHEDULE_ADDED,
    [vestingSchedule.from, vestingSchedule.to]
  );
}

interface VestingScheduleClaimedEvent {
  who: Uint8Array;
  asset: bigint;
  lockedAmount: bigint;
  claimedAmount: bigint;
  vestingScheduleIds: VestingScheduleIdSet;
}

/**
 * Extracts information about a VestingClaimed event
 * @param event
 */
function getVestingScheduleClaimedEvent(
  event: VestingClaimedEvent
): VestingScheduleClaimedEvent {
  return event.asV2401 ?? event.asLatest;
}

function updatedClaimedAmount(schedule: VestingSchedule, claimed: bigint) {
  // TODO
}

/**
 * Process `vesting.Claimed` event.
 *  - Update alreadyClaimed amount for each claimed schedule.
 * @param ctx
 * @param event
 */
export async function processVestingClaimedEvent(
  ctx: EventHandlerContext,
  event: VestingClaimedEvent
): Promise<void> {
  const {
    who,
    // asset, lockedAmount, claimedAmount, vestingScheduleIds
  } = getVestingScheduleClaimedEvent(event);

  // TODO: update claimed amount
  // this requires the pallet to emit the claimed amount PER SCHEDULE and not just
  // the total claimed amount

  // const schedule: VestingSchedule | undefined = await ctx.store.get(
  //   VestingSchedule,
  //   {
  //     where: {
  //       id:
  //     },
  //   }
  // );

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.VESTING_SCHEDULES_CLAIMED,
    encodeAccount(who)
  );
}
