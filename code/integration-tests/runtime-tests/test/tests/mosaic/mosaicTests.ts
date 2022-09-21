import { expect } from "chai";
import testConfiguration from "./test_configuration.json";
import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { CommonMosaicRemoteAssetId } from "@composable/types/interfaces";
import BN from "bn.js";
import { TxMosaicTests } from "@composabletests/tests/mosaic/testHandlers/mosaicTestHelper";
import { ApiPromise } from "@polkadot/api";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { AccountId32 } from "@polkadot/types/interfaces";
import { waitForBlocks } from "@composable/utils/polkadotjs";
import { u128 } from "@polkadot/types-codec";

/**
 * Mosaic Pallet Tests
 *  Checked functionalities are as follows;
 *  1. setRelayer
 *  2. rotateRelayer
 *  3. setNetwork
 *  4. setBudget
 *  5. transferTo
 *  6. acceptTransfer
 *  7. claimStaleTo
 *  8. timelockedMint
 *  9. setTimelockDuration
 * 10. rescindTimelockedMint
 * 11. claimTo
 * 12. updateAssetMapping
 *
 * This suite consists of happy path tests. Additionally, we started implementing suites for later references such as regression, smoke etc.
 *
 */
describe("[LAUNCH] tx.mosaic Tests", function () {
  // Check if group of tests are enabled.
  if (!testConfiguration.enabledTests.query.enabled) return;

  let api: ApiPromise;

  let sudoKey: KeyringPair,
    startRelayerWallet: KeyringPair,
    newRelayerWallet: KeyringPair,
    userWallet: KeyringPair,
    remoteAssetId: CommonMosaicRemoteAssetId;

  let transferAmount: number, assetId: number, networkId: number;

  let pNetworkId: u128;
  let ethAddress: string;

  describe("tx.mosaic Tests", function () {
    this.timeout(4 * 60 * 1000);
    if (!testConfiguration.enabledTests.query.account__success.enabled) return;

    before("Setting up the tests", async function () {
      this.timeout(4 * 60 * 1000);
      const { newClient, newKeyring } = await getNewConnection();
      api = newClient;
      const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
      sudoKey = devWalletAlice;
      startRelayerWallet = devWalletEve.derive("/tests/mosaicPallets/wallet1");
      newRelayerWallet = devWalletAlice.derive("/tests/mosaicPallets/wallet2");
      userWallet = devWalletFerdie.derive("/tests/mosaicPallets/wallet3");
      assetId = 4;
      transferAmount = 100000000000;
      networkId = 1;
      pNetworkId = api.createType("u128", 1);
      ethAddress = "0x";
      remoteAssetId = api.createType("CommonMosaicRemoteAssetId", {
        EthereumTokenAddress: api.createType("[u8;20]", "0x")
      });
    });

    before("Mint available assets into wallets", async function () {
      this.timeout(5 * 60 * 1000);
      await mintAssetsToWallet(api, startRelayerWallet, sudoKey, [1, 4]);
      await mintAssetsToWallet(api, newRelayerWallet, sudoKey, [1, 4]);
      await mintAssetsToWallet(api, userWallet, sudoKey, [1, 4]);
    });

    after("Closing the connection", async function () {
      await api.disconnect();
    });

    /**
     * Setting the first relayer.
     * Sudo call therefore result is checked by `.isOk`.
     */
    it("Should be able to set relayer @integration", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      const {
        data: [result]
      } = await TxMosaicTests.testSetRelayer(api, sudoKey, startRelayerWallet.address);
      expect(result.isOk).to.be.true;
    });

    /**
     * Setting the network.
     */
    it("Should be able to set the network @integration", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      const networkInfo = api.createType("PalletMosaicNetworkInfo", {
        enabled: api.createType("bool", true),
        minTransferSize: api.createType("u128", 0),
        maxTransferSize: api.createType("u128", 800000000000)
      });
      const {
        data: [retNetworkId, retNetworkInfo]
      } = await TxMosaicTests.testSetNetwork(api, startRelayerWallet, pNetworkId, networkInfo);
      expect(retNetworkId).to.not.be.an("Error");
      expect(retNetworkInfo).to.not.be.an("Error");
      //Verifies the newly created networkId
      expect(retNetworkId.toNumber()).to.be.equal(networkId);
    });

    /**
     * Setting the budget.
     * A sudo call therefore result is verified by isOk.
     */
    it("Should be able set the budget", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      const transAmount = 800000000000000;
      const pDecay = api.createType("PalletMosaicDecayBudgetPenaltyDecayer", {
        Linear: api.createType("PalletMosaicDecayLinearDecay", { factor: api.createType("u128", 5) })
      });
      const {
        data: [result]
      } = await TxMosaicTests.testSetBudget(api, sudoKey, assetId, transAmount, pDecay);
      expect(result.isOk).to.be.true;
    });

    it("Should be able to update asset mapping", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.updateAssetMapping) this.skip();
      const {
        data: [result]
      } = await TxMosaicTests.testUpdateAssetMapping(api, sudoKey, assetId, pNetworkId, remoteAssetId);
      expect(result.isOk).to.be.true;
    });

    it("Should be able to send transfers to another network, creating an outgoing transaction", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.sendTransferTo) this.skip();
      const {
        data: [result]
      } = await TxMosaicTests.testTransferTo(api, startRelayerWallet, pNetworkId, assetId, ethAddress, transferAmount);
      expect(result).to.not.be.an("Error");
      const lockedAmount = await api.query.mosaic.outgoingTransactions(startRelayerWallet.address, assetId);
      //verify that the amount sent is locked in the outgoing pool.
      expect(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
    });

    it("Relayer should be able to mint assets into pallet wallet with timelock//simulates incoming transfer from pair network", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.relayerCanMintAssets) this.skip();
      const toTransfer = newRelayerWallet.address;
      await TxMosaicTests.lockFunds(api, startRelayerWallet, pNetworkId, remoteAssetId, toTransfer, transferAmount);
      const lockedAmount = await api.query.mosaic.incomingTransactions(toTransfer, assetId);
      //verify that the incoming transaction is locked in the incoming transaction pool.
      expect(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
    });

    it("Other users should be able to mint assets into pallet wallet with timelock//simulates incoming transfer from pair network", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.userCanMintAssets) this.skip();
      const toTransfer = userWallet.address;
      await TxMosaicTests.lockFunds(api, startRelayerWallet, pNetworkId, remoteAssetId, toTransfer, transferAmount);
      const lockedAmount = await api.query.mosaic.incomingTransactions(toTransfer, assetId);
      //verify that the incoming transaction is locked in the incoming transaction pool.
      expect(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
    });

    it("Only relayer should mint assets into pallet wallet with timelock/incoming transactions (Failure Test)", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.OnlyRelayerCanMintAssets) this.skip();
      const toTransfer = newRelayerWallet.address;
      //verify that the transaction fails with BadOrigin message
      await TxMosaicTests.lockFunds(api, userWallet, pNetworkId, remoteAssetId, toTransfer, transferAmount).catch(
        error => expect(error.message).to.contain("BadOrigin")
      );
    });

    /**
     * Rotating the relayer.
     * Sudo call therefore result is checked by `.isOk`.
     */
    it("Should be able to rotate relayer", async function () {
      if (!testConfiguration.enabledTests.query.account__success.balanceGTZero1) this.skip();
      const {
        data: [result]
      } = await TxMosaicTests.testRotateRelayer(api, startRelayerWallet, newRelayerWallet.address);
      expect(result).to.not.be.an("Error");
      const relayerInfo = await api.query.mosaic.relayer();
      //verify that the relayer records information about the next relayer wallet
      expect(relayerInfo.unwrap().relayer.next.toJSON().account).to.be.equal(
        api.createType("AccountId32", newRelayerWallet.address).toString()
      );
    });

    it("Should the finality issues occur, relayer can burn untrusted amounts from tx", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.relayerCanRescindTimeLockFunds) this.skip();
      const wallet = startRelayerWallet;
      const returnWallet = newRelayerWallet;
      const {
        data: [result]
      } = await TxMosaicTests.testRescindTimeLockedFunds(api, wallet, returnWallet, remoteAssetId, transferAmount);
      //We can change the assertion, get the info from chain from incoming pool and verify that the amount locked is reduced from the amount total
      expect(result.toString()).to.be.equal(api.createType("AccountId32", newRelayerWallet.address).toString());
    });

    it("Only relayer should be able to burn untrusted amounts from incoming tx (Failure Test)", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.OnlyRelayerCanRescindTimeLockFunds) this.skip();
      const wallet = userWallet;
      const returnWallet = newRelayerWallet;
      await TxMosaicTests.testRescindTimeLockedFunds(api, wallet, returnWallet, remoteAssetId, transferAmount).catch(
        error => expect(error.message).to.contain("BadOrigin")
      );
    });

    it("Other users should be able to send transfers to another network, creating an outgoing transaction", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.userCanCreateOutgoingTransaction) this.skip();
      const paramRemoteTokenContAdd = "0x0423276a1da214B094D54386a1Fb8489A9d32730";
      const {
        data: [result]
      } = await TxMosaicTests.testTransferTo(
        api,
        userWallet,
        pNetworkId,
        assetId,
        paramRemoteTokenContAdd,
        transferAmount
      );
      expect(result).to.not.be.an("Error");
      const lockedAmount = await api.query.mosaic.outgoingTransactions(userWallet.address, assetId);
      //Verify that the transferred amount is locked in the outgoing transaction pool.
      expect(lockedAmount.unwrap()[0].toNumber()).to.be.equal(transferAmount);
    });

    it("Relayer should be able to accept outgoing transfer", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.relayerAcceptTransfer) this.skip();
      this.timeout(2 * 60 * 1000);
      const senderWallet = userWallet;
      const {
        data: [result]
      } = await TxMosaicTests.testAcceptTransfer(
        api,
        startRelayerWallet,
        senderWallet,
        pNetworkId,
        remoteAssetId,
        transferAmount
      );
      //verify that the relayer address is returned.
      expect(result.toString()).to.be.equal(api.createType("AccountId32", senderWallet.address).toString());
    });

    it("Only receiver should be able to claim incoming transfers (Failure Test)", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.OnlyReceiverCanClaimTransaction) this.skip();
      this.timeout(2 * 60 * 1000);
      const receiverWallet = startRelayerWallet;
      await TxMosaicTests.testClaimTransactions(api, receiverWallet, receiverWallet, assetId).catch(error => {
        expect(error.message).to.contain("NoClaimable");
      });
    });

    it("Receiver should be able to claim incoming transfers", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.receiverCanClaimTransaction) this.skip();
      this.timeout(2 * 60 * 1000);
      const receiverWallet = userWallet;
      const initialTokens = await api.query.tokens.accounts(userWallet.address, assetId);
      const {
        data: [result]
      } = await TxMosaicTests.testClaimTransactions(api, userWallet, receiverWallet, assetId);
      expect(result).to.not.be.an("Error");
      const afterTokens = await api.query.tokens.accounts(userWallet.address, assetId);
      expect(new BN(initialTokens.free).eq(new BN(afterTokens.free).sub(new BN(transferAmount)))).to.be.true;
    });

    it("User should be able to reclaim the stale funds not accepted by the relayer and locked in outgoing transactions pool", async function () {
      // Check if this test is enabled.
      if (!testConfiguration.enabledTests.userCanClaimStaleFunds) this.skip();
      this.timeout(5 * 60 * 1000);
      const wallet = startRelayerWallet;
      const initialTokens = await api.query.tokens.accounts(wallet.address, assetId);
      let retry = true;
      let finalResult: string | AccountId32;
      while (retry) {
        await waitForBlocks(api, 2);
        const {
          // @ts-ignore
          data: [result]
        } = await TxMosaicTests.testClaimStaleFunds(api, startRelayerWallet, assetId).catch(error => {
          if (error.message.includes("TxStillLocked")) return { data: ["Retrying..."] };
        });
        if (result !== "Retrying...") {
          retry = false;
          finalResult = result;
        }
      }
      // @ts-ignore
      expect(finalResult).to.not.be.an("Error");
      const afterTokens = await api.query.tokens.accounts(wallet.address, assetId);
      //verify that the reclaimed tokens are transferred into user balance.
      expect(new BN(initialTokens.free).eq(new BN(afterTokens.free).sub(new BN(transferAmount)))).to.be.true;
    });
  });
});
