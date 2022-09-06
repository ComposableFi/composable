import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import { OraclePriceChangedEvent } from "../types/events";
import { Asset, Currency, HistoricalAssetPrice } from "../model";

interface PriceChangedEvent {
  assetId: bigint;
  price: bigint;
}

function getPriceChangedEvent(
  event: OraclePriceChangedEvent
): PriceChangedEvent {
  const [assetId, price] = event.asV2401 ?? event.asLatest;
  return { assetId, price };
}

/**
 * Updates Asset object with last price and event id.
 * @param ctx
 * @param asset
 * @param price
 */
export function updateAsset(
  ctx: EventHandlerContext,
  asset: Asset,
  price: bigint
): void {
  asset.eventId = ctx.event.id;
  asset.price = price;
}

/**
 * Creates and returns a HistoricalAssetPrice.
 * @param ctx
 * @param asset
 * @param price
 */
export function getHistoricalAssetPrice(
  ctx: EventHandlerContext,
  asset: Asset,
  price: bigint
): HistoricalAssetPrice {
  return new HistoricalAssetPrice({
    id: randomUUID(),
    eventId: ctx.event.id,
    asset,
    price,
    timestamp: BigInt(ctx.block.timestamp),
    currency: Currency.USD,
  });
}

/**
 * Handle `oracle.PriceChanged` event.
 *  - Create or update Asset.
 *  - Create HistoricalAssetPrice.
 * @param ctx
 */
export async function processOraclePriceChanged(
  ctx: EventHandlerContext
): Promise<void> {
  console.log("Process price change");
  const event = new OraclePriceChangedEvent(ctx);
  const { assetId, price } = getPriceChangedEvent(event);

  let asset: Asset | undefined = await ctx.store.get(Asset, {
    where: { id: assetId.toString() },
  });

  if (!asset) {
    asset = new Asset({
      id: assetId.toString(),
    });
  }

  updateAsset(ctx, asset, price);

  await ctx.store.save(asset);

  const historicalAssetPrice: HistoricalAssetPrice = getHistoricalAssetPrice(
    ctx,
    asset,
    price
  );

  await ctx.store.save(historicalAssetPrice);
}
