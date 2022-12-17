import { SubstrateProcessor } from "@subsquid/substrate-processor";
import { TypeormDatabase } from "@subsquid/typeorm-store";

import { archive, chain } from "./config";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent,
  processSwappedEvent,
} from "./processors/pablo";
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
// import { processOraclePriceChanged } from "./processors/oracle";
import {
  processAssetRegisteredEvent,
  processAssetUpdatedEvent,
} from "./processors/assetsRegistry";

const processor = new SubstrateProcessor(new TypeormDatabase());

const chainConnectionString = chain();
const archiveConnectionString = archive();

// Start from a block close to this runtime upgrade from Picasso
// https://picasso.subscan.io/extrinsic/0xc875c8916e23c119f1d4202914dd0f28304aff62e46b0d51fed9b34e0aa30d9c
// const FIRST_BLOCK = chainConnectionString === "picasso" ? 1_227_000 : 0;
const FIRST_BLOCK = 0;

processor.setBlockRange({
  from: FIRST_BLOCK,
});

console.log(`Chain ${chainConnectionString}`);
console.log(`Archive ${archiveConnectionString}`);

processor.setBatchSize(500);
processor.setDataSource({
  archive: archiveConnectionString,
  chain: chainConnectionString,
});

processor.addEventHandler("Pablo.PoolCreated", async (ctx) => {
  await processPoolCreatedEvent(ctx);
});

processor.addEventHandler("Pablo.LiquidityAdded", async (ctx) => {
  await processLiquidityAddedEvent(ctx);
});

processor.addEventHandler("Pablo.LiquidityRemoved", async (ctx) => {
  await processLiquidityRemovedEvent(ctx);
});

processor.addEventHandler("Pablo.Swapped", async (ctx) => {
  await processSwappedEvent(ctx);
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

processor.addEventHandler(
  "AssetsRegistry.AssetRegistered",
  processAssetRegisteredEvent
);

processor.addEventHandler(
  "AssetsRegistry.AssetUpdated",
  processAssetUpdatedEvent
);

processor.run();
