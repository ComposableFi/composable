import {
  BatchBlock,
  BatchContext,
  BatchProcessorCallItem,
  BatchProcessorEventItem,
  BatchProcessorItem,
  SubstrateBatchProcessor
} from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { archive, chain, firstBlock } from "./config";

console.log(`Chain ${chain()}`);
console.log(`Archive ${archive()}`);

export const processor = new SubstrateBatchProcessor()
  .setDataSource({
    chain: chain(),
    archive: archive()
  })
  .setBlockRange({
    from: firstBlock()
    // from: 1823182,
    // from: 1720846,
    // from: 1823114,
    // to: 1823114
    // from: 1448591, // remove liquidity
    // from: 1449221, // add liquidity
    // from: 1408311,
    // to: 1823182
  })
  .addEvent("Pablo.PoolCreated", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Pablo.LiquidityAdded", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Pablo.LiquidityRemoved", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Pablo.Swapped", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Balances.Transfer", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Balances.Withdraw", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Balances.Deposit", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("BondedFinance.NewOffer", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("BondedFinance.NewBond", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("BondedFinance.OfferCancelled", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Vesting.VestingScheduleAdded", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("Vesting.Claimed", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addEvent("*", {
    data: { event: { extrinsic: true, args: true } }
  } as const)
  .addCall("Pablo.add_liquidity", {
    data: { call: { args: true, error: true }, extrinsic: true }
  })
  .addCall("Pablo.remove_liquidity", {
    data: { call: { args: true, error: true }, extrinsic: true }
  })
  .addCall("Pablo.swap", {
    data: { call: { args: true, error: true }, extrinsic: true }
  })
  .addCall("Pablo.buy", {
    data: { call: { args: true, error: true }, extrinsic: true }
  });

export type Item = BatchProcessorItem<typeof processor>;
export type EventItem = BatchProcessorEventItem<typeof processor>;
export type CallItem = BatchProcessorCallItem<typeof processor>;
export type Context = BatchContext<Store, Item>;
export type Block = BatchBlock<Item>;
