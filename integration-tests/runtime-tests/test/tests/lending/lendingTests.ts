/**
 * Tests for the lending pallet.
 *
 * Before tests, we need to create an oracle, and fake some data.
 * Then we need to create a lending pool to test.
 *
 */
import {KeyringPair} from "@polkadot/keyring/types";
import {txOracleAddAssetAndInfoSuccessTest} from "@composabletests/tests/oracle/testHandlers/addAssetAndInfoTests";
import {expect} from "chai";
import {
  handleLendingAssetMintSetup,
  handleLendingVaultSetup
} from "@composabletests/tests/lending/testHandlers/setupHandler";
import {sendAndWaitForSuccess, waitForBlocks} from "@composable/utils/polkadotjs";
import testConfiguration from './test_configuration.json';
import {createLendingMarketHandler} from "@composabletests/tests/lending/testHandlers/createLendingMarketHandler";
import {
  createLiquidationStrategyHandler
} from "@composabletests/tests/lending/testHandlers/createLiquidationStrategyHandler";
import {txOracleSubmitPriceSuccessTest} from "@composabletests/tests/oracle/testHandlers/submitPriceTests";
import {
  runBeforeTxOracleSetSigner,
  txOracleSetSignerSuccessTest
} from "@composabletests/tests/oracle/testHandlers/setSignerTests";
import {
  runBeforeTxOracleAddStake,
  txOracleAddStakeSuccessTest
} from "@composabletests/tests/oracle/testHandlers/addStakeTests";

