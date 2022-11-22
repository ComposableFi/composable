import { KeyringPair } from "@polkadot/keyring/types";
import testConfiguration from "./test_configuration.json";
import { ApiPromise, WsProvider } from "@polkadot/api";
import { sendAndWaitForSuccess, waitForBlocks } from "@composable/utils/polkadotjs";
import { SafeRpcWrapper, XcmV2TraitsOutcome, XcmVersionedMultiLocation } from "@composable/types/interfaces";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { expect } from "chai";
import BN from "bn.js";
import { u128 } from "@polkadot/types-codec";

/**
 * Contains tests for the XCMP system.
 *
 * 1. Transferring funds from 'RelayChain (KSM)' to Picasso/Dali
 * 2. The other way around with KSM.
 * 3. Again from Picasso/Dali to RelayChain with PICA.
 */
describe("[SHORT][LAUNCH] tx.xcmp Tests", function () {
  if (!testConfiguration.enabledTests.enabled) return;

  let api: ApiPromise;
  let walletAlice: KeyringPair;

  let relayChainApiClient: ApiPromise;
  let assetId: number;
  let ksmAssetID: SafeRpcWrapper;

  before(async function () {
    this.timeout(60 * 1000);
    // `getNewConnection()` establishes a new connection to the chain and gives us the ApiPromise & a Keyring.
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;

    // Using `getDevWallets(Keyring)` we're able to get a dict of all developer wallets.
    const { devWalletAlice } = getDevWallets(newKeyring);
    walletAlice = devWalletAlice;

    const relayChainEndpoint = "ws://" + (process.env.ENDPOINT_RELAYCHAIN ?? "127.0.0.1:9944");
    const relayChainProvider = new WsProvider(relayChainEndpoint);
    relayChainApiClient = await ApiPromise.create({
      provider: relayChainProvider,
      types: {
        XcmV2TraitsOutcome: {
          _enum: {
            Error: "Null",
            Complete: "u128",
            isError: "bool",
            isComplete: "bool"
          }
        }
      }
    });
    assetId = 4;
    ksmAssetID = api.createType("SafeRpcWrapper", assetId) as SafeRpcWrapper;
  });

  before("Providing assets for tests", async function () {
    this.timeout(2 * 60 * 1000);
    await mintAssetsToWallet(api, walletAlice, walletAlice, [1]);
  });

  after(async function () {
    await relayChainApiClient.disconnect();
    await api.disconnect();
  });

  /**
   * xcmPallet.reserveTransferAssets transfers an asset from parachain (Picasso) to a relayChain,
   * in this case the `Rococo Testnet`.
   *
   * Sudo command success is checked with `.isOk`.
   */
  describe("xcmPallet.reserveTransferAssets Success Test", function () {
    // Timeout set to 2 minutes
    this.timeout(10 * 60 * 1000);
    it("Can transfer asset(kUSD) from relay chain(KSM) to Picasso", async function () {
      if (!testConfiguration.enabledTests.addAssetAndInfo__success.add1) this.skip();

      // Setting the destination chain to Picasso/Dali
      const destination = relayChainApiClient.createType("XcmVersionedMultiLocation", {
        V0: relayChainApiClient.createType("XcmV0MultiLocation", {
          X1: relayChainApiClient.createType("XcmV0Junction", {
            Parachain: relayChainApiClient.createType("Compact<u32>", 2087)
          })
        })
      });

      // Setting the wallet receiving the funds
      const beneficiary = relayChainApiClient.createType("XcmVersionedMultiLocation", {
        V0: relayChainApiClient.createType("XcmV0MultiLocation", {
          X1: relayChainApiClient.createType("XcmV0Junction", {
            AccountId32: {
              network: relayChainApiClient.createType("XcmV0JunctionNetworkId", "Any"),
              id: walletAlice.publicKey
            }
          })
        })
      });

      const paraAmount = relayChainApiClient.createType("Compact<u128>", "100000000000000");

      // Setting up the asset & amount
      const assets = relayChainApiClient.createType("XcmVersionedMultiAssets", {
        V0: [
          relayChainApiClient.createType("XcmV0MultiAsset", {
            ConcreteFungible: {
              id: relayChainApiClient.createType("XcmV0MultiLocation", "Null"),
              amount: paraAmount
            }
          })
        ]
      });

      // Setting the asset which will be used for fees (0 refers to first in asset list)
      const feeAssetItem = relayChainApiClient.createType("u32", 0);

      // Getting Alice wallet balance before transaction.
      const walletBalanceAliceBeforeTransaction = await relayChainApiClient.query.system.account(walletAlice.publicKey);
      // Getting beneficiary wallet amount before transaction.
      const beneficiaryBalanceBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(ksmAssetID, walletAlice.publicKey)).toString()
      );

      // Making the transaction
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        relayChainApiClient,
        walletAlice,
        relayChainApiClient.events.xcmPallet.Attempted.is,
        relayChainApiClient.tx.xcmPallet.reserveTransferAssets(destination, beneficiary, assets, feeAssetItem)
      );
      await waitForBlocks(api, 3);

      // Verifying Stuff
      const convertedResult: XcmV2TraitsOutcome = relayChainApiClient.createType("XcmV2TraitsOutcome", result);
      // @ts-ignore
      expect(convertedResult.isComplete).to.be.true;
      // @ts-ignore
      expect(convertedResult.isError).to.be.false;

      // Getting Alice wallet balance after transaction.
      const walletBalanceAliceAfterTransaction = await relayChainApiClient.query.system.account(walletAlice.publicKey);
      expect(new BN(walletBalanceAliceAfterTransaction.data.free)).to.be.bignumber.lessThan(
        new BN(walletBalanceAliceBeforeTransaction.data.free)
      );
      // Beneficiary Wallet after transaction.
      const beneficiaryBalanceAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(ksmAssetID, walletAlice.publicKey)).toString()
      );

      expect(beneficiaryBalanceAfterTransaction).to.be.bignumber.greaterThan(beneficiaryBalanceBeforeTransaction);
    });
  });

  /**
   * Transfers an asset from RelayChain (Rococo Testnet) to Picasso/Dali.
   */
  describe("xTokens.transfer Success Test", function () {
    // Timeout set to 2 minutes
    this.timeout(10 * 60 * 1000);

    it("Can transfer KSM from Picasso to relay chain", async function () {
      // update name in test_configuration. Ask Dom
      if (!testConfiguration.enabledTests.addAssetAndInfo__success.add1) this.skip();

      //Set amount to transfer
      const amountToTransfer = relayChainApiClient.createType("u128", 10000000000000);

      //Set destination. Should have 2 Junctions, first to parent and then to wallet
      const destination = <XcmVersionedMultiLocation>api.createType("XcmVersionedMultiLocation", {
        V0: api.createType("XcmV0MultiLocation", {
          X2: [
            api.createType("XcmV0Junction", "Parent"),
            api.createType("XcmV0Junction", {
              AccountId32: {
                network: api.createType("XcmV0JunctionNetworkId", "Any"),
                id: walletAlice.publicKey
              }
            })
          ]
        })
      });

      // Set dest weight
      const destWeight = relayChainApiClient.createType("u64", 4000000000); // > 4000000000

      const transactorWalletBalanceBeforeTransaction = new BN(
        (await api.rpc.assets.balanceOf(ksmAssetID, walletAlice.publicKey)).toString()
      );

      const walletBalanceAliceBeforeTransaction = await relayChainApiClient.query.system.account(walletAlice.publicKey);
      //This tx pass on the parachain but encounter an error on relay. Barrier
      const {
        data: [resultTransactorAccountId, resultsAssetsList, resultMultiAsset, resultMultiLocation]
      } = await sendAndWaitForSuccess(
        api,
        walletAlice,
        api.events.xTokens.TransferredMultiAssets.is,
        api.tx.xTokens.transfer(<u128>new BN(ksmAssetID.toString()), amountToTransfer, destination, destWeight)
      );
      await waitForBlocks(api, 3);

      // Verifying Stuff
      const transactorWalletBalanceAfterTransaction = new BN(
        (await api.rpc.assets.balanceOf(ksmAssetID, walletAlice.publicKey)).toString()
      );

      const walletBalanceAliceAfterTransaction = await relayChainApiClient.query.system.account(walletAlice.publicKey);

      expect(new BN(walletBalanceAliceAfterTransaction.data.free))
        .to.be.bignumber.lessThan(new BN(walletBalanceAliceBeforeTransaction.data.free).add(amountToTransfer))
        .to.be.bignumber.greaterThan(new BN(walletBalanceAliceBeforeTransaction.data.free));

      expect(resultMultiLocation).to.not.be.an("Error");
      expect(resultTransactorAccountId).to.not.be.an("Error");
      expect(resultsAssetsList).to.not.be.an("Error");
      expect(resultMultiAsset).to.not.be.an("Error");

      expect(transactorWalletBalanceAfterTransaction).to.be.bignumber.lessThan(
        transactorWalletBalanceBeforeTransaction
      );
    });
  });
});
