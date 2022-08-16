import { VestingClaimedEvent } from "../types/events";
import { EventHandlerContext } from "@subsquid/substrate-processor";
import { encodeAccount } from "../utils";

interface VestingScheduleClaimedEvent {
  who: Uint8Array;
  asset: bigint;
  lockedAmount: bigint;
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
 * Updates database with vesting schedule information
 * @param ctx
 * @param event
 */
export async function processVestingClaimedEvent(
  ctx: EventHandlerContext,
  event: VestingClaimedEvent
) {
  const { who, asset, lockedAmount } = getVestingScheduleClaimedEvent(event);

  const account = encodeAccount(who);

  // const schedule: VestingSchedule | undefined = await ctx.store.get(VestingSchedule, {
  //   where: {
  //     // TODO
  //   }
  // })

  // if (account) {
  //   await ctx.store.save(schedule);
  // }
}