describe('Lending Tests', function() {
  if (!testConfiguration.enabled)
    return;
  let oracleId:number, lendingAssetId:number, baseAssetId: number;
  let sudoKey:KeyringPair,
    lenderWallet:KeyringPair,
    oracleControllerWallet:KeyringPair,
    oracleBaseAssetSignerWallet:KeyringPair,
    oracleLendingAssetSignerWallet:KeyringPair,
    vaultManagerWallet:KeyringPair;

  before('Before Lending Tests: Base Setup', async function() {
    if (!testConfiguration.enabledTests.runBeforeBaseSetup)
      return;
    baseAssetId = 1;
    lendingAssetId = 42000;
    sudoKey = walletAlice;
    oracleControllerWallet = walletAlice;
    vaultManagerWallet = walletAlice;
    lenderWallet = walletAlice.derive('/lenderWallet');
    oracleBaseAssetSignerWallet = walletAlice.derive('/oracleSigner/base');
    oracleLendingAssetSignerWallet = walletAlice.derive('/oracleSigner/lending');
  })

  it('Before Lending Tests: Mint lending asset', async function() {
    if (!testConfiguration.enabledTests.runBeforeMintLendingAsset)
      return;
    // Timeout set to 2 minutes.
    this.timeout(2 * 60 * 1000)
    const mintingAmount = 1000000000000
    const result = await handleLendingAssetMintSetup(sudoKey, lendingAssetId, lenderWallet, mintingAmount);
    expect(result[0].isOk).to.be.true;
    expect(result[1].isOk).to.be.true;
  });

  describe('Lending Tests - Oracle Setup', function() {
    /*before ('Before Lending Tests: Create asset vault', async function() {
      if (!testConfiguration.enabledTests.runBeforeCreateAssetVault)
        return;
      // Timeout set to 2 minutes.
      this.timeout(2 * 60 * 1000);
      await waitForBlocks();
      const reserved = api.createType('Perquintill', 1000000000000);
      const strategyMap = new Map();
      strategyMap.set('AccountId32', api.createType('AccountId32', vaultManagerWallet.address));
      strategyMap.set('Perquintill', api.createType('Perquintill', 1000000000000));
      const strategy = api.createType('BTreeMap<AccountId32,Perquintill>', strategyMap);
      const depositAmount = api.createType('u128', 1000000000000);
      const result = await handleLendingVaultSetup(lendingAssetId, reserved, vaultManagerWallet, strategy, depositAmount);
      console.debug(result.toString());
    });*/

    it('Before Lending Tests: Create Oracle for base asset', async function () {
      if (!testConfiguration.enabledTests.runBeforeCreateOracle)
        return;
      // Timeout set to 4 minutes.
      this.timeout(4 * 60 * 1000);
      // Create oracle
      const assetId = api.createType('u128', baseAssetId);
      const threshold = api.createType('Percent', 50);
      const minAnswers = api.createType('u32', 2);
      const maxAnswers = api.createType('u32', 5);
      const blockInterval = api.createType('u32', 6);
      const reward = api.createType('u128', 150000000000);
      const slash = api.createType('u128', 100000000000);
      const {data: [result],} = await txOracleAddAssetAndInfoSuccessTest(
        oracleControllerWallet,
        assetId,
        threshold,
        minAnswers,
        maxAnswers,
        blockInterval,
        reward,
        slash
      );
      if (result.isErr)
        console.debug(result.asErr.toString());
      expect(result.isOk).to.be.true;
      oracleId = (await api.query.oracle.assetsCount()).toNumber();
    });

    it('Before Lending Tests: Create Oracle for lending asset', async function () {
      if (!testConfiguration.enabledTests.runBeforeCreateOracle)
        return;
      // Timeout set to 4 minutes.
      this.timeout(4 * 60 * 1000);
      // Create oracle
      const assetId = api.createType('u128', lendingAssetId);
      const threshold = api.createType('Percent', 50);
      const minAnswers = api.createType('u32', 2);
      const maxAnswers = api.createType('u32', 5);
      const blockInterval = api.createType('u32', 6);
      const reward = api.createType('u128', 150000000000);
      const slash = api.createType('u128', 100000000000);
      const {data: [result],} = await txOracleAddAssetAndInfoSuccessTest(
        oracleControllerWallet,
        assetId,
        threshold,
        minAnswers,
        maxAnswers,
        blockInterval,
        reward,
        slash
      );
      if (result.isErr)
        console.debug(result.asErr.toString());
      expect(result.isOk).to.be.true;
      oracleId = (await api.query.oracle.assetsCount()).toNumber();
    });

    it('Setting oracle signer base asset', async function () {
      if (!testConfiguration.enabledTests.runBeforeSetOracleSigner)
        return;
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const sudoKey = walletAlice;
      const {data: [result],} = await runBeforeTxOracleSetSigner(sudoKey, oracleBaseAssetSignerWallet); // Making sure we have funds.
      expect(result.isOk).to.be.true;
      const {data: [resultAccount0, resultAccount1],} = await txOracleSetSignerSuccessTest(oracleControllerWallet, oracleBaseAssetSignerWallet)
        .catch(function (exc) {
          return {data: [exc]}; /* We can't call this.skip() from here. */
        });
      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
        resultAccount0.message == "oracle.ControllerUsed: This controller is already in use") {
        console.warn("The signer for the lending tests has already been set!\nTrying to ignore this and continuing with lending tests...");
        return;
      }
      expect(resultAccount0).to.not.be.an('Error');
      expect(resultAccount1).to.not.be.an('Error');
      expect(resultAccount0.toString()).to.be.equal(api.createType('AccountId32', oracleBaseAssetSignerWallet.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType('AccountId32', oracleControllerWallet.publicKey).toString());
    });

    it('Setting oracle signer lending asset', async function () {
      if (!testConfiguration.enabledTests.runBeforeSetOracleSigner)
        return;
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const sudoKey = walletAlice;
      const {data: [result],} = await runBeforeTxOracleSetSigner(sudoKey, oracleLendingAssetSignerWallet); // Making sure we have funds.
      expect(result.isOk).to.be.true;
      const {data: [resultAccount0, resultAccount1],} = await txOracleSetSignerSuccessTest(oracleControllerWallet, oracleLendingAssetSignerWallet)
        .catch(function (exc) {
          return {data: [exc]}; /* We can't call this.skip() from here. */
        });
      if (resultAccount0.message == "oracle.SignerUsed: This signer is already in use" ||
        resultAccount0.message == "oracle.ControllerUsed: This controller is already in use") {
        console.warn("The signer for the lending tests has already been set!\nTrying to ignore this and continuing with lending tests...");
        return;
      }
      expect(resultAccount0).to.not.be.an('Error');
      expect(resultAccount1).to.not.be.an('Error');
      expect(resultAccount0.toString()).to.be.equal(api.createType('AccountId32', oracleLendingAssetSignerWallet.publicKey).toString());
      expect(resultAccount1.toString()).to.be.equal(api.createType('AccountId32', oracleControllerWallet.publicKey).toString());
    });

    it('Can add oracle stake to base asset oracle', async function () {
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const sudoKey = walletAlice;
      await runBeforeTxOracleAddStake(sudoKey, oracleControllerWallet, oracleBaseAssetSignerWallet); // Preparing the signer to have funds.
      const stake = api.createType('u128', 250000000000);
      const {data: [result],} = await txOracleAddStakeSuccessTest(oracleControllerWallet, stake);
      expect(result).to.not.be.an('Error');
      expect(result.toString()).to.be
        .equal(api.createType('AccountId32', oracleBaseAssetSignerWallet.publicKey).toString());
    });

    it('Can add oracle stake to lending asset oracle', async function () {
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const sudoKey = walletAlice;
      await runBeforeTxOracleAddStake(sudoKey, oracleControllerWallet, oracleLendingAssetSignerWallet); // Preparing the signer to have funds.
      const stake = api.createType('u128', 250000000000);
      const {data: [result],} = await txOracleAddStakeSuccessTest(oracleControllerWallet, stake);
      expect(result).to.not.be.an('Error');
      expect(result.toString()).to.be
        .equal(api.createType('AccountId32', oracleLendingAssetSignerWallet.publicKey).toString());
    });

    it('Submit new price to oracle for base asset', async function () {
      if (!testConfiguration.enabledTests.runBeforeSubmitPriceOracle)
        return;
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const price = api.createType('u128', 1000000);
      const assetId = api.createType('u128', baseAssetId);
      const {data: [result],} = await txOracleSubmitPriceSuccessTest(oracleBaseAssetSignerWallet, price, assetId);
      expect(result).to.not.be.an('Error');
      expect(result.toString()).to.be
        .equal(api.createType('AccountId32', oracleBaseAssetSignerWallet.publicKey).toString());
    });

    it('Submit new price to oracle for lending asset', async function () {
      if (!testConfiguration.enabledTests.runBeforeSubmitPriceOracle)
        return;
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const price = api.createType('u128', 10000);
      const assetId = api.createType('u128', lendingAssetId);
      const {data: [result],} = await txOracleSubmitPriceSuccessTest(oracleLendingAssetSignerWallet, price, assetId);
      expect(result).to.not.be.an('Error');
      expect(result.toString()).to.be
        .equal(api.createType('AccountId32', oracleLendingAssetSignerWallet.publicKey).toString());
    });

    describe('Liquidation Strategy Success Tests', function () {
      it('Can create liquidation strategy (DutchAuction, LinearDecrease)', async function () {
        if (!testConfiguration.enabledTests.canCreateLiquidationStrategy.createLiquidationStrategyDutchAuctionLinearDecrease)
          this.skip();
        // Setting timeout to 2 minutes.
        this.timeout(2 * 60 * 1000);
        const configuration = api.createType('PalletLiquidationsLiquidationStrategyConfiguration', {
          DutchAuction: api.createType('ComposableTraitsTimeTimeReleaseFunction', {
            LinearDecrease: api.createType('ComposableTraitsTimeLinearDecrease', {
              total: api.createType('u64', 1)
            })
          }),
          UniswapV2: "Null",
          XcmDex: "Null"
        });
        const {data: [result],} = await createLiquidationStrategyHandler(sudoKey, configuration);
        console.debug(result);
      });
    });
  });

  describe('Lending Market Creation Success Tests', function () {
    it('Can create lending market (Jump Interest Rate Model', ()=>{return});

    it('Can create lending market (Curve Interest Rate Model)', async function () {
      if (!testConfiguration.enabledTests.canCreateLendingMarket.createMarketCurveInterestRateModel)
        this.skip();
      // Setting timeout to 2 minutes.
      this.timeout(2 * 60 * 1000);
      const collateralFactor = BigInt(2000000000000000000);
      const underCollaterializedWarnPercent = api.createType('Percent', 10);
      const liquidators = api.createType('Vec<u32>', []);
      const mathCurveModel = api.createType('ComposableTraitsLendingMathCurveModel', {
        baseRate: api.createType('u128', 100000000000)
      });
      const interestRateModel = api.createType('ComposableTraitsLendingMathInterestRateModel', {
        curve: mathCurveModel
      });
      const currencyPair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetId), // Collateral Asset
        quote: api.createType('u128', lendingAssetId) // Borrow Asset
      });
      const reservedFactor = api.createType('Perquintill', 1);
      const {data: [result],} = await createLendingMarketHandler(vaultManagerWallet, collateralFactor, underCollaterializedWarnPercent, liquidators, interestRateModel, currencyPair, reservedFactor)
      console.debug(result.toString());
    });

    it('Can create lending market (DynamicPIDController Interest Rate Model', ()=>{return});

    it('Can create lending market (Double Exponent Interest Rate Model', ()=>{return});
  });

  describe('Borrow success tests', function() {
    it('Lending Tests: Borrow very high amounts => High Interest Rate => High Borrow Rate', ()=>{return true}); // ToDo (D. Roth): Implement.

    it('Lending Tests: Very low borrow amount => Low accrue', ()=>{return true}); // ToDo (D. Roth): Implement.
  });
});
