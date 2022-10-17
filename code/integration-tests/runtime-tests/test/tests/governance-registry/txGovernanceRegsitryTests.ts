import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { Null, Result, u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces";
import { IEvent } from "@polkadot/types/types";
import { SpRuntimeDispatchError } from "@polkadot/types/lookup";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";

/**
 * Governance Registry Extrinsic Tests
 *
 * 1. Create governance asset.
 * 2. Remove governance asset.
 * 3. Set root for governance asset.
 */

describe("tx.governanceRegistry Tests", function () {
  if (!testConfiguration.enabledTests.tx.enabled) return;

  let api: ApiPromise;
  let walletAlice: KeyringPair, assetSigner: KeyringPair;
  let assetID: u128;

  before("Setting up the tests", async function () {
    this.timeout(2 * 60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice } = getDevWallets(newKeyring);
    walletAlice = devWalletAlice;
    assetSigner = walletAlice.derive("/governanceRegistry/signer");
    assetID = api.createType("u128", 1000);
  });

  before("Providing funds", async function () {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, assetSigner, walletAlice, [assetID.toNumber()]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("tx.governanceRegistry.set Tests", function () {
    this.timeout(2 * 60 * 1000);
    it("Setting assets governance wallet", async function () {
      if (!testConfiguration.enabledTests.tx.set__success) this.skip();

      const value = assetSigner.publicKey;
      const {
        data: [result]
      } = await TxGovernanceRegistryTests.setAsset(api, walletAlice, assetID, value);

      expect(result.isOk).to.be.true;

      const queryResult = await api.query.governanceRegistry.originsByAssetId(assetID);
      expect(queryResult.unwrap().isSigned).to.be.true;
    });
  });

  describe("tx.governanceRegistry.remove Tests", function () {
    this.timeout(2 * 60 * 1000);
    it("Removing governance asset", async function () {
      if (!testConfiguration.enabledTests.tx.remove__success) this.skip();

      const {
        data: [result]
      } = await TxGovernanceRegistryTests.removeAsset(api, walletAlice, assetID);
      expect(result.isOk).to.be.true;

      const queryResult = await api.query.governanceRegistry.originsByAssetId(assetID);
      expect(queryResult.isNone).to.be.true;
    });
  });

  describe("tx.governanceRegistry.grantRoot Tests", function () {
    this.timeout(2 * 60 * 1000);
    it("Grant root for governance asset", async function () {
      if (!testConfiguration.enabledTests.tx.remove__success) this.skip();

      const {
        data: [result]
      } = await TxGovernanceRegistryTests.grantRoot(api, walletAlice, assetID);
      expect(result.isOk).to.be.true;

      const queryResult = await api.query.governanceRegistry.originsByAssetId(assetID);
      expect(queryResult.unwrap().toString()).to.be.equal("Root");
    });
  });
});

class TxGovernanceRegistryTests {
  /**
   * Sets the value of an `asset_id` to the signed account id. Only callable by root.
   *
   * @param {ApiPromise} api Connected API Promise.
   * @param {Uint8Array|string} walletAddress wallet public key
   * @param {u128} assetID asset id
   * @param {AccountId32|Uint8Array} value Wallet to be set to
   */
  public static async setAsset(
    api: ApiPromise,
    wallet: KeyringPair,
    assetID: u128,
    value: AccountId32 | Uint8Array
  ): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>> {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.governanceRegistry.set(assetID, value))
    );
  }

  /**
   * Removes mapping of an `asset_id`. Only callable by root.
   *
   * @param {ApiPromise} api Connected API Promise.
   * @param {Uint8Array|string} wallet Wallet making the transaction.
   * @param {u128} assetID Asset id to be removed.
   */
  public static async removeAsset(
    api: ApiPromise,
    wallet: KeyringPair,
    assetID: u128
  ): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>> {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.governanceRegistry.remove(assetID))
    );
  }

  /**
   * Sets the value of an `asset_id` to root. Only callable by root.
   *
   * @param {ApiPromise} api Connected API Promise.
   * @param {Uint8Array|string} wallet Wallet making the transaction.
   * @param {u128} assetID Asset id to be set.
   */
  public static async grantRoot(
    api: ApiPromise,
    wallet: KeyringPair,
    assetID: u128
  ): Promise<IEvent<[Result<Null, SpRuntimeDispatchError>]>> {
    return await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(api.tx.governanceRegistry.grantRoot(assetID))
    );
  }
}
