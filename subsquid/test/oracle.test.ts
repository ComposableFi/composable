import { EventHandlerContext } from "@subsquid/substrate-processor";
import { Store } from "@subsquid/typeorm-store";
import { mock } from "ts-mockito";
import { expect } from "chai";
import { Asset, Currency, HistoricalAssetPrice } from "../src/model";
import { createCtx } from "../src/utils";
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
 * @param currency
 */
function assertHistoricalAssetPrice(
  historicalAssetPrice: HistoricalAssetPrice,
  assetId: string,
  price: bigint,
  currency: Currency
) {
  expect(historicalAssetPrice.asset.id).to.equal(assetId);
  expect(historicalAssetPrice.price).to.equal(price);
  expect(historicalAssetPrice.currency).to.equal(currency);
}

describe("Oracle price changed", () => {
  let storeMock: Store;
  let ctx: EventHandlerContext<Store>;

  beforeEach(() => {
    storeMock = mock<Store>();
    ctx = createCtx(storeMock, 1);
  });

  it("Should update asset", () => {
    const asset: Asset = {
      id: "1",
      eventId: "event-id",
      price: 1n,
      historicalPrices: [],
    };

    updateAsset(ctx, asset, 10n);

    assertAsset(asset, "1", 10n);
  });

  it("Should create HistoricalAssetPrice", () => {
    const asset: Asset = {
      id: "1",
      eventId: "event-id",
      price: 1n,
      historicalPrices: [],
    };

    const historicalAssetPrice = getHistoricalAssetPrice(ctx, asset, 20n);

    assertHistoricalAssetPrice(historicalAssetPrice, "1", 20n, Currency.USD);
  });
});
