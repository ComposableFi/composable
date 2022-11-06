import { SubstrateProcessor } from "@subsquid/substrate-processor";
import { TypeormDatabase } from "@subsquid/typeorm-store";
import { archive, chain } from "./config";
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
  processDepositEvent,
  processTransferEvent,
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
import {
  processAssetRegisteredEvent,
  processAssetUpdatedEvent,
} from "./processors/assetsRegistry";

const processor = new SubstrateProcessor(new TypeormDatabase());

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

processor.addEventHandler(
  "AssetsRegistry.AssetRegistered",
  processAssetRegisteredEvent
);

processor.addEventHandler(
  "AssetsRegistry.AssetUpdated",
  processAssetUpdatedEvent
);

processor.run();
