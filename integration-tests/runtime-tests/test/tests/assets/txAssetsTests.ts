/* eslint @typescript-eslint/ban-ts-comment: "off" */
import { expect } from "chai";
import { ApiPromise } from "@polkadot/api";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import BN from "bn.js";

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
describe("[LAUNCH] tx.assets Tests", function () {
  if (!testConfiguration.enabledTests.tx.enabled) return;

  let api: ApiPromise;
  let sudoKey: KeyringPair, senderWallet: KeyringPair;

  before("Setting up the tests", async function () {
    this.timeout(60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    const { devWalletAlice, devWalletBob } = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    senderWallet = devWalletBob.derive("/tests/assets/transferTestSenderWallet");
  });

  before("Providing funds for tests", async function () {
    this.timeout(5 * 60 * 1000);
    await mintAssetsToWallet(api, sudoKey, sudoKey, [1]);
    await mintAssetsToWallet(api, senderWallet, sudoKey, [1, 4]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  /**
   * The `transfer` extrinsic transfers any `asset` from `origin` to `dest`.
   */
  describe("tx.assets.transfer Tests", function () {
    if (!testConfiguration.enabledTests.tx.transfer__success) return;

    it("[SHORT] A wallet can `transfer` KSM to another wallet", async function () {
      this.timeout(2 * 60 * 1000);

      const paraAsset = api.createType("u128", 4);
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const senderFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), senderWallet.publicKey)).toString()
      );
      expect(senderFundsBeforeTransaction.gt(new BN(0))).to.be.true;
      const receiverFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      // We ignore the results of the transaction here, since  we don't use it for verification.
      await sendAndWaitForSuccess(
        api,
        senderWallet,
        api.events.balances.Deposit.is,
        api.tx.assets.transfer(paraAsset, paraDest, paraAmount, paraKeepAlive)
      );

      const senderFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), senderWallet.publicKey)).toString()
      );
      const receiverFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      expect(senderFundsAfterTransaction.lt(senderFundsBeforeTransaction)).to.be.true;
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(new BN(paraAmount.toNumber())))).to.be
        .true;
    });
  });

  /**
   * The `transfer_native` extrinsic transfers the blockchains native asset (PICA) from `origin` to `dest`.
   */
  describe("tx.assets.transferNative Tests", function () {
    if (!testConfiguration.enabledTests.tx.transferNative__success) return;

    it("[SHORT] A wallet can `transfer_native` asset PICA to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const senderFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf("1", senderWallet.publicKey)).toString()
      );
      expect(senderFundsBeforeTransaction.gt(new BN(0))).to.be.true;
      const receiverFundsBeforeTransaction = new BN((await api.rpc.assets.balanceOf("1", paraDest)).toString());

      const {
        data: [resultAccountId, resultAccountId2, resultTransferAmount]
      } = await sendAndWaitForSuccess(
        api,
        senderWallet,
        api.events.balances.Transfer.is,
        api.tx.assets.transferNative(paraDest, paraAmount, paraKeepAlive)
      );

      const senderFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf("1", senderWallet.publicKey)).toString()
      );
      const receiverFundsAfterTransaction = new BN((await api.rpc.assets.balanceOf("1", paraDest)).toString());

      expect(senderFundsAfterTransaction.lt(senderFundsBeforeTransaction)).to.be.true;
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(new BN(paraAmount.toNumber())))).to.be
        .true;
      expect(resultAccountId.toString()).to.be.equal(api.createType("AccountId32", senderWallet.publicKey).toString());
      expect(resultAccountId2.toString()).to.be.equal(api.createType("AccountId32", paraDest).toString());
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(resultTransferAmount))).to.be.true;
    });
  });

  /**
   * The `force_transfer` extrinsic transfers any `asset` from `origin` to `dest` with sudo privileges.
   */
  describe("tx.assets.forceTransfer Tests", function () {
    if (!testConfiguration.enabledTests.tx.forceTransfer__success) return;

    it("A *sudo* wallet can `forceTransfer` KSM to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraSource = senderWallet.publicKey;
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const senderFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraSource)).toString()
      );
      expect(senderFundsBeforeTransaction.gt(new BN(0))).to.be.true;
      const receiverFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.forceTransfer(paraAsset, paraSource, paraDest, paraAmount, paraKeepAlive))
      );
      expect(result.isOk).to.be.true;

      const senderFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraSource)).toString()
      );
      const receiverFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      expect(senderFundsAfterTransaction.lt(senderFundsBeforeTransaction)).to.be.true;
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(new BN(paraAmount.toNumber())))).to.be
        .true;
    });
  });

  /**
   * The `force_transfer_native` extrinsic transfers the blockchains native asset (PICA) from `origin` to `dest`
   * with sudo privileges.
   */
  describe("tx.assets.force_transfer_native Tests", function () {
    if (!testConfiguration.enabledTests.tx.forceTransferNative__success) return;

    it("A *sudo* wallet can `force_transfer_native` token to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraSource = senderWallet.publicKey;
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraAmount = api.createType("Balance", 100000000000);
      const paraKeepAlive = api.createType("bool", true);

      const senderFundsBeforeTransaction = new BN((await api.rpc.assets.balanceOf("1", paraSource)).toString());
      expect(senderFundsBeforeTransaction.gt(new BN(0))).to.be.true;
      const receiverFundsBeforeTransaction = new BN((await api.rpc.assets.balanceOf("1", paraDest)).toString());

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.forceTransferNative(paraSource, paraDest, paraAmount, paraKeepAlive))
      );
      expect(result.isOk).to.be.true;

      const senderFundsAfterTransaction = new BN((await api.rpc.assets.balanceOf("1", paraSource)).toString());
      const receiverFundsAfterTransaction = new BN((await api.rpc.assets.balanceOf("1", paraDest)).toString());

      expect(senderFundsAfterTransaction.lt(senderFundsBeforeTransaction)).to.be.true;
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(new BN(paraAmount.toNumber())))).to.be
        .true;
    });
  });

  /**
   * The `transfer_all` extrinsic transfers the remaining balance of a specified `asset` from `origin` to `dest`.
   */
  describe("tx.assets.transfer_all Tests", function () {
    if (!testConfiguration.enabledTests.tx.transferAll__success) return;

    it("A wallet can `transfer_all` remaining KSM to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraKeepAlive = api.createType("bool", false);

      const senderFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), senderWallet.publicKey)).toString()
      );
      expect(senderFundsBeforeTransaction.gt(new BN(0))).to.be.true;
      const receiverFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      // We ignore the results of the transaction here, since  we don't use it for verification.
      await sendAndWaitForSuccess(
        api,
        senderWallet,
        api.events.balances.Deposit.is,
        api.tx.assets.transferAll(paraAsset, paraDest, paraKeepAlive)
      );

      // Verification
      const senderFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), senderWallet.publicKey)).toString()
      );
      const receiverFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      expect(senderFundsAfterTransaction.eq(new BN(0))).to.be.true;
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(senderFundsBeforeTransaction))).to.be
        .true;
    });
  });

  /**
   * The `transfer_all_native` extrinsic transfers the remaining balance of the blockchains native asset (PICA)
   * from `origin` to `dest`.
   */
  describe("tx.assets.transfer_all_native Tests", function () {
    if (!testConfiguration.enabledTests.tx.transferAllNative__success) return;

    it("A wallet can `transfer_all_native` PICA tokens to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraKeepAlive = api.createType("bool", false);

      const senderFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf("1", senderWallet.publicKey)).toString()
      );
      expect(senderFundsBeforeTransaction.gt(new BN(0))).to.be.true;
      const receiverFundsBeforeTransaction = new BN((await api.rpc.assets.balanceOf("1", paraDest)).toString());

      const {
        data: [resultAccountId, resultAccountId2, resultTransferAmount]
      } = await sendAndWaitForSuccess(
        api,
        senderWallet,
        api.events.balances.Transfer.is,
        api.tx.assets.transferAllNative(paraDest, paraKeepAlive)
      );

      const senderFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf("1", senderWallet.publicKey)).toString()
      );
      const receiverFundsAfterTransaction = new BN((await api.rpc.assets.balanceOf("1", paraDest)).toString());

      /*
      Verifying everything:
      - Make sure the old wallet has 0 funds left.
      - Make sure the wallet sending funds is correct.
      - Make sure the wallet receiving funds is correct.
      - Make sure the wallet receiving funds, received the correct amount reported by the event.
       */
      expect(senderFundsAfterTransaction.eq(new BN(0))).to.be.true;
      expect(resultAccountId.toString()).to.be.equal(api.createType("AccountId32", senderWallet.publicKey).toString());
      expect(resultAccountId2.toString()).to.be.equal(api.createType("AccountId32", paraDest).toString());
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(resultTransferAmount))).to.be.true;
    });
  });

  /**
   * The `mint_initialize` extrinsic creates a new asset & mints a defined `amount` into the `dest` wallet.
   */
  describe("tx.assets.mint_initialize Tests", function () {
    if (!testConfiguration.enabledTests.tx.mintInitialize) return;

    it("A *sudo* wallet can `mint_initialize` a new asset to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraAmount = api.createType("u128", 100000000000);
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.mintInitialize(paraAmount, paraDest))
      );
      expect(result.isOk).to.be.true;

      // Verifying everything
      const newAssetData = await api.query.currencyFactory.assetIdRanges();
      /*
       * From the list of available (unused) asset IDs we subtract `1` to get the latest created asset.
       * Seems like a weird way to get the asset ID, since there is `tokens.accounts` or `tokens.totalIssuance`
       * which look promising in the PolkadotJS web interface.
       * Though they don't seem to work because if I query these,
       * for some reason the asset ID gets stripped out of the result.
       *
       * Please ignore the ts-ignore, it's annoyed about `ranges` not being defined.
       * Trust me, I dislike this as much as you do!
       */
      // @ts-ignore
      const newAssetId = new BN(newAssetData.ranges[1].current.toString()).sub(new BN(1));
      const amountNewAssetAfterMinting = await api.query.tokens.accounts(paraDest, newAssetId);
      expect(amountNewAssetAfterMinting.free.eq(paraAmount)).to.be.true;
    });
  });

  /**
   * The `mint_initialize_with_governance` extrinsic creates a new asset, mints a certain `amount` into `dest` wallet.
   * > The `dest` account can use the democracy pallet to mint further assets,
   * > or if the `governance_origin` is set to an owned account, using signed transactions.
   * > In general the governance_origin should be generated from the pallet id.
   */
  describe("tx.assets.mint_initialize_with_governance Tests", function () {
    if (!testConfiguration.enabledTests.tx.mintInitializeWithGovernance) return;

    it("A *sudo* wallet can `mint_initialize_with_governance` a new asset to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraAmount = api.createType("u128", 100000000000);
      const paraGovernanceOrigin = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.mintInitializeWithGovernance(paraAmount, paraGovernanceOrigin, paraDest))
      );
      expect(result.isOk).to.be.true;

      // Verifying everything, please take a look at above's test case for further information.
      const newAssetData = await api.query.currencyFactory.assetIdRanges();
      // @ts-ignore
      const newAssetId = new BN(newAssetData.ranges[1].current.toString()).sub(new BN(1));
      const amountNewAssetAfterMinting = await api.query.tokens.accounts(paraDest, newAssetId);
      expect(amountNewAssetAfterMinting.free.eq(paraAmount)).to.be.true;
    });
  });

  /**
   * The `mint_into` extrinsic mints `amount` of `asset_id` into `dest` wallet.
   */
  describe("tx.assets.mint_into Tests", function () {
    if (!testConfiguration.enabledTests.tx.mintInto) return;

    it("A *sudo* wallet can `mintInto` KSM to another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraAmount = api.createType("u128", 100000000000);
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const receiverFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.mintInto(paraAsset, paraDest, paraAmount))
      );

      // Verification
      expect(result.isOk).to.be.true;
      const receiverFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.add(paraAmount))).to.be.true;
    });
  });

  /**
   * The `burn_from` extrinsic burns `amount` of `asset_id` of `dest` wallet.
   */
  describe("tx.assets.burn_from Tests", function () {
    // Check if group of tests are enabled.
    if (!testConfiguration.enabledTests.tx.burnFrom) return;

    // it(name, function) describes a single test.
    it("A *sudo* wallet can `burn_from` KSM from another wallet", async function () {
      this.timeout(2 * 60 * 1000);
      const paraAsset = api.createType("u128", 4);
      const paraAmount = api.createType("u128", 50000000000);
      const paraDest = senderWallet.derive("/tests/assets/transferTestReceiverWallet1").publicKey;

      const receiverFundsBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );

      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.burnFrom(paraAsset, paraDest, paraAmount))
      );

      // Verification
      expect(result.isOk).to.be.true;
      const receiverFundsAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(paraAsset.toString(), paraDest)).toString()
      );
      expect(receiverFundsAfterTransaction.eq(receiverFundsBeforeTransaction.sub(paraAmount))).to.be.true;
    });
  });
});
