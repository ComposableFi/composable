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
import { VestingSchedule as VestingScheduleType } from "../src/types/v2300";
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
  periodCount: 1,
  perPeriod: BigInt(100),
};

function assertVestingSchedule(
  vestingSchedule: VestingSchedule,
  beneficiary: string,
  schedule: Schedule
) {
  expect(vestingSchedule.beneficiary).to.equal(beneficiary);
  expect(vestingSchedule.schedule).to.deep.equal(schedule);
}

async function assertVestingScheduleAddedEvent(
  ctx: EventHandlerContext,
  storeMock: Store,
  beneficiary: Uint8Array,
  schedule: Schedule
) {
  // Assert last save
  const [arg] = capture(storeMock.save).last();
  assertVestingSchedule(
    arg as unknown as VestingSchedule,
    encodeAccount(beneficiary),
    schedule
  );
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
  };

  when(eventMock.asV2300).thenReturn(evt);
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
      vestingSchedule
    );

    await processVestingScheduleAddedEvent(ctx, event);

    const schedule = createVestingSchedule(vestingSchedule);

    await assertVestingScheduleAddedEvent(
      ctx,
      storeMock,
      MOCK_ADDRESS_TO,
      schedule
    );

    verify(storeMock.save(anyOfClass(VestingSchedule))).times(1);
  });
});
