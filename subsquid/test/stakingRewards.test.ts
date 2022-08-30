import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { PicassoStakingPosition } from "../src/model";
import { mock } from "ts-mockito";
import { BOB, createCtx } from "../src/utils";
import { expect } from "chai";
import {
  createPicassoStakingPosition,
  extendPicassoStakingPosition,
  splitPicassoStakingPosition,
} from "../src/processors/stakingRewards";

/**
 * Check if PicassoStakingPosition has expected values.
 * @param position
 * @param poolId
 * @param positionId
 * @param owner
 * @param amount
 * @param startTimestamp
 * @param endTimestamp
 * @param eventId
 * @param transactionId
 */
function assertPicassoStakingPosition(
  position: PicassoStakingPosition,
  poolId: string,
  positionId: string,
  owner: string,
  amount: bigint,
  startTimestamp: bigint,
  endTimestamp: bigint,
  eventId: string,
  transactionId: string
) {
  expect(position.poolId).to.equal(poolId);
  expect(position.positionId).to.equal(positionId);
  expect(position.owner).to.equal(owner);
  expect(position.amount).to.equal(amount);
  expect(position.startTimestamp).to.equal(startTimestamp);
  expect(position.endTimestamp).to.equal(endTimestamp);
  expect(position.eventId).to.equal(eventId);
  expect(position.transactionId).to.equal(transactionId);
}

describe("Staking rewards", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext;
  let now = BigInt(new Date().valueOf());
  let end = now + 10_000n;

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);
    now = BigInt(new Date().valueOf());
    end = now + 10_000n;
  });

  it("Should create PicassoStakingPosition", async () => {
    const position = createPicassoStakingPosition(
      2,
      3n,
      BOB,
      123n,
      10n,
      "event-id",
      "transaction-id"
    );

    assertPicassoStakingPosition(
      position,
      "2",
      "3",
      BOB,
      123n,
      now,
      end,
      "event-id",
      "transaction-id"
    );
  });

  it("Should split PicassoStakingPosition", async () => {
    const position = createPicassoStakingPosition(
      2,
      3n,
      BOB,
      123n,
      10n,
      "event-id",
      "transaction-id"
    );
    const newPosition = splitPicassoStakingPosition(
      position,
      100n,
      50n,
      4n,
      "new-event-id",
      "new-transaction-id"
    );

    assertPicassoStakingPosition(
      position,
      "2",
      "3",
      BOB,
      100n,
      now,
      end,
      "new-event-id",
      "new-transaction-id"
    );
    assertPicassoStakingPosition(
      newPosition,
      "2",
      "4",
      BOB,
      50n,
      now,
      end,
      "new-event-id",
      "new-transaction-id"
    );
  });

  it("Should extend PicassoStakingPosition", async () => {
    const position = createPicassoStakingPosition(
      2,
      3n,
      BOB,
      123n,
      10n,
      "event-id",
      "transaction-id"
    );
    extendPicassoStakingPosition(
      position,
      150n,
      "new-event-id",
      "new-transaction-id"
    );

    assertPicassoStakingPosition(
      position,
      "2",
      "3",
      BOB,
      150n,
      now,
      end,
      "new-event-id",
      "new-transaction-id"
    );
  });
});
