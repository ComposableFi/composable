/**
 * Tests for the lending pallet.
 *
 * Before tests, we need to create an oracle, and fake some data.
 * Then we need to create a lending pool to test.
 *
 */
import { KeyringPair } from "@polkadot/keyring/types";
import { txOracleAddAssetAndInfoSuccessTest } from "@composabletests/tests/oracle/testHandlers/addAssetAndInfoTests";
import { expect } from "chai";
import testConfiguration from "./test_configuration.json";
import { txOracleSubmitPriceSuccessTest } from "@composabletests/tests/oracle/testHandlers/submitPriceTests";
import {
  runBeforeTxOracleSetSigner,
  txOracleSetSignerSuccessTest
} from "@composabletests/tests/oracle/testHandlers/setSignerTests";
import {
  runBeforeTxOracleAddStake,
  txOracleAddStakeSuccessTest
} from "@composabletests/tests/oracle/testHandlers/addStakeTests";
import { mintAssetsToWallet } from "@composable/utils/mintingHelper";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import { waitForBlocks } from "@composable/utils/polkadotjs";
import { handleLendingVaultSetup } from "@composabletests/tests/angular/testHandlers/vaultSetupHandler";
import {
  createLiquidationStrategyHandler
} from "@composabletests/tests/angular/testHandlers/createLiquidationStrategyHandler";
import { createLendingMarketHandler } from "@composabletests/tests/angular/testHandlers/createLendingMarketHandler";
import { depositCollateralHandler } from "@composabletests/tests/angular/testHandlers/depositCollateralHandler";
import { borrowHandler } from "@composabletests/tests/angular/testHandlers/borrowHandler";
import { withdrawCollateralHandler } from "@composabletests/tests/angular/testHandlers/withdrawCollateralHandler";

