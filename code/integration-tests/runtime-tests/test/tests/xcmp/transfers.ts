import { KeyringPair } from "@polkadot/keyring/types";
import testConfiguration from "./test_configuration.json";
import { ApiPromise } from "@polkadot/api";
import { waitForBlocks } from "@composable/utils/polkadotjs";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { Pica } from "@composable/utils/mintingHelper";
import { expect } from "chai";
import { u8aToHex } from "@polkadot/util"
import { decodeAddress } from "@polkadot/util-crypto"
import BN from "bn.js";
import {
  calculateTransferredAmount,
  changeGasToken,
  disconnectApis,
  fetchNativeBalance,
  fetchTokenBalance,
  fetchTotalIssuance,
  fetchXChainTokenBalances,
  initializeApis,
  mintTokensOnStatemine,
  registerAssetOnStatemine,
  saveTrappedAssets,
  sendAssetToRelaychain,
  sendFundsFromRelaychain,
  sendKSMFromStatemine,
  sendPicaToAnotherAccount,
  sendTokenToStatemine,
  sendUnknownFromStatemine,
  sendUSDTFromStatemine,
  sendXCMMessageFromRelayer,
  setAssetStatusMessage,
  setChainsForTests,
  to6Digit,
  trapAssetsOnKusama
} from "@composabletests/tests/xcmp/transfersTestHelper";

/**
 * Contains tests for the XCMP system.
 *
 * 1. Transferring (KSM) funds from 'RelayChain (Kusama)' to Picasso/Dali
 * 2. Transferrin
 */
