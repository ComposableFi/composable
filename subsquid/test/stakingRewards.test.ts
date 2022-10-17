import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { mock } from "ts-mockito";
import { expect } from "chai";
import { Event, EventType, LockedSource, StakingPosition } from "../src/model";
import { BOB, createCtx } from "../src/utils";
import {
  createRewardPool,
  createStakingPosition,
  updateStakingPositionAmount,
  splitStakingPosition,
} from "../src/processors/stakingRewards";

/**
 * Check if StakingPosition has expected values.
 * @param position
 * @param fnftCollectionId
 * @param fnftInstanceId
 * @param assetId
 * @param owner
 * @param amount
 * @param eventId
 * @param eventType
 * @param duration
 * @param rewardMultiplier
 */
function assertStakingPosition(
  position: StakingPosition,
  fnftCollectionId: string,
  fnftInstanceId: string,
  assetId: string,
  owner: string,
  amount: bigint,
  eventId: string,
  eventType: EventType,
  duration: bigint,
  rewardMultiplier: number
) {
  expect(position.fnftCollectionId).to.equal(fnftCollectionId);
  expect(position.fnftInstanceId).to.equal(fnftInstanceId);
  expect(position.assetId).to.equal(assetId);
  expect(position.owner).to.equal(owner);
  expect(position.amount).to.equal(amount);
  expect(position.event.id).to.equal(eventId);
  expect(position.event.eventType).to.equal(eventType);
  if (position.endTimestamp)
    expect(position.endTimestamp).to.equal(
      position.startTimestamp + 1_000n * duration
    );
  expect(position.duration).to.equal(duration);
  expect(position.source).to.equal(LockedSource.StakingRewards);
  expect(position.rewardMultiplier).to.equal(rewardMultiplier);
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
  let ctx: EventHandlerContext<Store>;

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);
  });

  it("Should create RewardPool", () => {
    const rewardPool = createRewardPool("event-id", 1n);

    expect(rewardPool.eventId).to.equal("event-id");
    expect(rewardPool.poolId).to.equal("1");
  });

  it("Should create StakingPosition", () => {
    const position = createStakingPosition(
      1n,
      2n,
      "3",
      BOB,
      123n,
      10n,
      456,
      createMockEvent("event-id", EventType.STAKING_REWARDS_STAKED),
      1662133770000n
    );

    assertStakingPosition(
      position,
      "1",
      "2",
      "3",
      BOB,
      123n,
      "event-id",
      EventType.STAKING_REWARDS_STAKED,
      10n,
      456
    );
  });

  it("Should split StakingPosition", () => {
    const position = createStakingPosition(
      1n,
      2n,
      "3",
      BOB,
      123n,
      10n,
      456,
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
      "1",
      "2",
      "3",
      BOB,
      100n,
      "event-id",
      EventType.STAKING_REWARDS_SPLIT_POSITION,
      10n,
      456
    );
    assertStakingPosition(
      newPosition,
      "1",
      "4",
      "3",
      BOB,
      50n,
      "new-event-id",
      EventType.STAKING_REWARDS_SPLIT_POSITION,
      10n,
      456
    );
  });

  it("Should extend StakingPosition", () => {
    const position = createStakingPosition(
      1n,
      2n,
      "3",
      BOB,
      123n,
      10n,
      456,
      createMockEvent(
        "event-id",
        EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED
      ),
      1662133770000n
    );
    updateStakingPositionAmount(
      position,
      150n,
      createMockEvent(
        "new-event-id",
        EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED
      )
    );

    assertStakingPosition(
      position,
      "1",
      "2",
      "3",
      BOB,
      150n,
      "new-event-id",
      EventType.STAKING_REWARDS_STAKE_AMOUNT_EXTENDED,
      10n,
      456
    );
  });
});
