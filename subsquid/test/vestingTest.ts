import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { Schedule, VestingSchedule } from "../src/model";
import {
  anyOfClass,
  anything,
  capture,
  instance,
  mock,
  verify,
  when,
} from "ts-mockito";
import { createAccount, createCtx, encodeAccount } from "../src/utils";
import {
  createVestingSchedule,
  processVestingScheduleAddedEvent,
} from "../src/vestingProcessor";
import { VestingSchedule as VestingScheduleType } from "../src/types/v2401";
import { VestingVestingScheduleAddedEvent } from "../src/types/events";
import { expect } from "chai";

const MOCK_ADDRESS_FROM = createAccount();
const MOCK_ADDRESS_TO = createAccount();

const MOCK_VESTING_SCHEDULE: VestingScheduleType = {
  window: {
    start: 1,
    period: 10,
    __kind: "BlockNumberBased",
  },
  vestingScheduleId: BigInt(1),
  alreadyClaimed: BigInt(0),
  periodCount: 1,
  perPeriod: BigInt(100),
};

/**
 * Check if vesting schedule has expected values
 * @param vestingSchedule
 * @param to
 * @param eventId
 * @param assetId
 * @param schedule
 */
function assertVestingSchedule(
  vestingSchedule: VestingSchedule,
  to: string,
  from: string,
  eventId: string,
  assetId: string,
  schedule: Schedule
) {
  const expectedScheduleId = `${to}-${assetId}`;
  expect(vestingSchedule.from).to.equal(from);
  expect(vestingSchedule.eventId).to.equal(eventId);
  expect(vestingSchedule.scheduleId).to.equal(expectedScheduleId);
  expect(vestingSchedule.to).to.equal(to);
  expect(vestingSchedule.schedule).to.deep.equal(schedule);
}

async function assertVestingScheduleAddedEvent(
  ctx: EventHandlerContext,
  storeMock: Store,
  from: Uint8Array,
  to: Uint8Array,
  assetId: string,
  schedule: Schedule
) {
  // Assert last save
  const [arg] = capture(storeMock.save).last();
  assertVestingSchedule(
    arg as unknown as VestingSchedule,
    encodeAccount(to),
    encodeAccount(from),
    ctx.event.id,
    assetId,
    schedule
  );
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

  it("Should add vesting schedule events correctly", async () => {
    const vestingSchedule = MOCK_VESTING_SCHEDULE;

    const { event } = createVestingScheduleAddedEvent(
      MOCK_ADDRESS_FROM,
      MOCK_ADDRESS_TO,
      BigInt(2),
      vestingSchedule,
      BigInt(1)
    );

    await processVestingScheduleAddedEvent(ctx, event);

    const schedule = createVestingSchedule(vestingSchedule);

    await assertVestingScheduleAddedEvent(
      ctx,
      storeMock,
      MOCK_ADDRESS_FROM,
      MOCK_ADDRESS_TO,
      event.asV2401.asset.toString(),
      schedule
    );

    verify(storeMock.save(anyOfClass(VestingSchedule))).times(1);
  });
});
