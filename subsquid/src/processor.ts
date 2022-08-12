import * as ss58 from "@subsquid/ss58";
import {
  EventHandlerContext,
  SubstrateProcessor,
} from "@subsquid/substrate-processor";
import { Account, HistoricalBalance } from "./model";
import {
  BalancesTransferEvent,
  PabloLiquidityAddedEvent,
  PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent,
  PabloPoolDeletedEvent,
  PabloSwappedEvent,
  BondedFinanceNewBondEvent,
  BondedFinanceNewOfferEvent,
  VestingVestingScheduleAddedEvent,
} from "./types/events";
import { getOrCreate } from "./dbHelper";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent,
  processPoolDeletedEvent,
  processSwappedEvent,
} from "./pabloProcessor";
import {
  processRewardPoolCreatedEvent,
  processSplitPositionEvent,
  processStakeAmountExtendedEvent,
  processStakedEvent,
  processUnstakedEvent,
} from "./picassoProcessor";
import {
  processTransferEvent,
  processDepositEvent,
  processWithdrawEvent,
} from "./balancesProcessor";
import {
  processNewBondEvent,
  processNewOfferEvent,
} from "./bondedFinanceProcessor";
import { processVestingScheduleAddedEvent } from "./vestingProcessor";

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
      }
      else {
        return "ws://127.0.0.1:9988";
      }
  }
};

const archive = (): string => {
  if ("SUBSQUID_ARCHIVE_URI" in process.env) {
    return process.env.SUBSQUID_ARCHIVE_URI!.toString();
  }
  else {
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

processor.addEventHandler("balances.Transfer", async (ctx) => {
  await processTransferEvent(ctx);
});

processor.addEventHandler("balances.Withdraw", async (ctx) => {
  await processWithdrawEvent(ctx);
});

processor.addEventHandler("balances.Deposit", async (ctx) => {
  await processDepositEvent(ctx);
});

processor.addEventHandler("bondedFinance.NewOffer", async (ctx) => {
  const event = new BondedFinanceNewOfferEvent(ctx);

  await processNewOfferEvent(ctx, event);
  // TODO: process event for Picasso
});

processor.addEventHandler("bondedFinance.NewBond", async (ctx) => {
  const event = new BondedFinanceNewBondEvent(ctx);

  await processNewBondEvent(ctx, event);
  // TODO: process event for Picasso
});

processor.addEventHandler("bondedFinance.OfferCancelled", async (ctx) => {
  // await processOfferCancelledEvent(ctx);
  // TODO
});

processor.addEventHandler("vesting.VestingScheduleAdded", async (ctx) => {
  const event = new VestingVestingScheduleAddedEvent(ctx);

  await processVestingScheduleAddedEvent(ctx, event);
});

processor.addEventHandler("stakingRewards.RewardPoolCreated", async (ctx) => {
  await processRewardPoolCreatedEvent(ctx);
});

processor.addEventHandler("stakingRewards.Staked", async (ctx) => {
  await processStakedEvent(ctx);
});

processor.addEventHandler("stakingRewards.StakeAmountExtended", async (ctx) => {
  await processStakeAmountExtendedEvent(ctx);
});

processor.addEventHandler("stakingRewards.Unstaked", async (ctx) => {
  await processUnstakedEvent(ctx);
});

processor.addEventHandler("stakingRewards.SplitPosition", async (ctx) => {
  await processSplitPositionEvent(ctx);
});

processor.run();

interface TransferEvent {
  from: Uint8Array;
  to: Uint8Array;
  amount: bigint;
}

function getTransferEvent(ctx: EventHandlerContext): TransferEvent {
  const event = new BalancesTransferEvent(ctx);
  return event.asV2401 ?? event.asLatest;
}
