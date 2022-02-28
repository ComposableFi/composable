import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";
import {KeyringPair} from "@polkadot/keyring/types";
import {expect} from "chai";


describe("tx.stableSwapDex Tests", function () {
  let wallet:KeyringPair,
    sudoKey:KeyringPair;
  let poolId:number,
    baseAssetId:number,
    quoteAssetId:number;

  before(function() {
    wallet = walletEve;
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

  describe('tx.stableSwapDex.create Tests', function () {
    it('tx.stableSwapDex.create Success Test', async function() {
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
      poolId = resultPoolId.toNumber();
    });
  });

  describe('tx.stableSwapDex.buy Success Tests', function () {
    it ('tx.stableSwapDex.buy Success Test', async function() {
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolId);
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
      expect(resultPoolId.toNumber()).to.be.equal(poolId);
      expect(resultAssetId.toNumber()).to.be.equal(assetId.toNumber());
      expect(resultBaseId.toNumber()).to.be.equal(baseAssetId);
    });
  });

  describe('tx.stableSwapDex.sell Tests', function () {
    it ('tx.stableSwapDex.sell Success Test', async function() {
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolId);
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
      expect(poolId).to.be.equal(resultPoolId.toNumber());
      expect(baseAssetId).to.be.equal(resultAssetId.toNumber());
    });
  });

  describe('tx.stableSwapDex.swap Tests', function () {
    it ('tx.stableSwapDex.swap Success Test', async function() {
      // Setting timeout to 2 minutes
      this.timeout(2 * 60 * 1000);
      const parameterPoolId = api.createType('u128', poolId);
      const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
        base: api.createType('u128', baseAssetId),
        quote: api.createType('u128', quoteAssetId)
      });
      const quoteAmount = api.createType('u128', 100);
      const minReceive = api.createType('u128', 0);
      const keepAlive = true;
      const {data: [resultAccountId, resultPoolId, resultAssetId, resultNumber3, resultNumber4, resultNumber5, resultNumber6]} = await sendAndWaitForSuccess(
        api,
        wallet,
        api.events.stableSwapDex.Swapped.is,
        api.tx.stableSwapDex.swap(parameterPoolId, pair, quoteAmount, minReceive, keepAlive)
      );
      expect(resultAccountId.toString()).to.be.equal(api.createType('AccountId32', wallet.address).toString());
      expect(poolId).to.be.equal(resultPoolId.toNumber());
    });
  });
});
