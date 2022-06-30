import * as ss58 from "@subsquid/ss58";
import {
  EventHandlerContext,
  Store,
  SubstrateProcessor,
} from "@subsquid/substrate-processor";
import { lookupArchive } from "@subsquid/archive-registry";
import { Account, HistoricalBalance } from "./model";
import {
  BalancesTransferEvent,
  PabloLiquidityAddedEvent, PabloLiquidityRemovedEvent,
  PabloPoolCreatedEvent, PabloPoolDeletedEvent,
  PabloSwappedEvent
} from "./types/events";
import {getOrCreate} from "./dbHelper";
import {
  processLiquidityAddedEvent,
  processLiquidityRemovedEvent,
  processPoolCreatedEvent, processPoolDeletedEvent,
  processSwappedEvent
} from "./pabloProcessor";

const processor = new SubstrateProcessor("composable_dali_dev");

const chain = (): string => {
  switch(process.env.ENV) {
    case 'dali':
      return 'wss://dali.devnets.composablefinance.ninja/parachain/alice'
    case 'dali-stage':
      return 'wss://dali-cluster-fe.composablefinance.ninja'
    default:
      return 'ws://localhost:9997'
  }
};

const chain_connection_string = chain();
const archive_connection_string = 'http://localhost:8080/v1/graphql';

console.log(`Chain ${chain_connection_string}`);
console.log(`Archive ${archive_connection_string}`);

processor.setBatchSize(500);
processor.setDataSource({
  archive: archive_connection_string,
  chain: chain_connection_string,
});

processor.addEventHandler('pablo.PoolCreated', async (ctx) => {
  const event = new PabloPoolCreatedEvent(ctx);
  await processPoolCreatedEvent(ctx, event);
});

processor.addEventHandler('pablo.PoolDeleted', async (ctx) => {
  const event = new PabloPoolDeletedEvent(ctx);
  await processPoolDeletedEvent(ctx, event);
});

processor.addEventHandler('pablo.LiquidityAdded', async (ctx) => {
  const event = new PabloLiquidityAddedEvent(ctx);
  await processLiquidityAddedEvent(ctx, event);
});

processor.addEventHandler('pablo.LiquidityRemoved', async (ctx) => {
  const event = new PabloLiquidityRemovedEvent(ctx);
  await processLiquidityRemovedEvent(ctx, event);
});

processor.addEventHandler('pablo.Swapped', async (ctx) => {
  const event = new PabloSwappedEvent(ctx);
  await processSwappedEvent(ctx, event);
});

processor.addEventHandler("balances.Transfer", async (ctx) => {
  const transfer = getTransferEvent(ctx);
  const tip = ctx.extrinsic?.tip || 0n;
  const from = ss58.codec("picasso").encode(transfer.from);
  const to = ss58.codec("picasso").encode(transfer.to);

  const fromAcc = await getOrCreate(ctx.store, Account, from);
  fromAcc.balance = fromAcc.balance || 0n;
  fromAcc.balance -= transfer.amount;
  fromAcc.balance -= tip;
  await ctx.store.save(fromAcc);

  const toAcc = await getOrCreate(ctx.store, Account, to);
  toAcc.balance = toAcc.balance || 0n;
  toAcc.balance += transfer.amount;
  await ctx.store.save(toAcc);

  await ctx.store.save(
    new HistoricalBalance({
      id: `${ctx.event.id}-to`,
      account: fromAcc,
      balance: fromAcc.balance,
      date: new Date(ctx.block.timestamp),
    })
  );

  await ctx.store.save(
    new HistoricalBalance({
      id: `${ctx.event.id}-from`,
      account: toAcc,
      balance: toAcc.balance,
      date: new Date(ctx.block.timestamp),
    })
  );
});

processor.run();

interface TransferEvent {
  from: Uint8Array;
  to: Uint8Array;
  amount: bigint;
}

function getTransferEvent(ctx: EventHandlerContext): TransferEvent {
  const event = new BalancesTransferEvent(ctx);
  if (event.isV2100) {
    const {from, to, amount} = event.asV2100;
    return { from, to, amount };
  } else {
    const { from, to, amount } = event.asLatest;
    return { from, to, amount };
  }
}
