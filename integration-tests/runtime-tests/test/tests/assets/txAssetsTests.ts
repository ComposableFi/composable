import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";

/**
 * Assets Pallet Extrinsics Integration Test
 *
 * In these tests we're testing the following extrinsics:
 * - Transfer
 * - Transfer Native
 * - Force Transfer
 * - Force Transfer Native
 * - Transfer All
 * - Transfer All Native
 * - Mint initialize
 * - Mint initialize with Governance
 * - Mint Into
 * - Burn From
 */
// describe(name, function) groups all query tests for the system pallet.
describe.only("tx.assets Tests", function() {
  // Check if group of tests are enabled.
  if (!testConfiguration.enabledTests.tx.enabled)
    return;

  let api: ApiPromise;
  let sudoKey: KeyringPair,
    walletBob: KeyringPair;

  before("Setting up the tests", async function() {
    this.timeout(60 * 1000);
    // `getNewConnection()` establishes a new connection to the chain and gives us the ApiPromise & a Keyring.
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    // Using `getDevWallets(Keyring)` we're able to get a dict of all developer wallets.
    const {
      devWalletAlice,
      devWalletBob
    } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletBob = devWalletBob;
  });

  before("Providing funds for tests", async function() {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, sudoKey, sudoKey, [1]);
    await mintAssetsToWallet(api, walletBob, sudoKey, [1, 4]);
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  /**
   * The `transfer` extrinsic transfers any `asset` from `origin` to `dest`.
   */
  describe("tx.assets.transfer Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A wallet can `transfer` KSM to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        walletBob,
        api.events.balances.Deposit.is,
        api.tx.assets.transfer(paraAsset, paraDest, paraAmount, paraKeepAlive)
      );

      console.debug(result);
    });
  });

  /**
   * The `transfer_native` extrinsic transfers the blockchains native asset (PICA) from `origin` to `dest`.
   */
  describe("tx.assets.transferNative Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A wallet can `transfer` native asset PICA to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        walletBob,
        api.events.balances.Deposit.is,
        api.tx.assets.transferNative(paraDest, paraAmount, paraKeepAlive)
      );

      console.debug(result);
    });
  });

  /**
   * The `force_transfer` extrinsic transfers any `asset` from `origin` to `dest` with sudo privileges.
   */
  describe("tx.assets.forceTransfer Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A *sudo* wallet can `forceTransfer` KSM to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraSource = walletBob.publicKey;
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(
          api.tx.assets.forceTransfer(
            paraAsset,
            paraSource,
            paraDest,
            paraAmount,
            paraKeepAlive
          )
        )
      );

      console.debug(result);
    });
  });

  /**
   * The `force_transfer_native` extrinsic transfers the blockchains native asset (PICA) from `origin` to `dest`
   * with sudo privileges.
   */
  describe("tx.assets.transfer Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A *sudo* wallet can `force_transfer_native` token to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraSource = walletBob.publicKey;
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(
          api.tx.assets.forceTransferNative(
            paraSource,
            paraDest,
            paraAmount,
            paraKeepAlive
          )
        )
      );

      console.debug(result);
    });
  });

  /**
   * The `transfer_all` extrinsic transfers the remaining balance of a specified `asset` from `origin` to `dest`.
   */
  describe("tx.assets.transfer_all Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A wallet can `transfer_all` remaining KSM to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraKeepAlive = api.createType("bool", false);

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        walletBob,
        api.events.balances.Deposit.is,
        api.tx.assets.transferAll(paraAsset, paraDest, paraKeepAlive)
      );

      console.debug(result);
    });
  });

  /**
   * The `transfer_all_native` extrinsic transfers the remaining balance of the blockchains native asset (PICA)
   * from `origin` to `dest`.
   */
  describe("tx.assets.transfer_all_native Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A wallet can `transfer_all_native` tokens to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraKeepAlive = api.createType("bool", false);

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        walletBob,
        api.events.balances.Deposit.is,
        api.tx.assets.transferAllNative(paraDest, paraKeepAlive)
      );

      console.debug(result);
    });
  });

  /**
   * The `mint_initialize` extrinsic creates a new asset & mints a defined `amount` into the `dest` wallet.
   */
  describe("tx.assets.mint_initialize Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A *sudo* wallet can `mint_initialize` a new asset to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraAmount = api.createType("u128", 100000000000);
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(
          api.tx.assets.mintInitialize(paraAmount, paraDest)
        )
      );

      console.debug(result);
    });
  });

  /**
   * The `mint_initialize_with_governance` extrinsic creates a new asset, mints a certain `amount` into `dest` wallet.
   * > The `dest` account can use the democracy pallet to mint further assets,
   * > or if the `governance_origin` is set to an owned account, using signed transactions.
   * > In general the governance_origin should be generated from the pallet id.
   */
  describe("tx.assets.mint_initialize_with_governance Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A *sudo* wallet can `mint_initialize_with_governance` a new asset to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraAmount = api.createType("u128", 100000000000);
      const paraGovernanceOrigin = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(
          api.tx.assets.mintInitializeWithGovernance(paraAmount, paraGovernanceOrigin, paraDest)
        )
      );

      console.debug(result);
    });
  });

  /**
   * The `mint_into` extrinsic mints `amount` of `asset_id` into `dest` wallet.
   */
  describe("tx.assets.mint_into Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A *sudo* wallet can `mintInto` KSM to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraAmount = api.createType("u128", 4);
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(
          api.tx.assets.mintInto(paraAsset, paraDest, paraAmount)
        )
      );

      console.debug(result);
    });
  });

  /**
   * The `burn_from` extrinsic burns `amount` of `asset_id` of `dest` wallet.
   */
  describe("tx.assets.burn_from Tests", function() {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    // it(name, function) describes a single test.
    it("A *sudo* wallet can `burn_from` KSM to another wallet", async function() {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraAmount = api.createType("u128", 4);
      const paraDest = walletBob.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const { data: [result] } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(
          api.tx.assets.burnFrom(paraAsset, paraDest, paraAmount)
        )
      );

      console.debug(result);
    });
  });
});
