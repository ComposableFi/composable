import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { anything, instance, mock, when } from "ts-mockito";
import { expect } from "chai";
import { Schedule, ScheduleWindow, VestingSchedule } from "../src/model";
import {
  BOB,
  CHARLIE,
  createAccount,
  createCtx,
  encodeAccount,
} from "../src/utils";
import {
  createSchedule,
  createVestingSchedule,
  updatedClaimedAmount,
} from "../src/processors/vestingSchedule";
import {
  VestingSchedule as VestingScheduleType,
  VestingScheduleIdSet,
} from "../src/types/v2402";
import {
  VestingClaimedEvent,
  VestingVestingScheduleAddedEvent,
} from "../src/types/events";

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

const createMockVestingSchedule = (
  scheduleId: bigint,
  assetId: string
): VestingSchedule =>
  new VestingSchedule({
    id: "123",
    scheduleId,
    from: BOB,
    eventId: "456",
    to: CHARLIE,
    assetId,
    schedule: new Schedule({
      vestingScheduleId: scheduleId,

      window: new ScheduleWindow({
        start: 1n,
        period: 10n,
        kind: "BlockNumberBased",
      }),
      periodCount: 1n,
      perPeriod: 100n,
      alreadyClaimed: 0n,
    }),
    totalAmount: 100n,
    fullyClaimed: false,
  });

/**
 * Check if vesting schedule has expected values
 * @param vestingSchedule
 * @param from
 * @param to
 * @param assetId
 * @param vestingScheduleId
 * @param alreadyClaimed
 * @param fullyClaimed
 * @param scheduleAmount
 * @param eventId
 * @param schedule
 */
function assertVestingSchedule(
  vestingSchedule: VestingSchedule,
  from: string,
  to: string,
  assetId: string,
  vestingScheduleId: bigint,
  alreadyClaimed: bigint,
  fullyClaimed: boolean,
  scheduleAmount: bigint,
  eventId?: string,
  schedule?: VestingScheduleType
) {
  expect(vestingSchedule.from).to.equal(from);
  expect(vestingSchedule.to).to.equal(to);

  expect(vestingSchedule.assetId).to.equal(assetId);
  expect(vestingSchedule.scheduleId).to.equal(vestingScheduleId);
  expect(vestingSchedule.schedule.alreadyClaimed).to.equal(alreadyClaimed);
  expect(vestingSchedule.fullyClaimed).to.equal(fullyClaimed);
  expect(vestingSchedule.totalAmount).to.equal(scheduleAmount);
  if (eventId) expect(vestingSchedule.eventId).to.equal(eventId);
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
}

function createVestingScheduleAddedEvent(
  from: Uint8Array,
  to: Uint8Array,
  assetId: string,
  schedule: VestingScheduleType,
  scheduleAmount: bigint
) {
  const eventMock = mock(VestingVestingScheduleAddedEvent);
  const evt = {
    from,
    to,
    asset: BigInt(assetId),
    schedule,
    vestingScheduleId: schedule.vestingScheduleId,
    alreadyClaimed: schedule.vestingScheduleId,
    scheduleAmount,
  };

  when(eventMock.asV2402).thenReturn(evt);

  const event = instance(eventMock);

  return { event };
}

function createVestingScheduleClaimedEvent(
  who: Uint8Array,
  asset: bigint,
  vestingScheduleIds: VestingScheduleIdSet,
  lockedAmount: bigint,
  claimedAmountPerSchedule: [bigint, bigint][]
) {
  const eventMock = mock(VestingClaimedEvent);
  const evt = {
    who,
    asset,
    vestingScheduleIds,
    lockedAmount,
    claimedAmountPerSchedule,
  };

  when(eventMock.asV2402).thenReturn(evt);

  const event = instance(eventMock);

  return { event };
}

describe("Vesting schedule added", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext<Store>;
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

  it("Should add block number based vesting schedule events", () => {
    const { event } = createVestingScheduleAddedEvent(
      MOCK_ADDRESS_FROM,
      MOCK_ADDRESS_TO,
      "2",
      MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED,
      100n
    );

    const vestingSchedule = createVestingSchedule(ctx, event);

    assertVestingSchedule(
      vestingSchedule,
      encodeAccount(MOCK_ADDRESS_FROM),
      encodeAccount(MOCK_ADDRESS_TO),
      "2",
      3n,
      10n,
      false,
      100n,
      undefined,
      MOCK_VESTING_SCHEDULE_BLOCK_NUMBER_BASED
    );
  });

  it("Should add moment based vesting schedule events", () => {
    const { event } = createVestingScheduleAddedEvent(
      MOCK_ADDRESS_FROM,
      MOCK_ADDRESS_TO,
      "5",
      MOCK_VESTING_SCHEDULE_MOMENT_BASED,
      100n
    );

    const vestingSchedule = createVestingSchedule(ctx, event);

    assertVestingSchedule(
      vestingSchedule,
      encodeAccount(MOCK_ADDRESS_FROM),
      encodeAccount(MOCK_ADDRESS_TO),
      "5",
      4n,
      20n,
      false,
      100n,
      undefined,
      MOCK_VESTING_SCHEDULE_MOMENT_BASED
    );
  });

  it("Should update all claimed schedules", () => {
    const { event } = createVestingScheduleClaimedEvent(
      MOCK_ADDRESS_FROM,
      5n,
      { __kind: "All" },
      50n,
      [
        [1n, 50n],
        [2n, 10n],
      ]
    );

    const { claimedAmountPerSchedule } = event.asV2402;

    for (let i = 0; i < claimedAmountPerSchedule.length; i += 1) {
      const [id, amount] = claimedAmountPerSchedule[i];

      const vestingSchedule = createMockVestingSchedule(id, "1");

      assertVestingSchedule(
        vestingSchedule,
        BOB,
        CHARLIE,
        "1",
        BigInt(id),
        BigInt(0),
        false,
        100n,
        "456",
        undefined
      );

      updatedClaimedAmount(vestingSchedule, amount);

      assertVestingSchedule(
        vestingSchedule,
        BOB,
        CHARLIE,
        "1",
        BigInt(id),
        BigInt(amount),
        false,
        100n,
        "456",
        undefined
      );
    }
  });

  it("Should fully claim schedule", () => {
    const { event } = createVestingScheduleClaimedEvent(
      MOCK_ADDRESS_FROM,
      5n,
      { __kind: "One", value: 3n },
      100n,
      [[3n, 100n]]
    );

    const { claimedAmountPerSchedule } = event.asV2402;
    const [id, amount] = claimedAmountPerSchedule[0];

    const vestingSchedule = createMockVestingSchedule(id, "1");

    assertVestingSchedule(
      vestingSchedule,
      BOB,
      CHARLIE,
      "1",
      BigInt(id),
      BigInt(0),
      false,
      100n,
      "456",
      undefined
    );

    updatedClaimedAmount(vestingSchedule, amount);

    assertVestingSchedule(
      vestingSchedule,
      BOB,
      CHARLIE,
      "1",
      BigInt(id),
      BigInt(amount),
      true,
      100n,
      "456",
      undefined
    );
  });
});
