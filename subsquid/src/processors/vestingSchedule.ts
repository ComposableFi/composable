import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { randomUUID } from "crypto";
import { VestingSchedule as VestingScheduleType, VestingScheduleIdSet } from "../types/v10002";
import { EventType, LockedSource, Schedule, ScheduleWindow, VestingSchedule } from "../model";
import { VestingClaimedEvent, VestingVestingScheduleAddedEvent } from "../types/events";
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
 * Extract information about a VestingScheduleAdded event.
 * @param event
 */
function getVestingScheduleAddedEvent(event: VestingVestingScheduleAddedEvent): VestingScheduleAddedEvent {
  return event.asV10002;
}

/**
 * Create Schedule.
 * @param vestingSchedule
 */
export function createSchedule(vestingSchedule: VestingScheduleType): Schedule {
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
export function createVestingSchedule(
  ctx: EventHandlerContext<Store>,
  event: VestingVestingScheduleAddedEvent
): VestingSchedule {
  const { from, to, asset, schedule, scheduleAmount } = getVestingScheduleAddedEvent(event);

  const fromAccount = encodeAccount(from);
  const toAccount = encodeAccount(to);

  return new VestingSchedule({
    id: randomUUID(),
    scheduleId: schedule.vestingScheduleId,
    from: fromAccount,
    eventId: ctx.event.id,
    to: toAccount,
    assetId: asset.toString(),
    schedule: createSchedule(schedule),
    totalAmount: scheduleAmount,
    fullyClaimed: false,
    blockId: ctx.block.hash
  });
}

/**
 * Handle `vesting.VestingScheduleAdded` event.
 *  - Create and store VestingSchedule.
 *  - Create/update account.
 *  - Create event.
 * @param ctx
 */
export async function processVestingScheduleAddedEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Process VestingScheduleAdded");
  const event = new VestingVestingScheduleAddedEvent(ctx);

  const vestingSchedule = createVestingSchedule(ctx, event);

  await saveAccountAndEvent(ctx, EventType.VESTING_SCHEDULES_VESTING_SCHEDULE_ADDED, [
    vestingSchedule.from,
    vestingSchedule.to
  ]);

  await ctx.store.save(vestingSchedule);

  const { scheduleAmount, asset } = getVestingScheduleAddedEvent(event);

  await storeHistoricalLockedValue(
    ctx,
    [[asset.toString(), scheduleAmount]],
    LockedSource.VestingSchedules,
    vestingSchedule.scheduleId.toString()
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
function getVestingScheduleClaimedEvent(event: VestingClaimedEvent): VestingScheduleClaimedEvent {
  return event.asV10002;
}

/**
 * Update already claimed amount and set the schedule as full claimed when
 * necessary.
 * @param ctx
 * @param vestingSchedule
 * @param claimed
 */
export function updatedClaimedAmount(
  ctx: EventHandlerContext<Store>,
  vestingSchedule: VestingSchedule,
  claimed: bigint
): void {
  vestingSchedule.schedule.alreadyClaimed += claimed;
  if (vestingSchedule.schedule.alreadyClaimed === vestingSchedule.totalAmount) {
    vestingSchedule.fullyClaimed = true;
  }
  vestingSchedule.blockId = ctx.block.hash;
}

/**
 * Process `vesting.Claimed` event.
 *  - Update alreadyClaimed amount for each claimed schedule.
 *  - Set fullyClaimed when whole locked value has been claimed.
 * @param ctx
 */
export async function processVestingClaimedEvent(ctx: EventHandlerContext<Store>): Promise<void> {
  console.log("Process Claimed");
  const event = new VestingClaimedEvent(ctx);

  const { who, claimedAmountPerSchedule } = getVestingScheduleClaimedEvent(event);

  await saveAccountAndEvent(ctx, EventType.VESTING_SCHEDULES_CLAIMED, encodeAccount(who));

  for (let i = 0; i < claimedAmountPerSchedule.length; i += 1) {
    const [id, amount] = claimedAmountPerSchedule[i];

    const schedule: VestingSchedule | undefined = await ctx.store.get(VestingSchedule, {
      where: {
        scheduleId: id
      }
    });

    if (!schedule) {
      // no-op
      return;
    }

    schedule.eventId = ctx.event.id;

    updatedClaimedAmount(ctx, schedule, amount);

    await ctx.store.save(schedule);

    await storeHistoricalLockedValue(
      ctx,
      [[schedule.assetId, -amount]],
      LockedSource.VestingSchedules,
      schedule.scheduleId.toString()
    );
  }
}
