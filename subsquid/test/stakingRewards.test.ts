import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { Event, EventType, StakingPosition, StakingSource } from "../src/model";
import { mock } from "ts-mockito";
import { BOB, createCtx } from "../src/utils";
import { expect } from "chai";
import {
  createRewardPool,
  createStakingPosition,
  extendStakingPosition,
  splitStakingPosition,
} from "../src/processors/stakingRewards";

/**
 * Check if StakingPosition has expected values.
 * @param position
 * @param positionId
 * @param assetId
 * @param owner
 * @param amount
 * @param eventId
 * @param eventType
 * @param duration
 */
function assertStakingPosition(
  position: StakingPosition,
  positionId: string,
  assetId: string,
  owner: string,
  amount: bigint,
  eventId: string,
  eventType: EventType,
  duration: bigint
) {
  expect(position.positionId).to.equal(positionId);
  expect(position.assetId).to.equal(assetId);
  expect(position.owner).to.equal(owner);
  expect(position.amount).to.equal(amount);
  expect(position.event.id).to.equal(eventId);
  expect(position.event.eventType).to.equal(eventType);
  if (position.endTimestamp)
    expect(position.endTimestamp).to.equal(
      position.startTimestamp + 1_000n * duration
    );
  expect(position.source).to.equal(StakingSource.StakingRewards);
}

const createMockEvent = (eventId: string, eventType: EventType) =>
  new Event({
    id: eventId,
    accountId: BOB,
    eventType,
    blockNumber: 1n,
    timestamp: 123n,
  });

describe("Staking rewards", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext;

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);
  });

  it("Should create RewardPool", async () => {
    const rewardPool = createRewardPool("event-id", 1n, 2n);

    expect(rewardPool.eventId).to.equal("event-id");
    expect(rewardPool.poolId).to.equal("1");
    expect(rewardPool.assetId).to.equal("2");
  });

  it("Should create StakingPosition", async () => {
    const position = createStakingPosition(
      "2",
      "3",
      BOB,
      123n,
      10n,
      createMockEvent("event-id", EventType.STAKING_REWARDS_STAKED),
      1662133770000n
    );

    assertStakingPosition(
      position,
      "2",
      "3",
      BOB,
      123n,
      "event-id",
      EventType.STAKING_REWARDS_STAKED,
      10n
    );
  });

  it("Should split StakingPosition", async () => {
    const position = createStakingPosition(
      "2",
      "3",
      BOB,
      123n,
      10n,
      createMockEvent("event-id", EventType.STAKING_REWARDS_SPLIT_POSITION),
      1662133770000n
    );
    const newPosition = splitStakingPosition(
      position,
      100n,
      50n,
      4n,
      createMockEvent("new-event-id", EventType.STAKING_REWARDS_SPLIT_POSITION)
    );

    assertStakingPosition(
      position,
      "2",
      "3",
      BOB,
      100n,
      "new-event-id",
      EventType.STAKING_REWARDS_SPLIT_POSITION,
      10n
    );
    assertStakingPosition(
      newPosition,
      "4",
      "3",
      BOB,
      50n,
      "new-event-id",
      EventType.STAKING_REWARDS_SPLIT_POSITION,
      10n
    );
  });

  it("Should extend StakingPosition", async () => {
    const position = createStakingPosition(
      "2",
      "3",
      BOB,
      123n,
      10n,
      createMockEvent(
        "event-id",
        EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED
      ),
      1662133770000n
    );
    extendStakingPosition(
      position,
      150n,
      createMockEvent(
        "new-event-id",
        EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED
      )
    );

    assertStakingPosition(
      position,
      "2",
      "3",
      BOB,
      150n,
      "new-event-id",
      EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED,
      10n
    );
  });
});
