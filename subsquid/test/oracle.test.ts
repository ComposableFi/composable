import { EventHandlerContext, Store } from "@subsquid/substrate-processor";
import { Asset, HistoricalAssetPrice } from "../src/model";
import { instance, mock, when } from "ts-mockito";
import { createCtx } from "../src/utils";
import { OraclePriceChangedEvent } from "../src/types/events";
import { expect } from "chai";
import { getHistoricalAssetPrice, updateAsset } from "../src/processors/oracle";

/**
 * Check if Asset has expected values.
 * @param asset
 * @param assetId
 * @param price
 */
function assertAsset(asset: Asset, assetId: string, price: bigint) {
  expect(asset.id).to.equal(assetId);
  expect(asset.price).to.equal(price);
}

/**
 * Check if HistoricalAssetPrice has expected values.
 * @param historicalAssetPrice
 * @param assetId
 * @param price
 */
function assertHistoricalAssetPrice(
  historicalAssetPrice: HistoricalAssetPrice,
  assetId: string,
  price: bigint
) {
  expect(historicalAssetPrice.asset.id).to.equal(assetId);
  expect(historicalAssetPrice.price).to.equal(price);
}

describe("Oracle price changed", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext;

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);
  });

  it("Should update asset", async () => {
    const asset: Asset = {
      id: "1",
      eventId: "event-id",
      price: 1n,
      historicalPrices: [],
    };

    updateAsset(ctx, asset, 10n);

    assertAsset(asset, "1", 10n);
  });

  it("Should create HistoricalAssetPrice", async () => {
    const asset: Asset = {
      id: "1",
      eventId: "event-id",
      price: 1n,
      historicalPrices: [],
    };

    const historicalAssetPrice = getHistoricalAssetPrice(ctx, asset, 20n);

    assertHistoricalAssetPrice(historicalAssetPrice, "1", 20n);
  });
});
