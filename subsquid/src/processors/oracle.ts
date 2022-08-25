import { EventHandlerContext } from "@subsquid/substrate-processor";
import { randomUUID } from "crypto";
import { OraclePriceChangedEvent } from "../types/events";
import { Asset, HistoricalAssetPrice } from "../model";

console.log("asddsadsa");

interface PriceChangedEvent {
  assetId: bigint;
  price: bigint;
}

function getPriceChangedEvent(
  event: OraclePriceChangedEvent
): PriceChangedEvent {
  const [assetId, price] = event.asV2401 ?? event.asLatest;
  console.log({ assetId, price });
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
  });
}

/**
 * Handle `oracle.PriceChanged` event.
 *  - Create or update Asset.
 *  - Create HistoricalAssetPrice.
 * @param ctx
 */
export async function processOraclePriceChanged(ctx: EventHandlerContext) {
  console.log("Process price change");
  const event = new OraclePriceChangedEvent(ctx);
  const { assetId, price } = getPriceChangedEvent(event);
  const price2 = BigInt(Math.round(Math.random() * 40));

  let asset: Asset | undefined = await ctx.store.get(Asset, {
    where: { id: assetId.toString() },
  });

  let asset2: Asset | undefined = await ctx.store.get(Asset, {
    where: { id: "2" },
  });

  if (!asset) {
    asset = new Asset({
      id: assetId.toString(),
    });
  }

  if (!asset2) {
    asset2 = new Asset({
      id: "2",
    });
  }

  updateAsset(ctx, asset, price);
  updateAsset(ctx, asset2, price2);

  await ctx.store.save(asset);
  await ctx.store.save(asset2);

  const historicalAssetPrice = getHistoricalAssetPrice(ctx, asset, price);
  const historicalAssetPrice2 = getHistoricalAssetPrice(ctx, asset2, price2);

  await ctx.store.save(historicalAssetPrice);
  await ctx.store.save(historicalAssetPrice2);
}
