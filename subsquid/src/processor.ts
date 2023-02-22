import { SubstrateProcessor } from "@subsquid/substrate-processor";
import { TypeormDatabase } from "@subsquid/typeorm-store";

import { archive, chain, firstBlock } from "./config";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent,
  processSwappedEvent
} from "./processors/pablo";
import { processDepositEvent, processTransferEvent, processWithdrawEvent } from "./processors/balances";
import { processNewBondEvent, processNewOfferEvent, processOfferCancelledEvent } from "./processors/bondedFinance";
import { processVestingClaimedEvent, processVestingScheduleAddedEvent } from "./processors/vestingSchedule";

const processor = new SubstrateProcessor(new TypeormDatabase());

const chainConnectionString = chain();
const archiveConnectionString = archive();

processor.setBlockRange({
  // from: firstBlock()
  // Pool creation events
  from: 1407898
  // Swap events
  // from: 1444112
});

console.log(`Chain ${chainConnectionString}`);
console.log(`Archive ${archiveConnectionString}`);

processor.setBatchSize(500);
processor.setDataSource({
  archive: archiveConnectionString,
  chain: chainConnectionString
});

processor.addEventHandler("Pablo.PoolCreated", async ctx => {
  await processPoolCreatedEvent(ctx);
});

processor.addEventHandler("Pablo.LiquidityAdded", async ctx => {
  await processLiquidityAddedEvent(ctx);
});

processor.addEventHandler("Pablo.LiquidityRemoved", async ctx => {
  await processLiquidityRemovedEvent(ctx);
});

processor.addEventHandler("Pablo.Swapped", async ctx => {
  await processSwappedEvent(ctx);
});

processor.addEventHandler("Balances.Transfer", processTransferEvent);

processor.addEventHandler("Balances.Withdraw", processWithdrawEvent);

processor.addEventHandler("Balances.Deposit", processDepositEvent);

processor.addEventHandler("BondedFinance.NewOffer", processNewOfferEvent);

processor.addEventHandler("BondedFinance.NewBond", processNewBondEvent);

processor.addEventHandler("BondedFinance.OfferCancelled", processOfferCancelledEvent);

processor.addEventHandler("Vesting.VestingScheduleAdded", processVestingScheduleAddedEvent);

processor.addEventHandler("Vesting.Claimed", processVestingClaimedEvent);

// processor.addEventHandler(
//   "StakingRewards.RewardPoolCreated",
//   processRewardPoolCreatedEvent
// );
//
// processor.addEventHandler("StakingRewards.Staked", processStakedEvent);
//
// processor.addEventHandler(
//   "StakingRewards.StakeAmountExtended",
//   processStakeAmountExtendedEvent
// );
//
// processor.addEventHandler("StakingRewards.Unstaked", processUnstakedEvent);
//
// processor.addEventHandler(
//   "StakingRewards.SplitPosition",
//   processSplitPositionEvent
// );
//
// processor.addEventHandler("Oracle.PriceChanged", processOraclePriceChanged);

// processor.addEventHandler(
//   "AssetsRegistry.AssetRegistered",
//   processAssetRegisteredEvent
// );
//
// processor.addEventHandler(
//   "AssetsRegistry.AssetUpdated",
//   processAssetUpdatedEvent
// );

processor.run();