describe("[SHORT][LAUNCH] tx.xcmp Tests", function () {
  if (!testConfiguration.enabledTests.enabled) return;
  let picassoApi: ApiPromise;
  let kusamaApi: ApiPromise;
  let statemineApi: ApiPromise;
  let devWalletAlice: KeyringPair, devWalletFerdie: KeyringPair;
  const { usdtOnStatemine, usdtOnPicasso, statemineParaId, ksm, destWeight, treasuryAddress, picassoParaAccount } =
    testConfiguration.testData.staticValues;
  const fees = testConfiguration.testData.fees;
  const existentialDeposits = testConfiguration.testData.existentialDeposits;
  let totalIssuance: BN;

  before(async function () {
    this.timeout(3 * 60 * 1000);
    const { newClient, newKeyring } = await getNewConnection();
    ({ picassoApi, kusamaApi, statemineApi } = await initializeApis(newClient));
    ({ devWalletAlice, devWalletFerdie } = getDevWallets(newKeyring));
    await setChainsForTests(
      kusamaApi,
      statemineApi,
      picassoApi,
      devWalletAlice,
      usdtOnStatemine,
      1_000,
      statemineParaId
    );
  });

  after(async function () {
    await disconnectApis(picassoApi, kusamaApi, statemineApi);
  });
  /**
   * Tests between relaychain and picasso
   */
  describe("xcmPallet transfers between Kusama to Picasso", function () {
    if (!testConfiguration.enabledTests.kusamaTests.enabled) {
      console.warn("Transfers between Kusama and Picasso are being skipped...");
      return;
    }
    // Timeout set to 10 minutes
    this.timeout(10 * 60 * 1000);
    const transferAmountToPicasso = 10;
    const transferAmountFromPicasso = 5;

    it("When users have sufficient funds on Kusama, I can send KSM to Picasso", async function () {
      const aliceBeforeBalanceOnPicasso = await fetchTokenBalance(picassoApi, devWalletAlice, ksm);
      const {
        data: [result]
      } = await sendFundsFromRelaychain(kusamaApi, devWalletAlice, transferAmountToPicasso);
      await waitForBlocks(picassoApi, 2);
      const aliceAfterBalanceOnPicasso = await fetchTokenBalance(picassoApi, devWalletAlice, ksm);
      const expectedAmount = calculateTransferredAmount(transferAmountToPicasso, fees.toPicasso.ksm);
      totalIssuance = await fetchTotalIssuance(picassoApi, ksm);
      expect(result.toString()).to.contain("complete");
      expect(aliceAfterBalanceOnPicasso.sub(aliceBeforeBalanceOnPicasso)).to.be.bignumber.equal(expectedAmount);
      expect(totalIssuance).to.be.bignumber.equal(new BN(Pica(transferAmountToPicasso).toString()));
    });
    it("When users don't have any Pica, I can set payment asset to newly transferred KSM", async function () {
      const {
        data: [currencyId, accountId, amount]
      } = await changeGasToken(picassoApi, devWalletAlice, ksm);
      expect(currencyId.toNumber()).to.be.equal(ksm);
      expect(accountId.toString()).to.be.equal(picassoApi.createType("AccountId32", devWalletAlice.address).toString());
      expect(amount.toNumber()).to.be.equal(existentialDeposits.ksm);
    });
    it("When users set KSM as payment asset, I can pay gas fee in KSM", async function () {
      const [treasuryBalanceBefore, ksmBalanceOfUserBefore] = await fetchXChainTokenBalances(
        [picassoApi],
        [treasuryAddress, devWalletAlice],
        [ksm]
      );
      await sendPicaToAnotherAccount(picassoApi, devWalletAlice, devWalletFerdie, 4, 2);
      const [treasuryBalanceAfter, ksmBalanceOfUserAfter] = await fetchXChainTokenBalances(
        [picassoApi],
        [treasuryAddress, devWalletAlice],
        [ksm]
      );
      expect(treasuryBalanceAfter).to.be.bignumber.greaterThan(treasuryBalanceBefore);
      expect(treasuryBalanceAfter.sub(treasuryBalanceBefore)).to.be.bignumber.equal(
        ksmBalanceOfUserBefore.sub(ksmBalanceOfUserAfter)
      );
    });
    it("The total issuance information stays the same after changing gas asset to KSM", async function () {
      const totalIssuanceAfterTxs = await fetchTotalIssuance(picassoApi, ksm);
      expect(totalIssuanceAfterTxs).to.be.bignumber.equal(totalIssuance);
    });
    it("Users can transfer asset(KSM) from Picasso to relay chain(Kusama).", async function () {
      const alicePreBalanceOnKSM = await fetchNativeBalance(kusamaApi, devWalletAlice);
      const [alicePreBalance, treasuryPreBalance] = await fetchXChainTokenBalances(
        [picassoApi],
        [devWalletAlice, treasuryAddress],
        [ksm]
      );
      await sendAssetToRelaychain(picassoApi, devWalletAlice, devWalletAlice, transferAmountFromPicasso, destWeight);
      await waitForBlocks(kusamaApi, 1);
      const [aliceAfterBalance, treasuryAfterBalance] = await fetchXChainTokenBalances(
        [picassoApi],
        [devWalletAlice, treasuryAddress],
        [ksm]
      );
      const aliceAfterBalanceOnKSM = await fetchNativeBalance(kusamaApi, devWalletAlice);
      const gasFeePaid = treasuryAfterBalance.sub(treasuryPreBalance);
      const expectedAmount = calculateTransferredAmount(transferAmountFromPicasso, fees.toKusama.ksm);
      expect(expectedAmount).to.be.bignumber.equal(aliceAfterBalanceOnKSM.sub(alicePreBalanceOnKSM));
      expect(aliceAfterBalance).to.be.bignumber.equal(alicePreBalance.sub(new BN(Pica(transferAmountFromPicasso).toString())).sub(gasFeePaid));
    });
    it("Total issuance changes correctly with xchain transfers", async function () {
      const totalIssuanceAfter = await fetchTotalIssuance(picassoApi, ksm);
      expect(totalIssuanceAfter).to.be.bignumber.equal(totalIssuance.sub(new BN(Pica(transferAmountFromPicasso).toString())));
    });
  });
  describe("Transfers between Statemine and Picasso", function () {
    if (!testConfiguration.enabledTests.statemineTests.enabled) {
      console.warn("Transfers between Statemine and Picasso are being skipped...");
      return;
    }
    const transferAmount = 1;
    this.timeout(3 * 60 * 1000);
    it("Given that users have KSM on Statemine, they can  send Ksm to Picasso", async function () {
      const [alicePreBalanceOnStatemine, alicePreBalanceOnPicasso] = await fetchXChainTokenBalances(
        [statemineApi, picassoApi],
        [devWalletAlice],
        [0, ksm]
      );
      const {
        data: [result]
      } = await sendKSMFromStatemine(statemineApi, devWalletAlice, devWalletAlice, transferAmount);
      await waitForBlocks(picassoApi, 2);
      const [aliceAfterBalanceOnStatemine, aliceAfterBalanceOnPicasso] = await fetchXChainTokenBalances(
        [statemineApi, picassoApi],
        [devWalletAlice],
        [0, ksm]
      );
      const expectedAmount = calculateTransferredAmount(transferAmount, fees.toPicasso.ksm);
      expect(result.toString()).to.contain("complete");
      expect(expectedAmount).to.be.bignumber.equal(aliceAfterBalanceOnPicasso.sub(alicePreBalanceOnPicasso));
      expect(alicePreBalanceOnStatemine).to.be.bignumber.greaterThan(
        aliceAfterBalanceOnStatemine.add(new BN(Pica(transferAmount).toString()))
      );
    });

    it("Users can send USDT from Statemine to Picasso", async function () {
      const transferAmount = 13;
      const [alicePreBalanceOnStatemine, alicePreBalanceOnPicasso] = await fetchXChainTokenBalances(
        [statemineApi, picassoApi],
        [devWalletAlice],
        [usdtOnStatemine, usdtOnPicasso]
      );
      const {
        data: [result]
      } = await sendUSDTFromStatemine(statemineApi, devWalletAlice, devWalletAlice, transferAmount);
      await waitForBlocks(picassoApi, 2);
      const [aliceAfterBalanceOnStatemine, aliceAfterBalanceOnPicasso] = await fetchXChainTokenBalances(
        [statemineApi, picassoApi],
        [devWalletAlice],
        [usdtOnStatemine, usdtOnPicasso]
      );
      const transferredAmount = to6Digit(transferAmount) - fees.toPicasso.usdt;
      expect(aliceAfterBalanceOnStatemine).to.be.bignumber.equal(alicePreBalanceOnStatemine.sub(new BN(to6Digit(13))));
      expect(aliceAfterBalanceOnPicasso).to.be.bignumber.equal(alicePreBalanceOnPicasso.add(new BN(transferredAmount)));
    });

    it("Users can change to pay with USDT", async function () {
      const {
        data: [currencyId, accountId, amount]
      } = await changeGasToken(picassoApi, devWalletAlice, usdtOnPicasso);
      expect(currencyId.toNumber()).to.be.equal(usdtOnPicasso);
      expect(accountId.toString()).to.be.equal(picassoApi.createType("AccountId32", devWalletAlice.address).toString());
      expect(amount.toNumber()).to.be.equal(existentialDeposits.usdt);
    });

    it("Users can pay tx fee with USDT", async function () {
      const [treasuryBalanceBefore, ksmBalanceOfUserBefore] = await fetchXChainTokenBalances(
        [picassoApi],
        [treasuryAddress, devWalletAlice],
        [usdtOnPicasso]
      );
      await sendPicaToAnotherAccount(picassoApi, devWalletAlice, devWalletFerdie, 4, 2);
      const [treasuryBalanceAfter, ksmBalanceOfUserAfter] = await fetchXChainTokenBalances(
        [picassoApi],
        [treasuryAddress, devWalletAlice],
        [usdtOnPicasso]
      );
      expect(treasuryBalanceAfter).to.be.bignumber.greaterThan(treasuryBalanceBefore);
      expect(treasuryBalanceAfter.sub(treasuryBalanceBefore)).to.be.bignumber.equal(
        ksmBalanceOfUserBefore.sub(ksmBalanceOfUserAfter)
      );
    });

    it("Users can send USDT from Picasso to Statemine", async function () {
      const transferAmount = 2;
      const treasuryPreBalance = await fetchTokenBalance(picassoApi, treasuryAddress, usdtOnPicasso);
      const [preAliceBalanceOnPicasso, preAliceBalanceOnStatemine] = await fetchXChainTokenBalances(
        [picassoApi, statemineApi],
        [devWalletAlice],
        [usdtOnPicasso, usdtOnStatemine]
      );
      await sendTokenToStatemine(picassoApi, devWalletAlice, devWalletAlice, transferAmount, usdtOnStatemine);
      await waitForBlocks(picassoApi, 2);
      const treasuryAfterBalance = await fetchTokenBalance(picassoApi, treasuryAddress, usdtOnPicasso);
      const gasFeePaid = treasuryAfterBalance.sub(treasuryPreBalance);
      const [afterAliceBalanceOnPicasso, afterAliceBalanceOnStatemine] = await fetchXChainTokenBalances(
        [picassoApi, statemineApi],
        [devWalletAlice],
        [usdtOnPicasso, usdtOnStatemine]
      );
      expect(preAliceBalanceOnPicasso.sub(new BN(to6Digit(transferAmount)))).to.be.bignumber.equal(
        afterAliceBalanceOnPicasso.add(gasFeePaid)
      );
      expect(preAliceBalanceOnStatemine.add(new BN(to6Digit(transferAmount)))).to.be.bignumber.equal(
        afterAliceBalanceOnStatemine
      );
    });
  });
  describe("Trapped assets and unknown tokens tests", function () {
    if (!testConfiguration.enabledTests.trapAndUnknownTests.enabled) {
      console.warn("Trapped assets and unknown tokens tests are being skipped...");
      return;
    }
    this.timeout(8 * 60 * 1000);
    const transferAmount = 3;
    const unknownAsset = 1985;

    before("Creates another asset for tests", async function () {
      await registerAssetOnStatemine(statemineApi, devWalletAlice, unknownAsset);
      const payload = setAssetStatusMessage(statemineApi, devWalletAlice, 5000, unknownAsset);
      await sendXCMMessageFromRelayer(kusamaApi, devWalletAlice, payload);
      await mintTokensOnStatemine(statemineApi, unknownAsset, devWalletAlice);
    });

    it("Users can trap assets on Kusama with falsy xcm messages", async function () {
      const paraAccountPreBalance = await fetchNativeBalance(kusamaApi, picassoParaAccount);
      await trapAssetsOnKusama(picassoApi, devWalletAlice, transferAmount);
      await waitForBlocks(kusamaApi, 1);
      const paraAccountAfterBalance = await fetchNativeBalance(kusamaApi, picassoParaAccount);
      expect(paraAccountPreBalance.sub(paraAccountAfterBalance)).to.be.bignumber.equal(
        new BN(Pica(transferAmount).toString())
      );
    });

    it("Users can save trapped assets with xcm messages", async function () {
      const preBalance = await fetchNativeBalance(kusamaApi, picassoParaAccount);
      const amountToSave = new BN(Pica(transferAmount).toString()).sub(new BN(fees.onKusama.ksm));
      await saveTrappedAssets(picassoApi, devWalletAlice, amountToSave);
      await waitForBlocks(kusamaApi, 1);
      const afterBalance = await fetchNativeBalance(kusamaApi, picassoParaAccount);
      expect(afterBalance).to.be.bignumber.equal(preBalance.add(amountToSave).sub(new BN(fees.onKusama.ksm)));
    });

    it("Users can send tokens to unknown tokens pallet", async function () {
      const [preBalanceOnStatemine] = await fetchXChainTokenBalances(
        [statemineApi],
        [devWalletAlice],
        [unknownAsset]
      );
      await sendUnknownFromStatemine(statemineApi, devWalletAlice, transferAmount, unknownAsset, usdtOnStatemine);
      const [afterBalanceOnStatemine] = await fetchXChainTokenBalances(
        [statemineApi],
        [devWalletAlice],
        [unknownAsset]
      );
      await waitForBlocks(picassoApi, 1);
      expect(preBalanceOnStatemine).to.be.bignumber.equal(
        afterBalanceOnStatemine.add(new BN(to6Digit(transferAmount)))
      );
    });
    it("I can send back if a token is deposited to unknown tokens", async function () {
      const [preBalanceOnStatemine] = await fetchXChainTokenBalances(
        [statemineApi],
        [devWalletAlice],
        [unknownAsset]
      );
      await sendTokenToStatemine(picassoApi, devWalletAlice, devWalletAlice, transferAmount, unknownAsset);
      await waitForBlocks(statemineApi, 2);
      const [afterBalanceOnStatemine] = await fetchXChainTokenBalances(
        [statemineApi],
        [devWalletAlice],
        [unknownAsset]
      );
      expect(afterBalanceOnStatemine).to.be.bignumber.equal(
        preBalanceOnStatemine.add(new BN(to6Digit(transferAmount)))
      );
    });
  });
  describe("XCMP Fail Tests", function () {
    if (!testConfiguration.enabledTests.trapAndUnknownTests.enabled) {
      console.warn("Fail tests are being skipped...");
      return;
    }
    this.timeout(5 * 60 * 1000);
    const falsyWeight = 1_000_000;
    const excessiveAmount = 3790;
    const nonTransferrableAsset = 5;
    it("Users can't send an untransferrable token to Picasso", async function () {
      const [preBalance] = await fetchXChainTokenBalances(
        [picassoApi],
        [devWalletAlice],
        [nonTransferrableAsset]
      );
      await sendTokenToStatemine(picassoApi, devWalletAlice, devWalletAlice, 10, nonTransferrableAsset)
        .catch(error => {
        expect(error).to.be.an("error");
      });
      const [afterBalance] = await fetchXChainTokenBalances(
        [picassoApi],
        [devWalletAlice],
        [nonTransferrableAsset]
      );
      expect(preBalance).to.be.bignumber.equal(afterBalance);
    });
    it("When users set incorrect weights, transfers don't reach to the target chain", async function () {
      const [preBalance] = await fetchXChainTokenBalances([kusamaApi], [devWalletAlice], [0]);
      await sendAssetToRelaychain(picassoApi, devWalletAlice, devWalletAlice, 1, falsyWeight).catch(result=>{
        expect(result).to.be.an("error");
      });
      const [afterBalance] = await fetchXChainTokenBalances([kusamaApi], [devWalletAlice], [0]);
      expect(preBalance).to.be.bignumber.equal(afterBalance);
    });
    it("When users don't have sufficient balance, then the tx will fail", async function () {
      const [preBalanceOnPicasso, preBalanceOnKusama] = await fetchXChainTokenBalances(
        [picassoApi, kusamaApi],
        [devWalletAlice],
        [ksm, 0]
      );
      await sendAssetToRelaychain(picassoApi, devWalletAlice, devWalletAlice, excessiveAmount, destWeight).catch(
        result => {
          expect(result).to.be.an("error");
        }
      );
      const [afterBalanceOnPicasso, afterBalanceOnKusama] = await fetchXChainTokenBalances(
        [picassoApi, kusamaApi],
        [devWalletAlice],
        [ksm, 0]
      );
      expect(afterBalanceOnPicasso).to.be.bignumber.equal(preBalanceOnPicasso);
      expect(afterBalanceOnKusama).to.be.bignumber.equal(preBalanceOnKusama);
    });
  });
});
