import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { StakingPosition } from "../src/model";
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
 * @param transactionId
 * @param duration
 */
function assertStakingPosition(
  position: StakingPosition,
  positionId: string,
  assetId: string,
  owner: string,
  amount: bigint,
  eventId: string,
  transactionId: string,
  duration: bigint
) {
  expect(position.positionId).to.equal(positionId);
  expect(position.assetId).to.equal(assetId);
  expect(position.owner).to.equal(owner);
  expect(position.amount).to.equal(amount);
  expect(position.eventId).to.equal(eventId);
  expect(position.transactionId).to.equal(transactionId);
  if (position.endTimestamp)
    expect(position.endTimestamp).to.equal(
      position.startTimestamp + 1_000n * duration
    );
}

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
      "event-id",
      "transaction-id",
      1662133770000n
    );

    assertStakingPosition(
      position,
      "2",
      "3",
      BOB,
      123n,
      "event-id",
      "transaction-id",
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
      "event-id",
      "transaction-id",
      1662133770000n
    );
    const newPosition = splitStakingPosition(
      position,
      100n,
      50n,
      4n,
      "new-event-id",
      "new-transaction-id"
    );

    assertStakingPosition(
      position,
      "2",
      "3",
      BOB,
      100n,
      "new-event-id",
      "new-transaction-id",
      10n
    );
    assertStakingPosition(
      newPosition,
      "4",
      "3",
      BOB,
      50n,
      "new-event-id",
      "new-transaction-id",
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
      "event-id",
      "transaction-id",
      1662133770000n
    );
    extendStakingPosition(position, 150n, "new-event-id", "new-transaction-id");

    assertStakingPosition(
      position,
      "2",
      "3",
      BOB,
      150n,
      "new-event-id",
      "new-transaction-id",
      10n
    );
  });
});
