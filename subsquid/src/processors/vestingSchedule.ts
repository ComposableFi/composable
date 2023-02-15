import { randomUUID } from "crypto";
import { VestingSchedule as VestingScheduleType, VestingScheduleIdSet } from "../types/v10002";
import { EventType, LockedSource, Schedule, ScheduleWindow, VestingSchedule } from "../model";
import { VestingClaimedEvent, VestingVestingScheduleAddedEvent } from "../types/events";
import { encodeAccount } from "../utils";
import { saveAccountAndEvent, storeHistoricalLockedValue } from "../dbHelper";
import { Context, EventItem, Block } from "../processorTypes";

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
  if (event.isV1000) {
    // Should never be reached
    const { asset, schedule, from, to } = event.asV1000;
    return {
      asset,
      schedule: {
        ...schedule,
        vestingScheduleId: 0n,
        alreadyClaimed: 0n
      },
      from,
      to,
      scheduleAmount: 0n,
      vestingScheduleId: 0n
    };
  }
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
 * Handle `vesting.VestingScheduleAdded` event.
 *  - Create and store VestingSchedule.
 *  - Create/update account.
 *  - Create event.
 * @param ctx
 * @param block
 * @param eventItem
 */
export async function processVestingScheduleAddedEvent(
  ctx: Context,
  block: Block,
  eventItem: EventItem
): Promise<void> {
  console.log("Process VestingScheduleAdded");
  const event = new VestingVestingScheduleAddedEvent(ctx, eventItem.event);

  const { from, to, asset, schedule, scheduleAmount } = getVestingScheduleAddedEvent(event);

  const fromAccount = encodeAccount(from);
  const toAccount = encodeAccount(to);

  const vestingSchedule = new VestingSchedule({
    id: randomUUID(),
    scheduleId: schedule.vestingScheduleId,
    from: fromAccount,
    eventId: eventItem.event.id,
    to: toAccount,
    assetId: asset.toString(),
    schedule: createSchedule(schedule),
    totalAmount: scheduleAmount,
    fullyClaimed: false,
    blockId: block.header.hash
  });

  await saveAccountAndEvent(ctx, block, eventItem, EventType.VESTING_SCHEDULES_VESTING_SCHEDULE_ADDED, [
    vestingSchedule.from,
    vestingSchedule.to
  ]);

  await ctx.store.save(vestingSchedule);

  await storeHistoricalLockedValue(
    ctx,
    block,
    eventItem,
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
  if (event.isV1000) {
    // Should never be reached
    const { asset, lockedAmount, who } = event.asV1000;
    return {
      asset,
      lockedAmount,
      who,
      claimedAmountPerSchedule: [],
      vestingScheduleIds: {
        __kind: "All"
      }
    };
  }
  return event.asV10002;
}

/**
 * Process `vesting.Claimed` event.
 *  - Update alreadyClaimed amount for each claimed schedule.
 *  - Set fullyClaimed when whole locked value has been claimed.
 * @param ctx
 * @param block
 * @param eventItem
 */
export async function processVestingClaimedEvent(ctx: Context, block: Block, eventItem: EventItem): Promise<void> {
  console.log("Process Claimed");
  const event = new VestingClaimedEvent(ctx, eventItem.event);

  const { who, claimedAmountPerSchedule } = getVestingScheduleClaimedEvent(event);

  await saveAccountAndEvent(ctx, block, eventItem, EventType.VESTING_SCHEDULES_CLAIMED, encodeAccount(who));

  for (let i = 0; i < claimedAmountPerSchedule.length; i += 1) {
    const [id, amount] = claimedAmountPerSchedule[i];

    const vestingSchedule: VestingSchedule | undefined = await ctx.store.get(VestingSchedule, {
      where: {
        scheduleId: id
      }
    });

    if (!vestingSchedule) {
      // no-op
      return;
    }

    vestingSchedule.eventId = eventItem.event.id;
    vestingSchedule.schedule.alreadyClaimed += amount;

    if (vestingSchedule.schedule.alreadyClaimed === vestingSchedule.totalAmount) {
      vestingSchedule.fullyClaimed = true;
    }
    vestingSchedule.blockId = block.header.hash;

    await ctx.store.save(vestingSchedule);

    await storeHistoricalLockedValue(
      ctx,
      block,
      eventItem,
      [[vestingSchedule.assetId, -amount]],
      LockedSource.VestingSchedules,
      vestingSchedule.scheduleId.toString()
    );
  }
}
