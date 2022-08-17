import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { VestingSchedule } from "../src/model";
import { anything, instance, mock, when } from "ts-mockito";
import { createAccount, createCtx, encodeAccount } from "../src/utils";
import { getNewVestingSchedule } from "../src/vestingProcessor";
import { VestingSchedule as VestingScheduleType } from "../src/types/v2401";
import { VestingVestingScheduleAddedEvent } from "../src/types/events";
import { expect } from "chai";

const MOCK_ADDRESS_FROM = createAccount();
const MOCK_ADDRESS_TO = createAccount();

const MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED: VestingScheduleType = {
  window: {
    start: 1,
    period: 10,
    __kind: "BlockNumberBased",
  },
  vestingScheduleId: BigInt(1),
  alreadyClaimed: BigInt(0),
  periodCount: 1,
  perPeriod: 100n,
};

const MOCK_VESTING_SCHEDULE_MOMENT_BASED: VestingScheduleType = {
  window: {
    start: 1n,
    period: 10n,
    __kind: "MomentBased",
  },
  vestingScheduleId: BigInt(1),
  alreadyClaimed: BigInt(0),
  periodCount: 1,
  perPeriod: 100n,
};

/**
 * Check if vesting schedule has expected values
 * @param vestingSchedule
 * @param from
 * @param to
 * @param eventId
 * @param assetId
 * @param schedule
 */
function assertVestingSchedule(
  vestingSchedule: VestingSchedule,
  from?: string,
  to?: string,
  eventId?: string,
  assetId?: string,
  schedule?: VestingScheduleType
) {
  const expectedScheduleId = `${to}-${assetId}`;
  if (from) expect(vestingSchedule.from).to.equal(from);
  if (to) expect(vestingSchedule.to).to.equal(to);
  if (eventId) expect(vestingSchedule.eventId).to.equal(eventId);
  if (from && assetId)
    expect(vestingSchedule.scheduleId).to.equal(expectedScheduleId);
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
  }
}

function createVestingScheduleAddedEvent(
  from: Uint8Array,
  to: Uint8Array,
  asset: bigint,
  schedule: VestingScheduleType,
  vestingScheduleId: bigint
) {
  let eventMock = mock(VestingVestingScheduleAddedEvent);
  let evt = {
    from,
    to,
    asset,
    schedule,
    vestingScheduleId
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
      BigInt(2),
      MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED,
      BigInt(1),
    );

    const vestingSchedule = getNewVestingSchedule(ctx, event);

    assertVestingSchedule(
      vestingSchedule,
      encodeAccount(MOCK_ADDRESS_FROM),
      encodeAccount(MOCK_ADDRESS_TO),
      undefined,
      undefined,
      MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED
    );
  });

  it("Should add moment based vesting schedule events", async () => {
    const { event } = createVestingScheduleAddedEvent(
      MOCK_ADDRESS_FROM,
      MOCK_ADDRESS_TO,
      BigInt(2),
      MOCK_VESTING_SCHEDULE_MOMENT_BASED,
      BigInt(1),
    );

    const vestingSchedule = getNewVestingSchedule(ctx, event);

    assertVestingSchedule(
      vestingSchedule,
      encodeAccount(MOCK_ADDRESS_FROM),
      encodeAccount(MOCK_ADDRESS_TO),
      undefined,
      undefined,
      MOCK_VESTING_SCHEDULE_MOMENT_BASED
    );
  });
});
