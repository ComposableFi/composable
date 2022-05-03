"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const ss58 = __importStar(require("@subsquid/ss58"));
const substrate_processor_1 = require("@subsquid/substrate-processor");
const model_1 = require("./model");
const events_1 = require("./types/events");
const dbHelper_1 = require("./dbHelper");
const pabloProcessor_1 = require("./pabloProcessor");
const processor = new substrate_processor_1.SubstrateProcessor("composable_dali_dev");
processor.setBatchSize(500);
processor.setDataSource({
    archive: `http://localhost:8080/v1/graphql`,
    chain: "ws://localhost:9988",
});
// TODO add event handlers for Pablo
processor.addEventHandler('pablo.PoolCreated', async (ctx) => {
    const event = new events_1.PabloPoolCreatedEvent(ctx);
    await (0, pabloProcessor_1.processPoolCreatedEvent)(ctx, event);
});
processor.addEventHandler('pablo.LiquidityAdded', async (ctx) => {
    const event = new events_1.PabloLiquidityAddedEvent(ctx);
    await (0, pabloProcessor_1.processLiquidityAddedEvent)(ctx, event);
});
processor.addEventHandler('pablo.Swapped', async (ctx) => {
    const event = new events_1.PabloSwappedEvent(ctx);
    await (0, pabloProcessor_1.processSwappedEvent)(ctx, event);
});
processor.addEventHandler("balances.Transfer", async (ctx) => {
    const transfer = getTransferEvent(ctx);
    const tip = ctx.extrinsic?.tip || 0n;
    const from = ss58.codec("picasso").encode(transfer.from);
    const to = ss58.codec("picasso").encode(transfer.to);
    const fromAcc = await (0, dbHelper_1.getOrCreate)(ctx.store, model_1.Account, from);
    fromAcc.balance = fromAcc.balance || 0n;
    fromAcc.balance -= transfer.amount;
    fromAcc.balance -= tip;
    await ctx.store.save(fromAcc);
    const toAcc = await (0, dbHelper_1.getOrCreate)(ctx.store, model_1.Account, to);
    toAcc.balance = toAcc.balance || 0n;
    toAcc.balance += transfer.amount;
    await ctx.store.save(toAcc);
    await ctx.store.save(new model_1.HistoricalBalance({
        id: `${ctx.event.id}-to`,
        account: fromAcc,
        balance: fromAcc.balance,
        date: new Date(ctx.block.timestamp),
    }));
    await ctx.store.save(new model_1.HistoricalBalance({
        id: `${ctx.event.id}-from`,
        account: toAcc,
        balance: toAcc.balance,
        date: new Date(ctx.block.timestamp),
    }));
});
processor.run();
function getTransferEvent(ctx) {
    const event = new events_1.BalancesTransferEvent(ctx);
    if (event.isV2100) {
        const { from, to, amount } = event.asV2100;
        return { from, to, amount };
    }
    else {
        const { from, to, amount } = event.asLatest;
        return { from, to, amount };
    }
}
//# sourceMappingURL=processor.js.map