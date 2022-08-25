import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { VestingSchedule } from "../src/model";
import { anything, instance, mock, when } from "ts-mockito";
import { createAccount, createCtx, encodeAccount } from "../src/utils";
import { getNewVestingSchedule } from "../src/processors/vestingSchedule";
import { VestingSchedule as VestingScheduleType } from "../src/types/v2401";
import { VestingVestingScheduleAddedEvent } from "../src/types/events";
import { expect } from "chai";

const MOCK_ADDRESS_FROM = createAccount();
const MOCK_ADDRESS_TO = createAccount();

const MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED: VestingScheduleType = {
  vestingScheduleId: 3n,
  window: {
    start: 1,
    period: 10,
    __kind: "BlockNumberBased",
  },
  periodCount: 1,
  perPeriod: 100n,
  alreadyClaimed: 10n,
};

const MOCK_VESTING_SCHEDULE_MOMENT_BASED: VestingScheduleType = {
  vestingScheduleId: 4n,
  window: {
    start: 1n,
    period: 10n,
    __kind: "MomentBased",
  },
  periodCount: 1,
  perPeriod: 100n,
  alreadyClaimed: 20n,
};

/**
 * Check if vesting schedule has expected values
 * @param vestingSchedule
 * @param from
 * @param to
 * @param eventId
 * @param asset
 * @param schedule
 * @param vestingScheduleId
 * @param fullyClaimed
 */
function assertVestingSchedule(
  vestingSchedule: VestingSchedule,
  from?: string,
  to?: string,
  eventId?: string,
  asset?: bigint,
  schedule?: VestingScheduleType,
  vestingScheduleId?: bigint,
  fullyClaimed?: boolean
) {
  if (from) expect(vestingSchedule.from).to.equal(from);
  if (to) expect(vestingSchedule.to).to.equal(to);
  if (eventId) expect(vestingSchedule.eventId).to.equal(eventId);
  if (asset) expect(vestingSchedule.asset).to.equal(asset);
  if (schedule) {
    expect(vestingSchedule.schedule.window.period).to.equal(
      BigInt(schedule.window.period)
    );
    expect(vestingSchedule.schedule.window.start).to.equal(
      BigInt(schedule.window.start)
    );
    expect(vestingSchedule.schedule.window.kind).to.equal(
      schedule.window.__kind
    );

    expect(vestingSchedule.schedule.periodCount).to.equal(
      BigInt(schedule.periodCount)
    );
    expect(vestingSchedule.schedule.perPeriod).to.equal(schedule.perPeriod);
    expect(vestingSchedule.schedule.vestingScheduleId).to.equal(
      schedule.vestingScheduleId
    );
    expect(vestingSchedule.schedule.alreadyClaimed).to.equal(
      schedule.alreadyClaimed
    );
  }
  if (vestingScheduleId)
    expect(vestingSchedule.scheduleId).to.equal(vestingScheduleId);
  if (fullyClaimed) expect(vestingSchedule.fullyClaimed).to.equal(fullyClaimed);
}

function createVestingScheduleAddedEvent(
  from: Uint8Array,
  to: Uint8Array,
  asset: bigint,
  schedule: VestingScheduleType
) {
  let eventMock = mock(VestingVestingScheduleAddedEvent);
  let evt = {
    from,
    to,
    asset,
    schedule,
    vestingScheduleId: schedule.vestingScheduleId,
    alreadyClaimed: schedule.vestingScheduleId,
  };

  when(eventMock.asV2401).thenReturn(evt);
  when(eventMock.asLatest).thenReturn(evt);

  let event = instance(eventMock);

  return { event };
}

describe("Vesting schedule added", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext;
  let vestingSchedulesStored: VestingSchedule[];

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);
    vestingSchedulesStored = [];

    // Stub store.get() to return the vesting schedules in the database
    when(storeMock.get<VestingSchedule>(VestingSchedule, anything())).thenCall(
      () => {
        return Promise.resolve(vestingSchedulesStored);
      }
    );

    // Stub store.save() to update the vesting schedules in the database
    when(storeMock.save<VestingSchedule>(anything())).thenCall(
      (vestingSchedule) => {
        vestingSchedulesStored.push(vestingSchedule);
      }
    );
  });

  it("Should add block number based vesting schedule events", async () => {
    const { event } = createVestingScheduleAddedEvent(
      MOCK_ADDRESS_FROM,
      MOCK_ADDRESS_TO,
      2n,
      MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED
    );

    const vestingSchedule = getNewVestingSchedule(ctx, event);

    assertVestingSchedule(
      vestingSchedule,
      encodeAccount(MOCK_ADDRESS_FROM),
      encodeAccount(MOCK_ADDRESS_TO),
      undefined,
      2n,
      MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED,
      3n,
      false
    );
  });

  it("Should add moment based vesting schedule events", async () => {
    const { event } = createVestingScheduleAddedEvent(
      MOCK_ADDRESS_FROM,
      MOCK_ADDRESS_TO,
      5n,
      MOCK_VESTING_SCHEDULE_MOMENT_BASED
    );

    const vestingSchedule = getNewVestingSchedule(ctx, event);

    assertVestingSchedule(
      vestingSchedule,
      encodeAccount(MOCK_ADDRESS_FROM),
      encodeAccount(MOCK_ADDRESS_TO),
      undefined,
      5n,
      MOCK_VESTING_SCHEDULE_MOMENT_BASED,
      4n,
      false
    );
  });
});
