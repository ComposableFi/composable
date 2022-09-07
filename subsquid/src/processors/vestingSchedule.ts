import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  VestingSchedule as VestingScheduleType,
  VestingScheduleIdSet,
} from "../types/v2401";
import { Schedule, ScheduleWindow, EventType, VestingSchedule } from "../model";
import {
  VestingClaimedEvent,
  VestingVestingScheduleAddedEvent,
} from "../types/events";
import { encodeAccount } from "../utils";
import { saveAccountAndEvent, storeHistoricalLockedValue } from "../dbHelper";

interface VestingScheduleAddedEvent {
  from: Uint8Array;
  to: Uint8Array;
  asset: bigint;
  vestingScheduleId: bigint;
  schedule: VestingScheduleType;
  scheduleAmount: bigint;
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
  const { from, to, asset, schedule, scheduleAmount } =
    getVestingScheduleAddedEvent(event);

  const fromAccount = encodeAccount(from);
  const toAccount = encodeAccount(to);

  return new VestingSchedule({
    id: randomUUID(),
    scheduleId: schedule.vestingScheduleId,
    from: fromAccount,
    eventId: ctx.event.id,
    to: toAccount,
    assetId: asset.toString(),
    schedule: createVestingSchedule(schedule),
    totalAmount: scheduleAmount,
    fullyClaimed: false,
  });
}

/**
 * Handle `vesting.VestingScheduleAdded` event.
 *  - Create and store VestingSchedule.
 *  - Create/update account.
 *  - Create event.
 * @param ctx
 */
export async function processVestingScheduleAddedEvent(
  ctx: EventHandlerContext
): Promise<void> {
  const event = new VestingVestingScheduleAddedEvent(ctx);

  const vestingSchedule = getNewVestingSchedule(ctx, event);

  await ctx.store.save(vestingSchedule);

  const { scheduleAmount, asset } = getVestingScheduleAddedEvent(event);

  await storeHistoricalLockedValue(
    ctx,
    scheduleAmount,
    ctx.event.id,
    asset.toString()
  );

  await saveAccountAndEvent(
    ctx,
    EventType.VESTING_SCHEDULES_VESTING_SCHEDULE_ADDED,
    [vestingSchedule.from, vestingSchedule.to]
  );
}

interface VestingScheduleClaimedEvent {
  who: Uint8Array;
  asset: bigint;
  lockedAmount: bigint;
  claimedAmountPerSchedule: [bigint, bigint][];
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

/**
 * Update already claimed amount and set the schedule as full claimed when
 * necessary.
 * @param vestingSchedule
 * @param claimed
 */
export function updatedClaimedAmount(
  vestingSchedule: VestingSchedule,
  claimed: bigint
): void {
  vestingSchedule.schedule.alreadyClaimed += claimed;
  if (vestingSchedule.schedule.alreadyClaimed === vestingSchedule.totalAmount) {
    vestingSchedule.fullyClaimed = true;
  }
}

/**
 * Process `vesting.Claimed` event.
 *  - Update alreadyClaimed amount for each claimed schedule.
 *  - Set fullyClaimed when whole locked value has been claimed.
 * @param ctx
 */
export async function processVestingClaimedEvent(
  ctx: EventHandlerContext
): Promise<void> {
  const event = new VestingClaimedEvent(ctx);

  const { who, claimedAmountPerSchedule } =
    getVestingScheduleClaimedEvent(event);

  for (let i = 0; i < claimedAmountPerSchedule.length; i += 1) {
    const [id, amount] = claimedAmountPerSchedule[i];

    const schedule: VestingSchedule | undefined = await ctx.store.get(
      VestingSchedule,
      {
        where: {
          id,
        },
      }
    );

    if (!schedule) {
      // no-op
      return;
    }

    schedule.eventId = ctx.event.id;

    updatedClaimedAmount(schedule, amount);

    await ctx.store.save(schedule);

    await storeHistoricalLockedValue(
      ctx,
      -amount,
      ctx.event.id,
      schedule.assetId
    );
  }

  await saveAccountAndEvent(
    ctx,
    EventType.VESTING_SCHEDULES_CLAIMED,
    encodeAccount(who)
  );
}
