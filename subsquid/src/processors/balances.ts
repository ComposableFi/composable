import { EventHandlerContext } from "@subsquid/substrate-processor";
import {
  BalancesDepositEvent,
  BalancesTransferEvent,
  BalancesWithdrawEvent,
} from "../types/events";
import { encodeAccount } from "../utils";
import { saveAccountAndTransaction } from "../dbHelper";
import { TransactionType } from "../model";

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
 * Handle `balance.Transfer` event.
 *   - Create/update Account who transfers funds.
 *   - Create/update Account who receives funds.
 *   - Create Transaction.
 *   - Create Activity
 * @param ctx
 */
export async function processTransferEvent(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Process transfer");
  const event = new BalancesTransferEvent(ctx);
  const transferEvent = getTransferEvent(event);
  const from = encodeAccount(transferEvent.from);
  const to = encodeAccount(transferEvent.to);

  await saveAccountAndTransaction(ctx, TransactionType.BALANCES_TRANSFER, [
    from,
    to,
  ]);
}

/**
 * Handle `balance.Withdraw` event.
 *   - Create/update Account who withdraws funds.
 *   - Create Transaction.
 *   - Create Activity.
 * @param ctx
 */
export async function processWithdrawEvent(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Process withdraw");
  const evt = new BalancesWithdrawEvent(ctx);
  const event = getWithdrawEvent(evt);
  const who = encodeAccount(event.who);

  await saveAccountAndTransaction(ctx, TransactionType.BALANCES_WITHDRAW, who);
}

/**
 * Handle `balance.Deposit` event.
 *   - Create/update Account who deposits funds.
 *   - Create Transaction.
 *   - Create Activity.
 * @param ctx
 */
export async function processDepositEvent(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Process deposit");
  const evt = new BalancesDepositEvent(ctx);
  const event = getDepositEvent(evt);
  const who = encodeAccount(event.who);

  await saveAccountAndTransaction(ctx, TransactionType.BALANCES_DEPOSIT, who);
}