describe("Lending Tests", function() {
  if (!testConfiguration.enabled)
    return;
  let oracleId: number,
    lendingMarketIdCurveInterestRate: number;

  let api: ApiPromise;

  let sudoKey: KeyringPair,
    lenderWallet: KeyringPair,
    borrowerWallet: KeyringPair,
    oracleControllerWallet: KeyringPair,
    oracleSignerWallet: KeyringPair,
    vaultManagerWallet: KeyringPair;

  before("Before Lending Tests: Base Setup", async function() {
    if (!testConfiguration.enabledTests.runBeforeBaseSetup)
      return;
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletCharlie, devWalletFerdie } = getDevWallets(newKeyring);
    lendingMarketIdCurveInterestRate = 1;
    sudoKey = devWalletAlice;
    oracleControllerWallet = devWalletAlice;
    vaultManagerWallet = devWalletAlice;
    lenderWallet = devWalletCharlie.derive("/test/lending/lenderWallet");
    borrowerWallet = devWalletFerdie.derive("/test/lending/borrowerWallet");
    oracleSignerWallet = devWalletAlice.derive("/test/lending/oracleSigner");
  });

  before("Before Lending Tests: Mint lending asset", async function() {
    if (!testConfiguration.enabledTests.runBeforeMintLendingAsset)
      return;
    // Timeout set to 2 minutes.
    this.timeout(15 * 60 * 1000);
    await mintAssetsToWallet(api, lenderWallet, sudoKey, [1, 2, 3]);
    await mintAssetsToWallet(api, borrowerWallet, sudoKey, [1, 3]);
    await mintAssetsToWallet(api, oracleSignerWallet, sudoKey, [1, 3]);
    //await handleAssetMintSetup(sudoKey, [ASSET_ID_BTC, ASSET_ID_USDT, ASSET_ID_PICA], walletAlice, mintingAmount);
  });

  after("Closing the connection", async function() {
    await api.disconnect();
  });

  describe("Lending Tests - Oracle Setup", function() {
    before("Before Lending Tests: Create asset vault", async function() {
      if (!testConfiguration.enabledTests.runBeforeCreateAssetVault)
        return;
      // Timeout set to 2 minutes.
      this.timeout(2 * 60 * 1000);
      await waitForBlocks(api);
      const reserved = api.createType("Perquintill", 1000000000000);
      const strategyMap = new Map();
      strategyMap.set("AccountId32", api.createType("AccountId32", vaultManagerWallet.address));
      strategyMap.set("Perquintill", api.createType("Perquintill", 1000000000000));
      const strategy = api.createType("BTreeMap<AccountId32,Perquintill>", strategyMap);
      const depositAmount = api.createType("u128", 1000000000000);

      // Transaction
      const result = await handleLendingVaultSetup(
        api,
        1,
        reserved,
        vaultManagerWallet,
        strategy,
        depositAmount
      );

      // ToDo (D. Roth): Verification!
      console.debug(result.toString());
    });

    describe("Before Lending Tests: Create Oracles", function() {
      it("Before Lending Tests: Create Oracle for PICA", async function() {
        if (!testConfiguration.enabledTests.runBeforeCreateOracle)
          return;
        // Timeout set to 4 minutes.
        this.timeout(4 * 60 * 1000);
        // Create oracle
        const assetId = api.createType("u128", 1);
        const threshold = api.createType("Percent", 50);
        const minAnswers = api.createType("u32", 1);
        const maxAnswers = api.createType("u32", 5);
        const blockInterval = api.createType("u32", 6);
        const reward = api.createType("u128", 150000000000);
        const slash = api.createType("u128", 100000000000);

        // Transaction
        const { data: [result] } = await txOracleAddAssetAndInfoSuccessTest(
          api,
          oracleControllerWallet,
          assetId,
          threshold,
          minAnswers,
          maxAnswers,
          blockInterval,
          reward,
          slash
        );

        // ToDo (D. Roth): Verification!
        if (result.isErr)
          console.debug(result.asErr.toString());
        expect(result.isOk).to.be.true;
        oracleId = (await api.query.oracle.assetsCount()).toNumber();
      });

      it("Before Lending Tests: Create Oracle for base asset", async function() {
        if (!testConfiguration.enabledTests.runBeforeCreateOracle)
          return;
        // Timeout set to 4 minutes.
        this.timeout(4 * 60 * 1000);
        const assetId = api.createType("u128", 1000);
        const threshold = api.createType("Percent", 50);
        const minAnswers = api.createType("u32", 1);
        const maxAnswers = api.createType("u32", 5);
        const blockInterval = api.createType("u32", 6);
        const reward = api.createType("u128", 150000000000);
        const slash = api.createType("u128", 100000000000);

        // Transaction
        const { data: [result] } = await txOracleAddAssetAndInfoSuccessTest(
          api,
          oracleControllerWallet,
          assetId,
          threshold,
          minAnswers,
          maxAnswers,
          blockInterval,
          reward,
          slash
        );

        // ToDo (D. Roth): Verification!
        if (result.isErr)
          console.debug(result.asErr.toString());
        expect(result.isOk).to.be.true;
        oracleId = (await api.query.oracle.assetsCount()).toNumber();
      });

      it("Before Lending Tests: Create Oracle for lending asset", async function() {
        if (!testConfiguration.enabledTests.runBeforeCreateOracle)
          return;
        // Timeout set to 4 minutes.
        this.timeout(4 * 60 * 1000);
        const assetId = api.createType("u128", 2000);
        const threshold = api.createType("Percent", 50);
        const minAnswers = api.createType("u32", 1);
        const maxAnswers = api.createType("u32", 5);
        const blockInterval = api.createType("u32", 6);
        const reward = api.createType("u128", 150000000000);
        const slash = api.createType("u128", 100000000000);

        // Transaction
        const { data: [result] } = await txOracleAddAssetAndInfoSuccessTest(
          api,
          oracleControllerWallet,
          assetId,
          threshold,
          minAnswers,
          maxAnswers,
          blockInterval,
          reward,
          slash
        );

        // ToDo (D. Roth): Verification!
        if (result.isErr)
          console.debug(result.asErr.toString());
        expect(result.isOk).to.be.true;
        oracleId = (await api.query.oracle.assetsCount()).toNumber();
      });
    });

    describe("Before Lending Tests: Set Oracle Signer and add stake", function() {
      it("Setting oracle signer", async function() {
        if (!testConfiguration.enabledTests.runBeforeSetOracleSigner)
          return;
        // Setting timeout to 2 minutes.
        this.timeout(2 * 60 * 1000);

        await runBeforeTxOracleSetSigner(api, sudoKey, oracleSignerWallet); // Making sure we have funds.

        // Transaction
        const { data: [resultAccount0, resultAccount1] } = await txOracleSetSignerSuccessTest(
          api,
          oracleControllerWallet,
          oracleSignerWallet
        ).catch(function(exc) {
          return { data: [exc] }; /* We can't call this.skip() from here. */
        });

        // ToDo (D. Roth): Verification!
        if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
          resultAccount0.message == "oracle.ControllerUsed: This controller is already in use") {
          console.warn("        The signer for the lending tests has already been set!\n        " +
            "Trying to ignore this and continuing with lending tests...");
          return;
        }
        expect(resultAccount0).to.not.be.an("Error");
        expect(resultAccount1).to.not.be.an("Error");
        expect(resultAccount0.toString()).to.be.equal(api.createType("AccountId32", oracleSignerWallet.publicKey).toString());
        expect(resultAccount1.toString()).to.be.equal(api.createType("AccountId32", oracleControllerWallet.publicKey).toString());
      });

      it("Can add oracle stake", async function() {
        // Setting timeout to 2 minutes.
        this.timeout(2 * 60 * 1000);

        // Preparing the signer to have funds.
        await runBeforeTxOracleAddStake(api, sudoKey, oracleControllerWallet, oracleSignerWallet);
        const stake = api.createType("u128", 250000000000);

        // Transaction
        const { data: [result] } = await txOracleAddStakeSuccessTest(api, oracleControllerWallet, stake);

        // ToDo (D. Roth): Verification!
        expect(result).to.not.be.an("Error");
        expect(result.toString()).to.be
          .equal(api.createType("AccountId32", oracleSignerWallet.publicKey).toString());
      });
    });
  });

  describe("Liquidation Strategy Success Tests", function() {
    it("Can create liquidation strategy (DutchAuction, LinearDecrease)", async function() {
      if (!testConfiguration.enabledTests.canCreateLiquidationStrategy.createLiquidationStrategyDutchAuctionLinearDecrease)
        this.skip();
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const configuration = api.createType("PalletLiquidationsLiquidationStrategyConfiguration", {
        DutchAuction: api.createType("ComposableTraitsTimeTimeReleaseFunction", {
          LinearDecrease: api.createType("ComposableTraitsTimeLinearDecrease", {
            total: api.createType("u64", 1)
          })
        }),
        UniswapV2: "Null",
        XcmDex: "Null"
      });

      // Transaction
      const { data: [result] } = await createLiquidationStrategyHandler(api, sudoKey, configuration);

      // ToDo (D. Roth): Add Check!
    });
  });

  describe("Before Lending Tests: Submit Oracle Prices", function() {
    it("Submit new price to oracle for base asset", async function() {
      if (!testConfiguration.enabledTests.runBeforeSubmitPriceOracle)
        this.skip();
      // Setting timeout to 2 minutes.
      this.timeout(5 * 60 * 1000);
      await waitForBlocks(api, 10);
      const price = api.createType("u128", 10000);
      const assetId = api.createType("u128", 1000);

      // Transaction
      const { data: [result] } = await txOracleSubmitPriceSuccessTest(api, oracleSignerWallet, price, assetId);

      // ToDo (D. Roth): Verification!
      expect(result).to.not.be.an("Error");
      expect(result.toString()).to.be
        .equal(api.createType("AccountId32", oracleSignerWallet.publicKey).toString());
      await waitForBlocks(api);
    });

    it("Submit new price to oracle for lending asset", async function() {
      if (!testConfiguration.enabledTests.runBeforeSubmitPriceOracle)
        this.skip();
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const price = api.createType("u128", 10000);
      const assetId = api.createType("u128", 2000);

      // Transaction
      const { data: [result] } = await txOracleSubmitPriceSuccessTest(api, oracleSignerWallet, price, assetId);

      // ToDo (D. Roth): Verification!
      expect(result).to.not.be.an("Error");
      expect(result.toString()).to.be
        .equal(api.createType("AccountId32", oracleSignerWallet.publicKey).toString());
    });
  });

  describe("Lending Market Creation Success Tests", function() {
    it("Can create lending market (Curve Interest Rate Model)", async function() {
      if (!testConfiguration.enabledTests.canCreateLendingMarket.createMarketCurveInterestRateModel)
        this.skip();
      this.retries(1);
      // Setting timeout to 2 minutes.
      this.timeout(8 * 60 * 1000);

      // Transaction | ToDo (D. Roth): Cleanup!
      const result = await createLendingMarketHandler(
        api,
        oracleSignerWallet, // Wallet
        BigInt(2000000000000000000), // collerateralFactor
        api.createType("Percent", 10), // underCollaterializedWarnPercent
        api.createType("Vec<u32>", []), // liquidators
        api.createType("ComposableTraitsLendingMathInterestRateModel", { // Interest Rate Model
          curve: api.createType("ComposableTraitsLendingMathCurveModel", { // Curve Model
            baseRate: api.createType("u128", 1)
          })
        }),
        api.createType("ComposableTraitsDefiCurrencyPair", { // Currency Pair
          base: api.createType("u128", 1000), // Borrow Asset
          quote: api.createType("u128", 2000) // Collateral Asset
        }),
        api.createType("Perquintill", 1) // reservedFactor
      );

      // ToDo (D. Roth): Verification!
      console.debug(result.toString());
      await waitForBlocks(api, 3);
    });
  });

  describe("Lending Deposit Collateral Tests", async function() {
    it("Can deposit collateral to curve market", async function() {
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const marketId = api.createType("u32", lendingMarketIdCurveInterestRate);
      const amount = api.createType("u128", 25000000000000);

      // Transaction
      const { data: [resultWallet, resultMarketId, resultAmount] } = await depositCollateralHandler(
        api,
        lenderWallet,
        marketId,
        amount
      );

      // ToDo (D. Roth): Verification!
      expect(resultWallet.toString()).to.be.equal(api.createType("AccountId32", lenderWallet.publicKey).toString());
      expect(resultMarketId.toNumber()).to.be.equal(marketId.toNumber());
      expect(resultAmount.toNumber()).to.be.equal(amount.toNumber());
    });
  });

  describe("Borrow success tests", function() {
    it("Lending Tests: Borrow very high amounts => High Interest Rate => High Borrow Rate", async function() {
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const marketId = api.createType("u32", lendingMarketIdCurveInterestRate);
      const amount = api.createType("u128", 12515000000000); // <1251 && >125

      // Transaction
      const result = await borrowHandler(
        api,
        lenderWallet,
        marketId,
        amount
      );

      // ToDo (D. Roth): Verification!
      console.debug(result);
    });
  });

  describe("Lending Withdraw Collateral Tests", async function() {
    it("Can withdraw collateral from curve market", async function() {
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const marketId = api.createType("u32", lendingMarketIdCurveInterestRate);
      const amount = api.createType("u128", 25000000000000);

      // Transaction
      const { data: [resultWallet, resultMarketId, resultAmount] } = await withdrawCollateralHandler(
        api,
        lenderWallet,
        marketId,
        amount
      );

      // ToDo (D. Roth): Verification!
      expect(resultWallet.toString()).to.be.equal(api.createType("AccountId32", lenderWallet.publicKey).toString());
      expect(resultMarketId.toNumber()).to.be.equal(marketId.toNumber());
      expect(resultAmount.toNumber()).to.be.equal(amount.toNumber());
    });
  });

  describe("Lending Repay Borrow Tests", function() {
    // ToDo!
    return;
  });
});
