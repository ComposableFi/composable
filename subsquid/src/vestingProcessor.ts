import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import { VestingSchedule as VestingScheduleType } from "./types/v2300";
import { Schedule, ScheduleWindow, VestingSchedule } from "./model";
import { VestingVestingScheduleAddedEvent } from "./types/events";
import { encodeAccount } from "./utils";

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
  if (event.isV2300) {
    const { from, to, asset, schedule } = event.asV2300;

    return { from, to, asset, schedule };
  }

  const { from, to, asset, schedule } = event.asLatest;

  return { from, to, asset, schedule };
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
  schedule.window = vestingWindow;
  schedule.periodCount = BigInt(vestingSchedule.periodCount);
  schedule.perPeriod = BigInt(vestingSchedule.perPeriod);

  return schedule;
}

/**
 * Updates database with vesting schedule information
 * @param ctx
 * @param event
 */
export async function processVestingScheduleAddedEvent(
  ctx: EventHandlerContext,
  event: VestingVestingScheduleAddedEvent
) {
  const { to, asset, schedule } = getVestingScheduleAddedEvent(event);

  const beneficiary = encodeAccount(to);

  const vestingSchedule = new VestingSchedule({
    id: randomUUID(),
    eventId: ctx.event.id,
    scheduleId: `${beneficiary}-${asset.toString()}`,
    beneficiary,
    schedule: createVestingSchedule(schedule),
  });

  await ctx.store.save(vestingSchedule);
}
