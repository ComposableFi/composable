import { SubstrateProcessor } from "@subsquid/substrate-processor";
import {
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloPoolDeletedEvent,
  PabloSwappedEvent,
} from "./types/events";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent,
  processPoolDeletedEvent,
  processSwappedEvent,
} from "./processors/pablo";
import {
  processRewardPoolCreatedEvent,
  processSplitPositionEvent,
  processStakeAmountExtendedEvent,
  processStakedEvent,
  processUnstakedEvent,
} from "./processors/stakingRewards";
import {
  processTransferEvent,
  processDepositEvent,
  processWithdrawEvent,
} from "./processors/balances";
import {
  processNewBondEvent,
  processNewOfferEvent,
  processOfferCancelledEvent,
} from "./processors/bondedFinance";
import {
  processVestingClaimedEvent,
  processVestingScheduleAddedEvent,
} from "./processors/vestingSchedule";
import { processOraclePriceChanged } from "./processors/oracle";

const processor = new SubstrateProcessor("composable_dali_dev");

const chain = (): string => {
  switch (process.env.ENV) {
    case "dali":
      return "wss://dali.devnets.composablefinance.ninja/parachain/alice";
    case "dali-stage":
      return "wss://dali-cluster-fe.composablefinance.ninja";
    default:
      if ("RELAYCHAIN_URI" in process.env) {
        return process.env.RELAYCHAIN_URI!.toString();
      } else {
        return "ws://127.0.0.1:9988";
      }
  }
};

const archive = (): string => {
  if ("SUBSQUID_ARCHIVE_URI" in process.env) {
    return process.env.SUBSQUID_ARCHIVE_URI!.toString();
  } else {
    return "http://127.0.0.1:8080/v1/graphql";
  }
};

const chainConnectionString = chain();
const archiveConnectionString = archive();

console.log(`Chain ${chainConnectionString}`);
console.log(`Archive ${archiveConnectionString}`);

processor.setBatchSize(500);
processor.setDataSource({
  archive: archiveConnectionString,
  chain: chainConnectionString,
});

processor.addEventHandler("pablo.PoolCreated", async (ctx) => {
  const event = new PabloPoolCreatedEvent(ctx);
  await processPoolCreatedEvent(ctx, event);
});

processor.addEventHandler("pablo.PoolDeleted", async (ctx) => {
  const event = new PabloPoolDeletedEvent(ctx);
  await processPoolDeletedEvent(ctx, event);
});

processor.addEventHandler("pablo.LiquidityAdded", async (ctx) => {
  const event = new PabloLiquidityAddedEvent(ctx);
  await processLiquidityAddedEvent(ctx, event);
});

processor.addEventHandler("pablo.LiquidityRemoved", async (ctx) => {
  const event = new PabloLiquidityRemovedEvent(ctx);
  await processLiquidityRemovedEvent(ctx, event);
});

processor.addEventHandler("pablo.Swapped", async (ctx) => {
  const event = new PabloSwappedEvent(ctx);
  await processSwappedEvent(ctx, event);
});

processor.addEventHandler("balances.Transfer", processTransferEvent);

processor.addEventHandler("balances.Withdraw", processWithdrawEvent);

processor.addEventHandler("balances.Deposit", processDepositEvent);

processor.addEventHandler("bondedFinance.NewOffer", processNewOfferEvent);

processor.addEventHandler("bondedFinance.NewBond", processNewBondEvent);

processor.addEventHandler(
  "bondedFinance.OfferCancelled",
  processOfferCancelledEvent
);

processor.addEventHandler(
  "vesting.VestingScheduleAdded",
  processVestingScheduleAddedEvent
);

processor.addEventHandler("vesting.Claimed", processVestingClaimedEvent);

processor.addEventHandler(
  "stakingRewards.RewardPoolCreated",
  processRewardPoolCreatedEvent
);

processor.addEventHandler("stakingRewards.Staked", processStakedEvent);

processor.addEventHandler(
  "stakingRewards.StakeAmountExtended",
  processStakeAmountExtendedEvent
);

processor.addEventHandler("stakingRewards.Unstaked", processUnstakedEvent);

processor.addEventHandler(
  "stakingRewards.SplitPosition",
  processSplitPositionEvent
);

processor.addEventHandler("oracle.PriceChanged", processOraclePriceChanged);

processor.run();
