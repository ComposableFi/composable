import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
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
    from: fromAccount,
    eventId: ctx.event.id,
    scheduleId: schedule.vestingScheduleId,
    to: toAccount,
    asset,
    schedule: createVestingSchedule(schedule),
  });
}

/**
 * Updates database with vesting schedule information
 * @param ctx
 */
export async function processVestingScheduleAddedEvent(
  ctx: EventHandlerContext
) {
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

function updatedClaimedAmount() {
  // TODO
}

/**
 * Updates database with vesting schedule information
 * @param ctx
 * @param event
 */
export async function processVestingClaimedEvent(
  ctx: EventHandlerContext,
  event: VestingClaimedEvent
) {
  const { who, asset, lockedAmount, claimedAmount } =
    getVestingScheduleClaimedEvent(event);

  // TODO: update claimed amount

  // const schedule: VestingSchedule | undefined = await ctx.store.get(VestingSchedule, {
  //   where: {
  //     // TODO
  //   }
  // })

  await saveAccountAndTransaction(
    ctx,
    PicassoTransactionType.VESTING_SCHEDULES_CLAIMED,
    encodeAccount(who)
  );
}
