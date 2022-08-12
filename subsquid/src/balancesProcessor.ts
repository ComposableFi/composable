import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import {
  BalancesDepositEvent,
  BalancesTransferEvent,
  BalancesWithdrawEvent,
} from "./types/events";
import { createTransaction, encodeAccount } from "./utils";
import { getOrCreate } from "./dbHelper";
import { Account, HistoricalBalance, PicassoTransactionType } from "./model";

interface TransferEvent {
  from: Uint8Array;
  to: Uint8Array;
  amount: bigint;
}

interface DepositEvent {
  who: Uint8Array;
  amount: bigint;
}

interface WithdrawEvent {
  who: Uint8Array;
  amount: bigint;
}

function getTransferEvent(event: BalancesTransferEvent): TransferEvent {
  const { from, to, amount } = event.asV2401 ?? event.asLatest;
  return { from, to, amount };
}

function getWithdrawEvent(event: BalancesWithdrawEvent): DepositEvent {
  const { who, amount } = event.asV2401 ?? event.asLatest;
  return { who, amount };
}

function getDepositEvent(event: BalancesDepositEvent): WithdrawEvent {
  const { who, amount } = event.asV2401 ?? event.asLatest;
  return { who, amount };
}

/**
 * Create HistoricalBalance for an account
 * @param ctx
 * @param transactionId
 * @param account
 */
function createHistoricalBalance(
  ctx: EventHandlerContext,
  transactionId: string,
  account: Account
): HistoricalBalance {
  return new HistoricalBalance({
    id: randomUUID(),
    eventId: ctx.event.id,
    transactionId,
    account: account,
    balance: account.balance,
    date: new Date(ctx.block.timestamp),
  });
}

/**
 * Handle `balance.Transfer` event.
 *   - Create/update account who transfers funds, and update balance.
 *   - Create/update account who receives funds, and update balance.
 *   - Create transaction.
 *   - Create HistoricalBalance for both accounts
 * @param ctx
 */
export async function processTransferEvent(ctx: EventHandlerContext) {
  console.log("Process transfer");
  const event = new BalancesTransferEvent(ctx);
  const transferEvent = getTransferEvent(event);
  const from = encodeAccount(transferEvent.from);
  const to = encodeAccount(transferEvent.to);
  const tip = ctx.extrinsic?.tip || 0n;
  const { amount } = transferEvent;

  const accountFrom = await getOrCreate(ctx.store, Account, from);
  const accountTo = await getOrCreate(ctx.store, Account, to);

  // Update balance
  accountFrom.balance =
    BigInt(accountFrom.balance || 0n) - BigInt(amount) - BigInt(tip);

  accountTo.balance = BigInt(accountTo.balance || 0n) + BigInt(amount);

  // TODO: get correct initial balance

  const txId = randomUUID();

  // Create transaction
  const tx = createTransaction(
    ctx,
    from,
    PicassoTransactionType.BALANCES_TRANSFER,
    txId
  );

  await ctx.store.save(tx);
  await ctx.store.save(accountFrom);
  await ctx.store.save(accountTo);

  const historicalBalanceFrom = createHistoricalBalance(ctx, txId, accountFrom);
  await ctx.store.save(historicalBalanceFrom);

  const historicalBalanceTo = createHistoricalBalance(ctx, txId, accountTo);
  await ctx.store.save(historicalBalanceTo);
}

/**
 * Handle `balance.Withdraw` event.
 *   - Create/update account who withdraws funds, and update balance.
 *   - Create transaction.
 *   - Create HistoricalBalance.
 * @param ctx
 */
export async function processWithdrawEvent(ctx: EventHandlerContext) {
  console.log("Process withdraw");
  const evt = new BalancesWithdrawEvent(ctx);
  const event = getWithdrawEvent(evt);
  const who = encodeAccount(event.who);
  const tip = ctx.extrinsic?.tip || 0n;
  const { amount } = event;

  const account = await getOrCreate(ctx.store, Account, who);

  // Update balance
  account.balance =
    BigInt(account.balance || 0n) - BigInt(amount) - BigInt(tip);

  // TODO: get correct initial balance

  const txId = randomUUID();

  // Create transaction
  const tx = createTransaction(
    ctx,
    who,
    PicassoTransactionType.BALANCES_WITHDRAW,
    txId
  );

  await ctx.store.save(tx);
  await ctx.store.save(account);

  const historicalBalance = createHistoricalBalance(ctx, txId, account);
  await ctx.store.save(historicalBalance);

  console.log("Finish processing `withdraw`");
}

/**
 * Handle `balance.Deposit` event.
 *   - Create/update account who deposits funds, and update balance.
 *   - Create transaction.
 *   - Create HistoricalBalance.
 * @param ctx
 */
export async function processDepositEvent(ctx: EventHandlerContext) {
  console.log("Process deposit");
  const evt = new BalancesDepositEvent(ctx);
  const event = getDepositEvent(evt);
  const who = encodeAccount(event.who);
  const tip = ctx.extrinsic?.tip || 0n;
  const { amount } = event;

  const account = await getOrCreate(ctx.store, Account, who);

  // Update balance
  account.balance =
    BigInt(account.balance || 0n) + BigInt(amount) - BigInt(tip);

  // TODO: get correct initial balance

  const txId = randomUUID();

  // Create transaction
  const tx = createTransaction(
    ctx,
    who,
    PicassoTransactionType.BALANCES_DEPOSIT,
    txId
  );

  await ctx.store.save(account);
  await ctx.store.save(tx);

  const historicalBalance = createHistoricalBalance(ctx, txId, account);
  await ctx.store.save(historicalBalance);

  console.log("Finish processing `deposit`");
}
