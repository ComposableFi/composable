import { SubstrateProcessor } from "@subsquid/substrate-processor";
import { TypeormDatabase } from "@subsquid/typeorm-store";
import * as dotenv from "dotenv"; // see https://github.com/motdotla/dotenv#how-do-i-use-dotenv-with-import
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

dotenv.config();

const processor = new SubstrateProcessor(new TypeormDatabase());

const chain = (): string => {
  switch (process.env.ENV) {
    case "dali":
      return "wss://dali.devnets.composablefinance.ninja/parachain/alice";
    case "dali-stage":
      return "wss://dali-cluster-fe.composablefinance.ninja";
    default:
      if ("RELAYCHAIN_URI" in process.env) {
        return process.env.RELAYCHAIN_URI!.toString();
      }

      return "ws://127.0.0.1:9988";
  }
};

const archive = (): string => {
  if ("SUBSQUID_ARCHIVE_URI" in process.env) {
    return process.env.SUBSQUID_ARCHIVE_URI!.toString();
  }

  return "http://127.0.0.1:8888/graphql";
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

processor.addEventHandler("Pablo.PoolCreated", async (ctx) => {
  const event = new PabloPoolCreatedEvent(ctx);
  await processPoolCreatedEvent(ctx, event);
});

processor.addEventHandler("Pablo.PoolDeleted", async (ctx) => {
  const event = new PabloPoolDeletedEvent(ctx);
  await processPoolDeletedEvent(ctx, event);
});

processor.addEventHandler("Pablo.LiquidityAdded", async (ctx) => {
  const event = new PabloLiquidityAddedEvent(ctx);
  await processLiquidityAddedEvent(ctx, event);
});

processor.addEventHandler("Pablo.LiquidityRemoved", async (ctx) => {
  const event = new PabloLiquidityRemovedEvent(ctx);
  await processLiquidityRemovedEvent(ctx, event);
});

processor.addEventHandler("Pablo.Swapped", async (ctx) => {
  const event = new PabloSwappedEvent(ctx);
  await processSwappedEvent(ctx, event);
});

processor.addEventHandler("Balances.Transfer", processTransferEvent);

processor.addEventHandler("Balances.Withdraw", processWithdrawEvent);

processor.addEventHandler("Balances.Deposit", processDepositEvent);

processor.addEventHandler("BondedFinance.NewOffer", processNewOfferEvent);

processor.addEventHandler("BondedFinance.NewBond", processNewBondEvent);

processor.addEventHandler(
  "BondedFinance.OfferCancelled",
  processOfferCancelledEvent
);

processor.addEventHandler(
  "Vesting.VestingScheduleAdded",
  processVestingScheduleAddedEvent
);

processor.addEventHandler("Vesting.Claimed", processVestingClaimedEvent);

processor.addEventHandler(
  "StakingRewards.RewardPoolCreated",
  processRewardPoolCreatedEvent
);

processor.addEventHandler("StakingRewards.Staked", processStakedEvent);

processor.addEventHandler(
  "StakingRewards.StakeAmountExtended",
  processStakeAmountExtendedEvent
);

processor.addEventHandler("StakingRewards.Unstaked", processUnstakedEvent);

processor.addEventHandler(
  "StakingRewards.SplitPosition",
  processSplitPositionEvent
);

processor.addEventHandler("Oracle.PriceChanged", processOraclePriceChanged);

processor.run();
