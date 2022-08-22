import { EventHandlerContext } from "@subsquid/substrate-processor";
import { OraclePriceChangedEvent } from "../types/events";
import { encodeAccount } from "../utils";
import { saveAccountAndTransaction } from "../dbHelper";
import { Asset, HistoricalAssetPrice } from "../model";

interface PriceChangedEvent {}

function getPriceChangedEvent(
  event: OraclePriceChangedEvent
): PriceChangedEvent {
  const all = event.asV2401 ?? event.asLatest;
  console.log({ all });
  return {};
}

/**
 * Handle `oracle.PriceChanged` event.
 * @param ctx
 */
export async function processOraclePriceChanged(ctx: EventHandlerContext) {
  console.log("Process price change");
  const event = new OraclePriceChangedEvent(ctx);
  const priceChangedEvent = getPriceChangedEvent(event);

  // await saveAccountAndTransaction(
  //   ctx,
  //   PicassoTransactionType.BALANCES_TRANSFER,
  //   [from, to]
  // );
}
