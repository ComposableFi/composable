import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
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
  const [assetId, price] = event.asV2402;
  return { assetId, price };
}

/**
 * Updates Asset object with last price and event id.
 * @param ctx
 * @param asset
 * @param price
 */
export function updateAsset(
  ctx: EventHandlerContext<Store>,
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
  ctx: EventHandlerContext<Store>,
  asset: Asset,
  price: bigint
): HistoricalAssetPrice {
  return new HistoricalAssetPrice({
    id: randomUUID(),
    eventId: ctx.event.id,
    asset,
    price,
    timestamp: new Date(ctx.block.timestamp),
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
  ctx: EventHandlerContext<Store>
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
