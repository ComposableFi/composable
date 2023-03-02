import { TypeormDatabase } from "@subsquid/typeorm-store";
import { processor, Context } from "./processorTypes";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent,
  processSwappedEvent,
  processSwapCallError,
  processAddLiquidityCallError,
  processRemoveLiquidityCallError
} from "./processors/pablo";
import { processDepositEvent, processTransferEvent, processWithdrawEvent } from "./processors/balances";
import { processVestingClaimedEvent, processVestingScheduleAddedEvent } from "./processors/vestingSchedule";
import { processNewBondEvent, processNewOfferEvent, processOfferCancelledEvent } from "./processors/bondedFinance";
import { PabloAddLiquidityCall, PabloBuyCall, PabloRemoveLiquidityCall, PabloSwapCall } from "./types/calls";

processor.run(new TypeormDatabase(), async (ctx: Context) => {
  for (const block of ctx.blocks) {
    for (const item of block.items) {
      if (item.kind === "event") {
        if (item.name === "Balances.Transfer") {
          await processTransferEvent(ctx, block, item);
        } else if (item.name === "Balances.Deposit") {
          await processDepositEvent(ctx, block, item);
        } else if (item.name === "Balances.Withdraw") {
          await processWithdrawEvent(ctx, block, item);
        } else if (item.name === "Vesting.VestingScheduleAdded") {
          await processVestingScheduleAddedEvent(ctx, block, item);
        } else if (item.name === "Vesting.Claimed") {
          await processVestingClaimedEvent(ctx, block, item);
        } else if (item.name === "Pablo.PoolCreated") {
          await processPoolCreatedEvent(ctx, block, item);
        } else if (item.name === "Pablo.LiquidityAdded") {
          await processLiquidityAddedEvent(ctx, block, item);
        } else if (item.name === "Pablo.LiquidityRemoved") {
          await processLiquidityRemovedEvent(ctx, block, item);
        } else if (item.name === "Pablo.Swapped") {
          await processSwappedEvent(ctx, block, item);
        } else if (item.name === "BondedFinance.NewOffer") {
          await processNewOfferEvent(ctx, block, item);
        } else if (item.name === "BondedFinance.NewBond") {
          await processNewBondEvent(ctx, block, item);
        } else if (item.name === "BondedFinance.OfferCancelled") {
          await processOfferCancelledEvent(ctx, block, item);
        }
      } else if (item.kind === "call" && !item.call.success) {
        if (item.name === "Pablo.add_liquidity") {
          const call = new PabloAddLiquidityCall(ctx, item.call);
          await processAddLiquidityCallError(ctx, block, item, call);
        } else if (item.name === "Pablo.remove_liquidity") {
          const call = new PabloRemoveLiquidityCall(ctx, item.call);
          await processRemoveLiquidityCallError(ctx, block, item, call);
        } else if (item.name === "Pablo.swap") {
          const call = new PabloSwapCall(ctx, item.call);
          await processSwapCallError(ctx, block, item, call);
        }
      }
    }
  }
});
