import { BalancesDepositEvent, BalancesTransferEvent, BalancesWithdrawEvent } from "../types/events";
import { encodeAccount } from "../utils";
import { saveAccountAndEvent } from "../dbHelper";
import { EventType } from "../model";
import { Block, Context, EventItem } from "../processorTypes";

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
  return event.asV200;
}

function getWithdrawEvent(event: BalancesWithdrawEvent): DepositEvent {
  return event.asV200;
}

function getDepositEvent(event: BalancesDepositEvent): WithdrawEvent {
  return event.asV200;
}

/**
 * Handle `balance.Transfer` event.
 *   - Create/update Account who transfers funds.
 *   - Create/update Account who receives funds.
 *   - Create Event.
 *   - Create Activity
 * @param ctx
 * @param block
 * @param eventItem
 */
export async function processTransferEvent(ctx: Context, block: Block, eventItem: EventItem): Promise<void> {
  const evt = new BalancesTransferEvent(ctx, eventItem.event);
  const transferEvent = getTransferEvent(evt);
  const from = encodeAccount(transferEvent.from);
  const to = encodeAccount(transferEvent.to);

  await saveAccountAndEvent(ctx, block, eventItem, EventType.BALANCES_TRANSFER, [from, to]);
}

/**
 * Handle `balance.Withdraw` event.
 *   - Create/update Account who withdraws funds.
 *   - Create Event.
 *   - Create Activity.
 * @param ctx
 * @param block
 * @param eventItem
 */
export async function processWithdrawEvent(ctx: Context, block: Block, eventItem: EventItem): Promise<void> {
  const evt = new BalancesWithdrawEvent(ctx, eventItem.event);
  const withdrawEvent = getWithdrawEvent(evt);
  const who = encodeAccount(withdrawEvent.who);

  await saveAccountAndEvent(ctx, block, eventItem, EventType.BALANCES_WITHDRAW, who);
}

/**
 * Handle `balance.Deposit` event.
 *   - Create/update Account who deposits funds.
 *   - Create Event.
 *   - Create Activity.
 * @param ctx
 * @param block
 * @param eventItem
 */
export async function processDepositEvent(ctx: Context, block: Block, eventItem: EventItem): Promise<void> {
  const evt = new BalancesDepositEvent(ctx, eventItem.event);
  const depositEvent = getDepositEvent(evt);
  const who = encodeAccount(depositEvent.who);

  await saveAccountAndEvent(ctx, block, eventItem, EventType.BALANCES_DEPOSIT, who);
}
