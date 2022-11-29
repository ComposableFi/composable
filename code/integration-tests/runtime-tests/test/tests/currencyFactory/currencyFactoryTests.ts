import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { AnyNumber } from "@polkadot/types-codec/types";
import { u128, u64 } from "@polkadot/types-codec";
import BN from "bn.js";
import { AssetId } from "@polkadot/types/interfaces/runtime";
import { ComposableTraitsAssetsBasicAssetMetadata } from "@composable/types/interfaces";

/**
 *
 * Currency Factory Integration Tests
 *
 * The currency factory pallet persists of 2 extrinsics.
 * - currencyFactory.setMetadata
 * - currencyFactory.addRange
 */
describe("[SHORT] Currency Factory Tests", function () {
  if (!testConfiguration.enabledTests.enabled) return;

  let api: ApiPromise;
  let sudoKey: KeyringPair;
  let assetIdToSetMetadata: BN | u128 | AssetId;

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
    it("Sudo can initialize new asset", async function () {
      console.warn("setMetadata tests are temporarily disabled!");
      this.skip();

      // this.timeout(2 * 60 * 1000);
      // const amount = 999_999_999_999;
      // const beneficiary = sudoKey.publicKey;
      // const assetListBefore = await api.query.currencyFactory.assetEd.entries();
      // const {
      //   data: [resultCurrencyId, resultBeneficiary, resultAmount]
      // } = await CurrencyFactoryTests.initializeNewAsset(api, sudoKey, amount, beneficiary);
      // assetIdToSetMetadata = resultCurrencyId;
      // expect(resultBeneficiary.toString()).to.equal(api.createType("AccountId32", beneficiary).toString());
      // expect(resultAmount).to.be.bignumber.equal(amount.toString());
      // const assetListAfter = await api.query.currencyFactory.assetEd.entries();
      // expect(assetListAfter.length).to.be.greaterThan(assetListBefore.length);
    });
    it("Sudo can set metadata for newly created asset", async function () {
      console.warn("setMetadata tests are temporarily disabled!");
      this.timeout(2 * 60 * 1000);
      this.skip();

      // const assetId = assetIdToSetMetadata;
      // const metadata = {
      //   symbol: {
      //     inner: "BAN"
      //   },
      //   name: {
      //     inner: "Banana Coin"
      //   }
      // };
      //
      // const {
      //   data: [result]
      // } = await CurrencyFactoryTests.setMetadata(api, sudoKey, assetId, metadata);
      // expect(result.isOk).to.be.true;
      //
      // const assetMetadataAfter = <Option<ComposableTraitsAssetsBasicAssetMetadata>>(
      //   await api.query.currencyFactory.assetMetadata(assetId)
      // );
      // expect(assetMetadataAfter.unwrap().isEmpty).to.be.false;
      // expect(CurrencyFactoryTests.hex2a(assetMetadataAfter.unwrap().name.inner)).to.be.equal(
      //   metadata.name.inner.toString()
      // );
      // expect(CurrencyFactoryTests.hex2a(assetMetadataAfter.unwrap().symbol.inner)).to.be.equal(
      //   metadata.symbol.inner.toString()
      // );
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

class CurrencyFactoryTests {
  public static async setMetadata(
    api: ApiPromise,
    sudoKey: KeyringPair,
    assetId: u128 | AnyNumber | AssetId,
    metadata: string | Uint8Array | ComposableTraitsAssetsBasicAssetMetadata | { symbol?: any; name?: any }
  ) {
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

  public static async initializeNewAsset(
    api: ApiPromise,
    sudoKey: KeyringPair,
    amount: AnyNumber | u128,
    beneficiary: Uint8Array
  ) {
    return await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.tokens.Endowed.is,
      api.tx.sudo.sudo(api.tx.assets.mintInitialize(amount, beneficiary))
    );
  }

  /**
   * Converts hex to ascii.
   * Source: https://stackoverflow.com/questions/3745666/how-to-convert-from-hex-to-ascii-in-javascript/3745677#3745677
   * @param hex
   */
  public static hex2a(hex: any) {
    const hex_string = hex.toString().replace("0x", ""); //force conversion
    let str = "";
    for (let i = 0; i < hex_string.length; i += 2) str += String.fromCharCode(parseInt(hex_string.substr(i, 2), 16));
    return str;
  }
}
