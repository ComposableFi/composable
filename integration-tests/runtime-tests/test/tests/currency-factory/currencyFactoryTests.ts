import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { AnyNumber } from "@polkadot/types-codec/types";
import { u64 } from "@polkadot/types-codec";

/**
 *
 * Currency Factory Integration Tests
 *
 * The currency factory pallet persists of 2 extrinsics.
 * - currencyFactory.addRange
 * - currencyFactory.setMetadata
 */
describe("[SHORT] Currency Factory Tests", function () {
  if (!testConfiguration.enabledTests.enabled) return;

  let api: ApiPromise;
  let sudoKey: KeyringPair;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("tx.currencyFactory.setMetadata", function () {
    it("Can set metadata for asset ID 100,000,000,001", async function () {
      this.timeout(2 * 60 * 1000);

      const assetId = 100_000_000_001;
      const metadata = {
        symbol: {
          inner: "BAN"
        },
        name: {
          inner: "Banana Coin"
        }
      };

      const assetMetadataBefore = await api.query.currencyFactory.assetMetadata(assetId);

      const {
        data: [result]
      } = await CurrencyFactoryTests.setMetadata(api, sudoKey, assetId, metadata);
      expect(result.isOk).to.be.true;

      const assetMetadataAfter = await api.query.currencyFactory.assetMetadata(assetId);
      expect(assetMetadataBefore.isEmpty).to.be.true;
      expect(assetMetadataAfter.unwrap().isEmpty).to.be.false;
      expect(assetMetadataAfter.unwrap().name.toString()).to.be.equal(metadata.name.inner.toString());
      expect(assetMetadataAfter.unwrap().symbol.toString()).to.be.equal(metadata.symbol.inner.toString());
    });

    it("Can set metadata for asset ID 1 (PICA)", async function () {
      this.timeout(2 * 60 * 1000);

      const assetId = 100_000_000_001;
      const metadata = {
        symbol: {
          inner: "BAN"
        },
        name: {
          inner: "Banana Coin"
        }
      };

      const assetMetadataBefore = await api.query.currencyFactory.assetMetadata(assetId);

      const {
        data: [result]
      } = await CurrencyFactoryTests.setMetadata(api, sudoKey, assetId, metadata);
      expect(result.isOk).to.be.true;

      const assetMetadataAfter = await api.query.currencyFactory.assetMetadata(assetId);
      expect(assetMetadataBefore.isEmpty).to.be.true;
      expect(assetMetadataAfter.unwrap().isEmpty).to.be.false;
      expect(assetMetadataAfter.unwrap().name.toString()).to.be.equal(metadata.name.inner.toString());
      expect(assetMetadataAfter.unwrap().symbol.toString()).to.be.equal(metadata.symbol.inner.toString());
    });

    it("Can set metadata for asset ID 1000 (BTC)", async function () {
      this.timeout(2 * 60 * 1000);

      const assetId = 1000;
      const metadata = {
        symbol: {
          inner: "BTC"
        },
        name: {
          inner: "Bitcoin"
        }
      };

      const assetMetadataBefore = await api.query.currencyFactory.assetMetadata(assetId);

      const {
        data: [result]
      } = await CurrencyFactoryTests.setMetadata(api, sudoKey, assetId, metadata);
      expect(result.isOk).to.be.true;

      const assetMetadataAfter = await api.query.currencyFactory.assetMetadata(assetId);
      expect(assetMetadataBefore.isEmpty).to.be.true;
      expect(assetMetadataAfter.unwrap().isEmpty).to.be.false;
      expect(assetMetadataAfter.unwrap().name.toString()).to.be.equal(metadata.name.inner.toString());
      expect(assetMetadataAfter.unwrap().symbol.toString()).to.be.equal(metadata.symbol.inner.toString());
    });
  });

  describe("tx.currencyFactory.addRange", function () {
    it("Sudo can add new currency ID range: ", async function () {
      this.timeout(2 * 60 * 1000);

      const range = 100;
      expect(range).to.be.greaterThan(0);
      const assetIdRangesBefore = await api.query.currencyFactory.assetIdRanges();
      const rangeLengthBefore: number = assetIdRangesBefore.ranges.length;
      const lastAssetIdRangeBefore = assetIdRangesBefore.ranges[rangeLengthBefore - 1];
      const {
        data: [result]
      } = await CurrencyFactoryTests.addRange(api, sudoKey, range);
      expect(result.isOk).to.be.true;
      const assetIdRangesAfter = await api.query.currencyFactory.assetIdRanges();
      const rangeLengthAfter: number = assetIdRangesAfter.ranges.length;
      const lastAssetIdRangeAfter = assetIdRangesAfter.ranges[rangeLengthAfter - 1];

      expect(rangeLengthAfter).to.be.greaterThan(rangeLengthBefore);
      expect(lastAssetIdRangeAfter.current).to.be.bignumber.equal(lastAssetIdRangeBefore.end.addn(1));
    });
  });
});

export class CurrencyFactoryTests {
  public static async setMetadata(api: ApiPromise, sudoKey: KeyringPair, assetId, metadata) {
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.currencyFactory.setMetadata(assetId, metadata))
    );
  }

  public static async addRange(api: ApiPromise, sudoKey: KeyringPair, range: AnyNumber | u64) {
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.currencyFactory.addRange(range))
    );
  }
}
