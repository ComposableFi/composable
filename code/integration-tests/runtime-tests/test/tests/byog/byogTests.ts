import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { expect } from "chai";
import { u128 } from "@polkadot/types-codec";
import { ITuple } from "@polkadot/types/types";
import BN from "bn.js";

/**
 * Bring your own Gas tests
 *
 * BYOG allows XCM transfers with any asset registered in the XCMP pallet,
 * and has a ratio to PICA set.
 *
 * Currently only KSM is supported.
 * Though other assets can easily be added by modifying the XCMP pallet,
 * and defining a ratio.
 *
 * Tests:
 * 1.1. Set gas asset to `KSM`.
 * 1.2. Verify paying gas in `KSM` by a test transaction.
 *
 * 2.1. Set gas asset back to `PICA`
 * 2.2 Verify we`re now back to paying fees in `PICA`
 */
// describe(name, function) groups all query tests for the system pallet.
describe("[SHORT] BYOG Tests", function () {
  let api: ApiPromise;
  let sudoKey: KeyringPair, transactorWallet: KeyringPair;

  const PICA_ASSET_ID = 1;
  const KSM_ASSET_ID = 4;
  const KUSD_ASSET_ID = 129; // Used for test transactions.

  before("Setting up the tests", async function () {
    this.timeout(2 * 60 * 1000);
    // `getNewConnection()` establishes a new connection to the chain and gives us the ApiPromise & a Keyring.
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    // Using `getDevWallets(Keyring)` we're able to get a dict of all developer wallets.
    const { devWalletAlice } = getDevWallets(newKeyring);
    transactorWallet = devWalletAlice.derive("/test/byog/transactor");
    sudoKey = devWalletAlice;

    // Minting funds for wallets.
    await mintAssetsToWallet(
      api,
      transactorWallet,
      sudoKey,
      [PICA_ASSET_ID, KSM_ASSET_ID, KUSD_ASSET_ID],
      999_999_999_999_999n
    );
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("Can change gas asset to KSM", function () {
    it("User can set their gas asset ID to `KSM (4)`", async function () {
      this.timeout(2 * 60 * 1000);
      // Transaction parameters
      const newPaymentAsset = KSM_ASSET_ID;

      // Transaction
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        transactorWallet,
        api.events.system.ExtrinsicSuccess.is,
        api.tx.assetTxPayment.setPaymentAsset(transactorWallet.publicKey, newPaymentAsset)
      );

      // Verification
      expect(result).to.not.be.an("Error");
      const paymentAssetAfter = await api.query.assetTxPayment.paymentAssets(transactorWallet.publicKey);
      expect(paymentAssetAfter.unwrap()[0]).to.be.bignumber.equal(new BN(newPaymentAsset));
    });

    // Our test transaction is sending some funds to another wallet.
    it("A user can pay fees with KSM", async function () {
      this.timeout(2 * 60 * 1000);

      // Getting KSM funds before transaction
      const ksmFundsBeforeTransaction = await api.rpc.assets.balanceOf(
        KSM_ASSET_ID.toString(),
        transactorWallet.publicKey
      );
      // Getting PICA funds before transaction
      const picaFundsBeforeTransaction = await api.rpc.assets.balanceOf(
        PICA_ASSET_ID.toString(),
        transactorWallet.publicKey
      );

      // Parameters
      const assetToTransfer = KUSD_ASSET_ID; // Transferring `kUSD` token
      const transferAmount = 100_000_000_000;
      const receiverWallet = sudoKey.publicKey;
      const keepAlive = true;

      // Transaction
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        transactorWallet,
        api.events.system.ExtrinsicSuccess.is,
        api.tx.assets.transfer(assetToTransfer, receiverWallet, transferAmount, keepAlive)
      );

      // Verification
      expect(result).to.not.be.an("Error");

      // Getting KSM Funds after transaction
      const ksmFundsAfterTransaction = await api.rpc.assets.balanceOf(
        KSM_ASSET_ID.toString(),
        transactorWallet.publicKey
      );
      // Getting PICA Funds after transaction
      const picaFundsAfterTransaction = await api.rpc.assets.balanceOf(
        PICA_ASSET_ID.toString(),
        transactorWallet.publicKey
      );

      // Verifying fee has been paid in KSM
      expect(new BN(ksmFundsAfterTransaction.toString())).to.be.bignumber.lessThan(
        new BN(ksmFundsBeforeTransaction.toString())
      );

      // Making sure the PICA balance is untouched
      expect(new BN(picaFundsAfterTransaction.toString())).to.be.bignumber.equal(
        new BN(picaFundsBeforeTransaction.toString())
      );
    });

    // Our test transaction is sending some funds to multiple wallets.
    it("A user can use a different gas asset for batch transactions", async function () {
      this.timeout(2 * 60 * 1000);

      // Getting KSM funds before transaction
      const ksmFundsBeforeTransaction = await api.rpc.assets.balanceOf(
        KSM_ASSET_ID.toString(),
        transactorWallet.publicKey
      );
      // Getting PICA funds before transaction
      const picaFundsBeforeTransaction = await api.rpc.assets.balanceOf(
        PICA_ASSET_ID.toString(),
        transactorWallet.publicKey
      );

      // Parameters
      const assetToTransfer = KUSD_ASSET_ID; // Transferring `kUSD` token
      const transferAmount = 100_000_000_000;
      const receiverWallet = transactorWallet.derive("/receiver").publicKey;
      const receiverWallet2 = transactorWallet.derive("/receiver2").publicKey;
      const receiverWallet3 = transactorWallet.derive("/receiver3").publicKey;
      const receiverWallet4 = transactorWallet.derive("/receiver4").publicKey;
      const keepAlive = true;

      // Transaction
      const {
        data: [result]
      } = await sendWithBatchAndWaitForSuccess(
        api,
        transactorWallet,
        api.events.system.ExtrinsicSuccess.is,
        [
          api.tx.assets.transfer(assetToTransfer, receiverWallet, transferAmount, keepAlive),
          api.tx.assets.transfer(assetToTransfer, receiverWallet2, transferAmount, keepAlive),
          api.tx.assets.transfer(assetToTransfer, receiverWallet3, transferAmount, keepAlive),
          api.tx.assets.transfer(assetToTransfer, receiverWallet4, transferAmount, keepAlive)
        ],
        false
      );

      // Verification
      expect(result).to.not.be.an("Error");

      // Getting KSM Funds after transaction
      const ksmFundsAfterTransaction = await api.rpc.assets.balanceOf(
        KSM_ASSET_ID.toString(),
        transactorWallet.publicKey
      );
      // Getting PICA Funds after transaction
      const picaFundsAfterTransaction = await api.rpc.assets.balanceOf(
        PICA_ASSET_ID.toString(),
        transactorWallet.publicKey
      );

      // Verifying fee has been paid in KSM
      expect(new BN(ksmFundsAfterTransaction.toString())).to.be.bignumber.lessThan(
        new BN(ksmFundsBeforeTransaction.toString())
      );

      // Making sure the PICA balance is untouched
      expect(new BN(picaFundsAfterTransaction.toString())).to.be.bignumber.equal(
        new BN(picaFundsBeforeTransaction.toString())
      );
    });
  });

  describe("Can change gas asset back to PICA", function () {
    it("User can set their gas asset ID (again) to `PICA (1)`", async function () {
      this.timeout(2 * 60 * 1000);
      // Getting the previous set payment asset
      const paymentAssetBefore = await api.query.assetTxPayment.paymentAssets(transactorWallet.publicKey);

      // Transaction parameters
      const newPaymentAsset = null;

      // Transaction
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        transactorWallet,
        api.events.system.ExtrinsicSuccess.is,
        api.tx.assetTxPayment.setPaymentAsset(transactorWallet.publicKey, newPaymentAsset)
      );

      // Verification
      expect(result).to.not.be.an("Error");

      const paymentAssetAfter = await api.query.assetTxPayment.paymentAssets(transactorWallet.publicKey);
      // If the payment asset is PICA, it'll equal `undefined`
      expect(paymentAssetBefore.unwrapOr(undefined)).to.satisfy(
        (paymentAssetInfoBefore: undefined | ITuple<[u128, u128]>) => {
          return !!(paymentAssetInfoBefore && paymentAssetInfoBefore[0].eq(KSM_ASSET_ID));
        }
      );
      expect(paymentAssetAfter.unwrapOr(undefined)).to.be.undefined;
    });

    // Our test transaction is sending some funds to another wallet.
    it("Verifying the user is now paying fees with PICA again", async function () {
      this.timeout(2 * 60 * 1000);

      // Getting PICA funds before transaction
      const picaFundsBeforeTransaction = await api.rpc.assets.balanceOf(
        PICA_ASSET_ID.toString(),
        transactorWallet.publicKey
      );
      // Getting KSM funds before transaction
      const ksmFundsBeforeTransaction = await api.rpc.assets.balanceOf(
        KSM_ASSET_ID.toString(),
        transactorWallet.publicKey
      );

      // Parameters
      const assetToTransfer = KUSD_ASSET_ID; // Transferring `kUSD` token
      const transferAmount = 100_000_000_000;
      const receiverWallet = sudoKey.publicKey;
      const keepAlive = true;

      // Transaction
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        transactorWallet,
        api.events.system.ExtrinsicSuccess.is,
        api.tx.assets.transfer(assetToTransfer, receiverWallet, transferAmount, keepAlive)
      );

      // Verification
      expect(result).to.not.be.an("Error");

      // Getting PICA Funds after transaction
      const picaFundsAfterTransaction = await api.rpc.assets.balanceOf(
        PICA_ASSET_ID.toString(),
        transactorWallet.publicKey
      );
      // Getting KSM Funds after transaction
      const ksmFundsAfterTransaction = await api.rpc.assets.balanceOf(
        KSM_ASSET_ID.toString(),
        transactorWallet.publicKey
      );

      // Verifying fee has been paid in PICA
      expect(new BN(picaFundsAfterTransaction.toString())).to.be.bignumber.lessThan(
        new BN(picaFundsBeforeTransaction.toString())
      );

      // Making sure the KSM balance is untouched
      expect(new BN(ksmFundsAfterTransaction.toString())).to.be.bignumber.equal(
        new BN(ksmFundsBeforeTransaction.toString())
      );
    });
  });
});
