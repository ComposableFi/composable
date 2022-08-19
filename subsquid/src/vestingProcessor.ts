import { EventHandlerContext } from "@subsquid/substrate-processor";
import { VestingSchedule as VestingScheduleType } from "./types/v2401";
import { Schedule, ScheduleWindow, VestingSchedule } from "./model";
import { VestingVestingScheduleAddedEvent } from "./types/events";
import { encodeAccount } from "./utils";

interface VestingScheduleAddedEvent {
  from: Uint8Array;
  to: Uint8Array;
  asset: bigint;
  schedule: VestingScheduleType;
  vestingScheduleId: bigint;
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
  const { from, to, asset, schedule, vestingScheduleId } =
    getVestingScheduleAddedEvent(event);

  const toAccount = encodeAccount(to);
  const fromAccount = encodeAccount(from);

  const vestingSchedule = new VestingSchedule({
    id: vestingScheduleId.toString(),
    from: fromAccount,
    eventId: ctx.event.id,
    scheduleId: `${toAccount}-${asset.toString()}`,
    to: toAccount,
    schedule: createVestingSchedule(schedule),
  });

  await ctx.store.save(vestingSchedule);
}
