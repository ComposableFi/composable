import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import {
  BalancesDepositEvent,
  BalancesTransferEvent,
  BalancesWithdrawEvent,
} from "../types/events";
import { encodeAccount } from "../utils";
import { saveAccountAndEvent } from "../dbHelper";
import { EventType } from "../model";

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
  const { from, to, amount } = event.asV2402;
  return { from, to, amount };
}

function getWithdrawEvent(event: BalancesWithdrawEvent): DepositEvent {
  const { who, amount } = event.asV2402;
  return { who, amount };
}

function getDepositEvent(event: BalancesDepositEvent): WithdrawEvent {
  const { who, amount } = event.asV2402;
  return { who, amount };
}

/**
 * Handle `balance.Transfer` event.
 *   - Create/update Account who transfers funds.
 *   - Create/update Account who receives funds.
 *   - Create Event.
 *   - Create Activity
 * @param ctx
 */
export async function processTransferEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process transfer");
  const event = new BalancesTransferEvent(ctx);
  const transferEvent = getTransferEvent(event);
  const from = encodeAccount(transferEvent.from);
  const to = encodeAccount(transferEvent.to);

  await saveAccountAndEvent(ctx, EventType.BALANCES_TRANSFER, [from, to]);
}

/**
 * Handle `balance.Withdraw` event.
 *   - Create/update Account who withdraws funds.
 *   - Create Event.
 *   - Create Activity.
 * @param ctx
 */
export async function processWithdrawEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process withdraw");
  const evt = new BalancesWithdrawEvent(ctx);
  const event = getWithdrawEvent(evt);
  const who = encodeAccount(event.who);

  await saveAccountAndEvent(ctx, EventType.BALANCES_WITHDRAW, who);
}

/**
 * Handle `balance.Deposit` event.
 *   - Create/update Account who deposits funds.
 *   - Create Event.
 *   - Create Activity.
 * @param ctx
 */
export async function processDepositEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process deposit");
  const evt = new BalancesDepositEvent(ctx);
  const event = getDepositEvent(evt);
  const who = encodeAccount(event.who);

  await saveAccountAndEvent(ctx, EventType.BALANCES_DEPOSIT, who);
}
