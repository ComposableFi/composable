import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";
import {KeyringPair} from "@polkadot/keyring/types";
import {expect} from "chai";
import testConfiguration from './test_configuration.json';


describe("tx.stableSwapDex Tests", function () {
  if (!testConfiguration.enabledTests.enabled)
    return;

  let wallet:KeyringPair,
    sudoKey:KeyringPair;
  let poolIdNum1:number,
    poolIdNum2:number,
    baseAssetId:number,
    quoteAssetId:number;

  before(function() {
    wallet = walletEve.derive("/test/stableSwapDex");
    sudoKey = walletAlice;
    baseAssetId = ASSET_ID_PICA;
    quoteAssetId = ASSET_ID_USDT;
  });

  before('Minting assets', async function() {
    // Setting timeout to 10 minutes
    this.timeout(10 * 60 * 1000);
    const {data: [result],} = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.assets.mintInto(baseAssetId, wallet.address, 555555555555555)
      )
    );
    expect(result.isOk).to.be.true;
    const {data: [result2],} = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.assets.mintInto(quoteAssetId, wallet.address, 555555555555555)
      )
    );
    expect(result2.isOk).to.be.true;
  });

  describe('tx.stableSwapDex.create Success Tests', function () {
    if (!testConfiguration.enabledTests.create__success.enabled)
      return;

    it('Can create stableSwapDex pool [Pool #1] (ampCoe: 1, fee: 0, ownerFee: 0)', async function() {
      if (!testConfiguration.enabledTests.create__success.create1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetId),
        quote: api.createType('u128', quoteAssetId)
      });
      const amplificationCoefficient = api.createType('u16', 1);
      const fee = api.createType('Permill', 0);
      const ownerFee = api.createType('Permill', 0);
      const {data: [resultAccountId, resultPoolId]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.PoolCreated.is,
        api.tx.stableSwapDex.create(pair, amplificationCoefficient, fee, ownerFee)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      poolIdNum1 = resultPoolId.toNumber();
    });

    it('Can create stableSwapDex pool [Pool #2] (ampCoe: 0, fee: 0, ownerFee: 0)', async function() {
      if (!testConfiguration.enabledTests.create__success.create2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetId),
        quote: api.createType('u128', quoteAssetId)
      });
      const amplificationCoefficient = api.createType('u16', 0);
      const fee = api.createType('Permill', 0);
      const ownerFee = api.createType('Permill', 0);
      const {data: [resultAccountId, resultPoolId]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.PoolCreated.is,
        api.tx.stableSwapDex.create(pair, amplificationCoefficient, fee, ownerFee)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      poolIdNum2 = resultPoolId.toNumber();
    });
  });

  describe('Providing Liquidity Success Tests | To be implemented!', ()=>{return;});

  describe('tx.stableSwapDex.buy Success Tests Success Tests', function () {
    if (!testConfiguration.enabledTests.buy__success.enabled)
      return;
    it ('Can buy from stableSwapDex pool [Pool #1] (amount: 100000000)', async function() {
      if (!testConfiguration.enabledTests.buy__success.buy1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum1);
      const assetId = api.createType('u128', quoteAssetId);
      const amount = api.createType('u128', 100000000);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultBaseId, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.buy(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum1);
      expect(resultAssetId.toNumber()).to.be.equal(assetId.toNumber());
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetId);
    });

    it ('Can buy from stableSwapDex pool [Pool #2] (amount: 100000000)', async function() {
      if (!testConfiguration.enabledTests.create__success.create1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum2);
      const assetId = api.createType('u128', quoteAssetId);
      const amount = api.createType('u128', 100000000);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultBaseId, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.buy(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      expect(resultPoolId.toNumber()).to.be.equal(poolIdNum2);
      expect(resultAssetId.toNumber()).to.be.equal(assetId.toNumber());
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetId);
    });
  });

  describe('tx.stableSwapDex.sell Success Tests', function () {
    if (!testConfiguration.enabledTests.sell__success.enabled)
      return;
    it ('Can sell to stableSwapDex pool [Pool #1] (amount: 100)', async function() {
      if (!testConfiguration.enabledTests.sell__success.sell1)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum1);
      const assetId = api.createType('u128', quoteAssetId);
      const amount = api.createType('u128', 100); // > 4294967295 && < 4294967296
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.sell(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      expect(poolIdNum1).to.be.equal(resultPoolId.toNumber());
      expect(baseAssetId).to.be.equal(resultAssetId.toNumber());
    });

    it ('Can sell to stableSwapDex pool [Pool #2] (amount: 100)', async function() {
      if (!testConfiguration.enabledTests.sell__success.sell2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum2);
      const assetId = api.createType('u128', quoteAssetId);
      const amount = api.createType('u128', 100); // > 4294967295 && < 4294967296
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.sell(parameterPoolId, assetId, amount, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      expect(poolIdNum2).to.be.equal(resultPoolId.toNumber());
      expect(baseAssetId).to.be.equal(resultAssetId.toNumber());
    });
  });

  /***
   * stableSwapDex.swap Success Tests
   *
   * Results are:
   * 1. The wallet who sent the transaction.
   * 2. The id of the pool in which the swap happened.
   * 3. The
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
        base: api.createType('u128', baseAssetId),
        quote: api.createType('u128', quoteAssetId)
      });
      const quoteAmount = api.createType('u128', 100);
      const minReceive = api.createType('u128', 0);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultBaseAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.swap(parameterPoolId, pair, quoteAmount, minReceive, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      expect(poolIdNum1).to.be.equal(resultPoolId.toNumber());
      expect(resultBaseAssetId.toNumber()).to.be.equal(baseAssetId);
    });

    it ('Can swap in the stableSwapDex pool [Pool #2] (quoteAmount: 100, minReceive: 0)', async function() {
      if (!testConfiguration.enabledTests.swap__success.swap2)
        this.skip();
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolIdNum2);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetId),
        quote: api.createType('u128', quoteAssetId)
      });
      const quoteAmount = api.createType('u128', 100);
      const minReceive = api.createType('u128', 0);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultBaseAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.swap(parameterPoolId, pair, quoteAmount, minReceive, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      expect(poolIdNum2).to.be.equal(resultPoolId.toNumber());
      expect(resultBaseAssetId.toNumber()).to.be.equal(baseAssetId);
    });
  });
});
