import {sendAndWaitForSuccess, waitForBlocks} from "@composable/utils/polkadotjs";
import {KeyringPair} from "@polkadot/keyring/types";
import {expect} from "chai";
import testConfiguration from './test_configuration.json';
import {mintAssetsToWallet} from "@composable/utils/mintingHelper";

/**
 * All tests for the stableSwapDex Pallet.
 *
 * The second tests are supposed to test amplificationCoefficient == 0.
 * Those are currently disabled, cause #725 disabled ampCo==0 after some nasty bugs.
 */
describe("tx.stableSwapDex Tests", function () {
  if (!testConfiguration.enabledTests.enabled)
    return;

  let walletPool1:KeyringPair,
    walletPool2:KeyringPair,
    walletPool3:KeyringPair,
    walletLP1:KeyringPair,
    walletLP2:KeyringPair,
    walletTrader1:KeyringPair,
    walletTrader2:KeyringPair,
    sudoKey:KeyringPair;
  let poolIdNum1:number,
    poolIdNum2:number,
    poolIdNum3:number,
    baseAssetIdPool1:number,
    quoteAssetIdPool1:number,
    baseAssetIdPool3:number,
    quoteAssetIdPool3:number;

  before(function() {
    walletPool1 = walletEve.derive("/test/stableSwapDex/walletPool1");
    walletPool2 = walletFerdie.derive("/test/stableSwapDex/walletPool1");
    walletPool3 = walletCharlie.derive("/test/stableSwapDex/walletPool3");
    walletLP1 = walletAlice.derive("/test/stableSwapDex/walletLP1");
    walletLP2 = walletBob.derive("/test/stableSwapDex/walletLP2");
    walletTrader1 = walletBob.derive("/test/stableSwapDex/walletTrader1");
    walletTrader2 = walletDave.derive("/test/stableSwapDex/walletTrader2");
    sudoKey = walletAlice;
    baseAssetIdPool1 = ASSET_ID_BTC;
    quoteAssetIdPool1 = ASSET_ID_USDT;
    baseAssetIdPool3 = 2;
    quoteAssetIdPool3 = 3;
  });

  /***
   * Minting assets to the regarding wallets.
   */
  before('Minting assets', async function() {
    // Setting timeout to 10 minutes
    this.timeout(15 * 60 * 1000);
    await mintAssetsToWallet(sudoKey, sudoKey, [ASSET_ID_PICA]);
    // Pool creator wallets.
    if (testConfiguration.enabledTests.create__success.create1)
      await mintAssetsToWallet(walletPool1, sudoKey, [ASSET_ID_PICA, baseAssetIdPool1, quoteAssetIdPool1]);
    if (testConfiguration.enabledTests.create__success.create2)
      await mintAssetsToWallet(walletPool2, sudoKey, [ASSET_ID_PICA, baseAssetIdPool1, quoteAssetIdPool1]);
    if (testConfiguration.enabledTests.create__success.create3)
      await mintAssetsToWallet(walletPool3, sudoKey, [ASSET_ID_PICA, baseAssetIdPool3, quoteAssetIdPool3]);
    // LP Wallet for #1 Pool.
    if (testConfiguration.enabledTests.add_liquidity__success.add_liquidity1 || testConfiguration.enabledTests.remove_liquidity__success.remove_liquidity1)
      await mintAssetsToWallet(walletLP1, sudoKey, [ASSET_ID_PICA, baseAssetIdPool1, quoteAssetIdPool1]);
    // LP Wallet for #2 Pool.
    if (testConfiguration.enabledTests.add_liquidity__success.add_liquidity2
      || testConfiguration.enabledTests.remove_liquidity__success.remove_liquidity2
      || testConfiguration.enabledTests.remove_liquidity__failure.remove_liquidity1)
      await mintAssetsToWallet(walletLP2, sudoKey, [ASSET_ID_PICA, baseAssetIdPool1, quoteAssetIdPool1]);
    // Wallet for success trading tests
    if (testConfiguration.enabledTests.buy__success.buy1
      || testConfiguration.enabledTests.sell__success.sell1
      || testConfiguration.enabledTests.swap__success.swap1)
      await mintAssetsToWallet(walletTrader1, sudoKey, [ASSET_ID_PICA, baseAssetIdPool1]);
    // Wallet for failure trading tests for pool without liquidity
    if (testConfiguration.enabledTests.buy__failure.buy1
      || testConfiguration.enabledTests.sell__failure.sell1
      || testConfiguration.enabledTests.swap__failure.swap1)
      await mintAssetsToWallet(walletTrader2, sudoKey, [ASSET_ID_PICA, baseAssetIdPool3, quoteAssetIdPool3]);
    console.debug("=>  Minting done!") // ToDo (D. Roth): Remove!
  });

  /***
   * Here we create the pools to make our transactions in.
   *
   * A second test here is required to make sure the amplificationCoefficient works with 0.
   *
   * The results are:
   * 1. The public key of the wallet creating the pool.
   * 2. The pool id of the newly created pool.
   */
  describe('tx.stableSwapDex.create Success Tests', function () {
    if (!testConfiguration.enabledTests.create__success.enabled)
      return;

    it('Can create stableSwapDex pool [Pool #1] (ampCoe: 1, fee: 0, ownerFee: 0)', async function() {
      if (!testConfiguration.enabledTests.create__success.create1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetIdPool1),
        quote: api.createType('u128', quoteAssetIdPool1)
      });
      const amplificationCoefficient = api.createType('u16', 1);
      const fee = api.createType('Permill', 0);
      const ownerFee = api.createType('Permill', 0);
      const {data: [resultAccountId, resultPoolId]} = await sendAndWaitForSuccess(
        api,
        walletPool1,
        api.events.stableSwapDex.PoolCreated.is,
        api.tx.stableSwapDex.create(pair, amplificationCoefficient, fee, ownerFee)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletPool1.address).toString());
      poolIdNum1 = resultPoolId.toNumber();
    });

    it('Can create stableSwapDex pool [Pool #2] (ampCoe: 0, fee: 0, ownerFee: 0)', async function() {
      if (!testConfiguration.enabledTests.create__success.create2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetIdPool1),
        quote: api.createType('u128', quoteAssetIdPool1)
      });
      const amplificationCoefficient = api.createType('u16', 0);
      const fee = api.createType('Permill', 0);
      const ownerFee = api.createType('Permill', 0);
      const {data: [resultAccountId, resultPoolId]} = await sendAndWaitForSuccess(
        api,
        walletPool2,
        api.events.stableSwapDex.PoolCreated.is,
        api.tx.stableSwapDex.create(pair, amplificationCoefficient, fee, ownerFee)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletPool2.address).toString());
      poolIdNum2 = resultPoolId.toNumber();
    });

    /***
     * Idea of this pool is to test buy, sell, swap without adding liquidity first.
     * This should cause buy, sell, swap, and removeLiquidity to fail.
     */
    it('Can create stableSwapDex pool [Pool #3] (ampCoe: 1, fee: 0, ownerFee: 0)', async function() {
      if (!testConfiguration.enabledTests.create__success.create3)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetIdPool3),
        quote: api.createType('u128', quoteAssetIdPool3)
      });
      const amplificationCoefficient = api.createType('u16', 1);
      const fee = api.createType('Permill', 0);
      const ownerFee = api.createType('Permill', 0);
      const {data: [resultAccountId, resultPoolId]} = await sendAndWaitForSuccess(
        api,
        walletPool3,
        api.events.stableSwapDex.PoolCreated.is,
        api.tx.stableSwapDex.create(pair, amplificationCoefficient, fee, ownerFee)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletPool3.address).toString());
      poolIdNum3 = resultPoolId.toNumber();
      console.debug("=>  The pool without liquidity has ID: " + poolIdNum3); // ToDo (D. Roth): Remove debug msg!
    });

    // ToDo (D. Roth): Create Pools with fee & ownerFee!
  });

  /**
   * Here we provide liquidity to the newly created pools.
   *
   * Results are:
   * 1. Public key of the wallet providing liquidity.
   * 2. The pool id, liquidity was provided to.
   * 3. The asset id of the quote asset.
   * 4. The amount of the quote asset.
   * 5. Unknown!
   */
  describe('tx.addLiquidity Success Tests', function () {
    if (!testConfiguration.enabledTests.add_liquidity__success.enabled)
      return;
    it ('Can provide liquidity to stableSwapDex pool [Pool #1] (amount: 100000000)', async function() {
      if (!testConfiguration.enabledTests.add_liquidity__success.add_liquidity1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('PoolId', poolIdNum1);
      const baseAmount = api.createType('Balance', 100000000);
      const quoteAmount = api.createType('Balance', 100000000);
      const minMintAmount = api.createType('Balance', 100000000);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultQuoteAmount, resultNumber4]} = await sendAndWaitForSuccess(
        api,
        walletLP1,
        api.events.stableSwapDex.LiquidityAdded.is,
        api.tx.stableSwapDex.addLiquidity(parameterPoolId, baseAmount, quoteAmount, minMintAmount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletLP1.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum1);
      expect(resultQuoteAmount.toNumber()).to.be.equal(quoteAmount.toNumber());
    });

    it ('Can provide liquidity to stableSwapDex pool [Pool #2] (amount: 100000000)', async function() {
      if (!testConfiguration.enabledTests.add_liquidity__success.add_liquidity2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('PoolId', poolIdNum2);
      const baseAmount = api.createType('Balance', 5555555555555);
      const quoteAmount = api.createType('Balance', 5555555555555);
      const minMintAmount = api.createType('Balance', 1);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultBaseId, resultNumber4]} = await sendAndWaitForSuccess(
        api,
        walletLP2,
        api.events.stableSwapDex.LiquidityAdded.is,
        api.tx.stableSwapDex.addLiquidity(parameterPoolId, baseAmount, quoteAmount, minMintAmount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletPool1.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum1);
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetIdPool1);
    });
  });

  describe('tx.stableSwapDex.buy Success Tests Success Tests', function () {
    if (!testConfiguration.enabledTests.buy__success.enabled)
      return;
    it ('Can buy from stableSwapDex pool [Pool #1] (amount: 10000000)', async function() {
      if (!testConfiguration.enabledTests.buy__success.buy1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum1);
      const assetId = api.createType('u128', quoteAssetIdPool1);
      const amount = api.createType('u128', 10000000);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultBaseId, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.buy(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader1.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum1);
      expect(resultAssetId.toNumber()).to.be.equal(assetId.toNumber());
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetIdPool1);
    });

    it ('Can buy from stableSwapDex pool [Pool #2] (amount: 10000000)', async function() {
      if (!testConfiguration.enabledTests.buy__success.buy2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum2);
      const assetId = api.createType('u128', quoteAssetIdPool1);
      const amount = api.createType('u128', 10000000);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultBaseId, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader2,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.buy(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader2.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum2);
      expect(resultAssetId.toNumber()).to.be.equal(assetId.toNumber());
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetIdPool1);
    });
  });

  describe('tx.stableSwapDex.buy Failure Tests', function () {
    if (!testConfiguration.enabledTests.buy__failure.enabled)
      return;
    it ('Should not be able to buy from pool without liquidity [Pool #3] (amount: 10000000)', async function() {
      if (!testConfiguration.enabledTests.buy__failure.buy1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum3);
      const assetId = api.createType('u128', quoteAssetIdPool3);
      const amount = api.createType('u128', 10000000);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultBaseId, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader2,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.buy(parameterPoolId, assetId, amount, keepAlive)
      );
      // ToDo (D. Roth): Change to check for error!
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader2.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum3);
      expect(resultAssetId.toNumber()).to.be.equal(assetId.toNumber());
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetIdPool3);
    });
  });

  /***
   * Tests to check if we can successfully sell assets to the pool.
   *
   * Results are:
   * 1. Public key of the transacting wallet.
   * 2. The pool id the token were sold to.
   * 3. The asset ID of the base asset.
   * 4. - 7. No clue.
   */
  describe('tx.stableSwapDex.sell Success Tests', function () {
    if (!testConfiguration.enabledTests.sell__success.enabled)
      return;
    it ('Can sell to stableSwapDex pool [Pool #1] (amount: 100)', async function() {
      if (!testConfiguration.enabledTests.sell__success.sell1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum1);
      const assetId = api.createType('u128', quoteAssetIdPool1);
      const amount = api.createType('u128', 100);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.sell(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader1.address).toString());
      expect(poolIdNum1).to.be.equal(resultPoolId.toNumber());
      expect(baseAssetIdPool1).to.be.equal(resultAssetId.toNumber());
    });

    it ('Can sell to stableSwapDex pool [Pool #2] (amount: 100)', async function() {
      if (!testConfiguration.enabledTests.sell__success.sell2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum2);
      const assetId = api.createType('u128', quoteAssetIdPool1);
      const amount = api.createType('u128', 100); // > 4294967295 && < 4294967296
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader2,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.sell(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader2.address).toString());
      expect(poolIdNum2).to.be.equal(resultPoolId.toNumber());
      expect(baseAssetIdPool1).to.be.equal(resultAssetId.toNumber());
    });
  });

  describe('tx.stableSwapDex.sell Failure Tests', function () {
    if (!testConfiguration.enabledTests.sell__failure.enabled)
      return;
    it('Should not be able to sell to pool without liquidity [Pool #3] (amount: 100)', async function () {
      if (!testConfiguration.enabledTests.sell__failure.sell1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum3);
      const assetId = api.createType('u128', quoteAssetIdPool3);
      const amount = api.createType('u128', 100);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader2,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.sell(parameterPoolId, assetId, amount, keepAlive)
      );
      // ToDo (D. Roth): This should check for an error!
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader2.address).toString());
      expect(poolIdNum3).to.be.equal(resultPoolId.toNumber());
      expect(baseAssetIdPool3).to.be.equal(resultAssetId.toNumber());
    });
  });

  /***
   * stableSwapDex.swap Success Tests
   *
   * Results are:
   * 1. The wallet who sent the transaction.
   * 2. The id of the pool in which the swap happened.
   * 3. The asset ID of the base asset.
   * 4. - 7. Unknown! Couldn't identify them.
   */
  describe('tx.stableSwapDex.swap Success Tests', function () {
    if (!testConfiguration.enabledTests.swap__success.enabled)
      return;
    it ('Can swap in the stableSwapDex pool [Pool #1] (quoteAmount: 100, minReceive: 0)', async function() {
      if (!testConfiguration.enabledTests.swap__success.swap1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum1);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetIdPool1),
        quote: api.createType('u128', quoteAssetIdPool1)
      });
      const quoteAmount = api.createType('u128', 100);
      const minReceive = api.createType('u128', 0);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultBaseAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader1,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.swap(parameterPoolId, pair, quoteAmount, minReceive, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader1.address).toString());
      expect(poolIdNum1).to.be.equal(resultPoolId.toNumber());
      expect(resultBaseAssetId.toNumber()).to.be.equal(baseAssetIdPool1);
    });

    it ('Can swap in the stableSwapDex pool [Pool #2] (quoteAmount: 100, minReceive: 0)', async function() {
      if (!testConfiguration.enabledTests.swap__success.swap2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum2);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetIdPool1),
        quote: api.createType('u128', quoteAssetIdPool1)
      });
      const quoteAmount = api.createType('u128', 100);
      const minReceive = api.createType('u128', 0);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultBaseAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader2,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.swap(parameterPoolId, pair, quoteAmount, minReceive, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader2.address).toString());
      expect(poolIdNum2).to.be.equal(resultPoolId.toNumber());
      expect(resultBaseAssetId.toNumber()).to.be.equal(baseAssetIdPool1);
    });
  });

  describe('tx.stableSwapDex.swap Failure Tests', function () {
    if (!testConfiguration.enabledTests.swap__failure.enabled)
      return;
    it('Should not be able to swap in pool without liquidity [Pool #3] (quoteAmount: 100, minReceive: 0)', async function () {
      if (!testConfiguration.enabledTests.swap__failure.swap1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum3);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetIdPool3),
        quote: api.createType('u128', quoteAssetIdPool3)
      });
      const quoteAmount = api.createType('u128', 100);
      const minReceive = api.createType('u128', 0);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultBaseAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        walletTrader2,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.swap(parameterPoolId, pair, quoteAmount, minReceive, keepAlive)
      );
      // ToDo (D. Roth): This should check for an error!
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletTrader2.address).toString());
      expect(poolIdNum3).to.be.equal(resultPoolId.toNumber());
      expect(resultBaseAssetId.toNumber()).to.be.equal(baseAssetIdPool3);
    });
  });

  /***
   * stableSwapDex.removeLiquidity Success Tests
   *
   * The results are:
   * 1. The transacting wallet.
   * 2. The pool id from which liquidity was removed.
   * 3. TBD
   * 4. The amount of quote asset that were removed from the pool.
   * 5. TBD
   */
  describe('Remove Liquidity Success Tests', function() {
    if (!testConfiguration.enabledTests.remove_liquidity__success.enabled)
      return;
    it ('Can remove liquidity from stableSwapDex pool [Pool #1] (baseAmount: 100, quoteAmount: 10, minMintAmount: 10)', async function() {
      if (!testConfiguration.enabledTests.remove_liquidity__success.remove_liquidity1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('PoolId', poolIdNum1);
      const baseAmount = api.createType('Balance', 100);
      const quoteAmount = api.createType('Balance', 10);
      const minMintAmount = api.createType('Balance', 10);
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultQuoteAmount, resultNumber4]} = await sendAndWaitForSuccess(
        api,
        walletLP1,
        api.events.stableSwapDex.LiquidityRemoved.is,
        api.tx.stableSwapDex.removeLiquidity(parameterPoolId, baseAmount, quoteAmount, minMintAmount)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletLP1.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum1);
      expect(resultQuoteAmount.toNumber()).to.be.at.least(quoteAmount.toNumber());
    });

    it ('Can remove liquidity from stableSwapDex pool [Pool #2] (baseAmount: 100, quoteAmount: 10, minMintAmount: 10)', async function() {
      if (!testConfiguration.enabledTests.remove_liquidity__success.remove_liquidity2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('PoolId', poolIdNum2);
      const baseAmount = api.createType('Balance', 100);
      const quoteAmount = api.createType('Balance', 10);
      const minMintAmount = api.createType('Balance', 10);
      const {data: [resultAccountId, resultPoolId, resultQuoteAssetId, resultBaseId, resultNumber4]} = await sendAndWaitForSuccess(
        api,
        walletLP2,
        api.events.stableSwapDex.LiquidityRemoved.is,
        api.tx.stableSwapDex.removeLiquidity(parameterPoolId, baseAmount, quoteAmount, minMintAmount)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', walletLP2.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum1);
      expect(resultQuoteAssetId.toNumber()).to.be.equal(quoteAssetIdPool1);
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetIdPool1);
    });
  });

  describe('Remove Liquidity Failure Tests', function() {
    if (!testConfiguration.enabledTests.remove_liquidity__failure.enabled)
      return;
    it('Should not be able to remove liquidity from pool without any [Pool #3] (baseAmount: 100, quoteAmount: 10, minMintAmount: 10)', async function () {
      if (!testConfiguration.enabledTests.remove_liquidity__failure.remove_liquidity1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('PoolId', poolIdNum3);
      const baseAmount = api.createType('Balance', 100);
      const quoteAmount = api.createType('Balance', 10);
      const minMintAmount = api.createType('Balance', 10);
      const {data} = await sendAndWaitForSuccess(
        api,
        walletLP2,
        api.events.stableSwapDex.LiquidityRemoved.is,
        api.tx.stableSwapDex.removeLiquidity(parameterPoolId, baseAmount, quoteAmount, minMintAmount)
      );
      console.debug(typeof(data));
    });
  });
});
