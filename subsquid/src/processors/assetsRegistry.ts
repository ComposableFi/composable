import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { getHistoricalAssetPrice } from "./oracle";
import { Asset, HistoricalAssetPrice } from "../model";
import { chain } from "../config";
import {
  AssetsRegistryAssetRegisteredEvent,
  AssetsRegistryAssetUpdatedEvent,
} from "../types/events";

interface AssetRegisteredEvent {
  assetId: bigint;
  decimals: number | undefined | null;
}

interface AssetUpdatedEvent {
  assetId: bigint;
  decimals: number | undefined | null;
}

function getAssetRegisteredEvent(
  event: AssetsRegistryAssetRegisteredEvent
): AssetRegisteredEvent {
  if (event.isV1200) {
    return {
      decimals: 0,
      ...event.asV1200,
    };
  }
  return event.asV10002;
}

function getAssetUpdatedEvent(
  event: AssetsRegistryAssetUpdatedEvent
): AssetUpdatedEvent {
  if (event.isV1200) {
    return {
      assetId: event.asV1200.assetId,
      decimals: 0,
    };
  }
  return event.asV10002;
}

/**
 * Handle AssetRegistry.AssetRegisteredEvent
 *  - Get the latest price from oracle
 *  - Create new asset
 *  - Update historical price
 * No need to store account activity, as this should only be executed by
 * sudo account
 * @param ctx
 */
export async function processAssetRegisteredEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process AssetsRegistry.AssetRegistered event");
  const event = new AssetsRegistryAssetRegisteredEvent(ctx);
  const assetRegisteredEvent = getAssetRegisteredEvent(event);
  const { assetId, decimals } = assetRegisteredEvent;

  const wsProvider = new WsProvider(chain());
  const api = await ApiPromise.create({ provider: wsProvider });

  let price = 0n;

  try {
    const oraclePrice = await api.query.oracle.prices(assetId);
    if (!oraclePrice?.price) {
      return;
    }

    price = BigInt(oraclePrice.price.toString());
  } catch (error) {
    console.warn("Warning: Oracle not available.");
    return;
  }

  const asset = new Asset({
    id: assetId.toString(),
    eventId: ctx.event.id,
    price,
    decimals,
  });

  await ctx.store.save(asset);

  const historicalAssetPrice: HistoricalAssetPrice = getHistoricalAssetPrice(
    ctx,
    asset,
    price
  );

  await ctx.store.save(historicalAssetPrice);
}

/**
 * Handle AssetRegistry.AssetUpdatedEvent
 *  - Get asset
 *  - Update decimals and event
 * No need to store account activity, as this should only be executed by
 * sudo account
 * @param ctx
 */
export async function processAssetUpdatedEvent(
  ctx: EventHandlerContext<Store>
): Promise<void> {
  console.log("Process AssetsRegistry.AssetUpdated event");
  const event = new AssetsRegistryAssetUpdatedEvent(ctx);
  const assetUpdatedEvent = getAssetUpdatedEvent(event);
  const { assetId, decimals } = assetUpdatedEvent;

  const asset: Asset | undefined = await ctx.store.get(Asset, {
    where: {
      id: assetId.toString(),
    },
  });

  if (!asset) {
    console.log(`Asset ${assetId} not found`);
    return;
  }

  asset.decimals = decimals;
  asset.eventId = ctx.event.id;

  await ctx.store.save(asset);
}
