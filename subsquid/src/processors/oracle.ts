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
 * Handle `oracle.PriceChanged` event.
 *  - Create or update Asset.
 *  - Create HistoricalAssetPrice.
 * @param ctx
 */
export async function processOraclePriceChanged(ctx: EventHandlerContext) {
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

  asset.eventId = ctx.event.id;
  asset.price = price;

  await ctx.store.save(asset);

  const historicalAssetPrice = new HistoricalAssetPrice({
    id: randomUUID(),
    eventId: ctx.event.id,
    asset,
    price,
    timestamp: BigInt(ctx.block.timestamp),
  });

  await ctx.store.save(historicalAssetPrice);
}
