import testConfiguration from './test_configuration.json';
import {expect} from "chai";
import {KeyringPair} from "@polkadot/keyring/types";
import { addFundstoThePool, buyFromPool, createPool, getOwnerFee, getUserTokens, removeLiquidityFromPool, sellToPool, swapTokenPairs } from './testHandlers/constantProductDexHelper';
import { mintAssetsToWallet } from '@composable/utils/mintingHelper';

/**
   * This suite includes tests for the constantProductDex in Pablo pallet.
   * Tested functionalities are:
   * RPC - Create - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations with constantProductFormula and OwnerFee.
   * Mainly consists of happy path testing.
*/
describe('tx.pablo.constantProductDex Tests', function () {

  let walletId1: KeyringPair,
  walletId2: KeyringPair;
  let poolId: number,
  baseAssetId: number,
  quoteAssetId: number,
  wallet1LpTokens: number,
  baseAmount: number,
  quoteAmount: number,
  ownerFee: number,
  walletId1Account: string,
  walletId2Account: string;
    
  before('Initialize variables', function() {
    walletId1 = walletEve.derive("/test/constantProductDex/walletId1");
    walletId2 = walletBob.derive("/test/constantProductDex/walletId2");
    walletId1Account = api.createType('AccountId32', walletId1.address).toString();
    walletId2Account = api.createType('AccountId32', walletId2.address).toString();
    baseAssetId = 129;
    quoteAssetId = 4;
    baseAmount = 2500;
    quoteAmount = 2500;
    //sets the owner fee to 1.00%/Type Permill
    ownerFee = 10000;      
  });   
  
  before('Minting assets', async function() {
    this.timeout(8*60*1000);
    await mintAssetsToWallet(walletId1, walletAlice, [1, baseAssetId, quoteAssetId]);
    await mintAssetsToWallet(walletId2, walletAlice, [1, baseAssetId, quoteAssetId]);       
  });
  
  describe('tx.pablo.constantProductDex Success Tests', function() {
    if(!testConfiguration.enabledTests.successTests.enabled){
      return;
    }

    it('Users can create a constantProduct pool', async function() {
      if(!testConfiguration.enabledTests.successTests.createPool.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      poolId = await createPool(walletId1, 
        baseAssetId,
        quoteAssetId,
        ownerFee
      );
      const returnedOwnerFee = await getOwnerFee(poolId);
      console.log(poolId);
      //verify if the pool is created
      expect(poolId).to.be.a('number');
      //Verify if the pool is created with specified owner Fee
      expect(returnedOwnerFee).to.be.equal(ownerFee);              
    })     
        
    it('Given that users has sufficient balance, User1 can send funds to pool', async function(){
      if(!testConfiguration.enabledTests.successTests.addLiquidityTests.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      const result = await addFundstoThePool(walletId1,
        baseAmount,
        quoteAmount
      );
      console.log(result);
      //Once funds added to the pool, User is deposited with LP Tokens. 
      wallet1LpTokens = result.returnedLPTokens.toNumber();    
      expect(result.baseAdded.toNumber()).to.be.equal(baseAmount);
      expect(result.quoteAdded.toNumber()).to.be.equal(quoteAmount);
      expect(result.walletIdResult.toString()).to.be.equal(walletId1Account);
    });  

    it('User2 can send funds to pool and router adjusts deposited amounts based on constantProductFormula to prevent arbitrage', async function(){
      if(!testConfiguration.enabledTests.successTests.addLiquidityTests.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      const assetAmount = 30;
      const quoteAmount = 100;
      const result = await addFundstoThePool(walletId2, assetAmount, quoteAmount);    
      //The deposited amount should be maintained by the dex router hence should maintain 1:1. 
      expect(result.quoteAdded.toNumber()).to.be.equal(assetAmount);
      expect(result.walletIdResult.toString()).to.be.equal(walletId2Account);
    });

    it("Given the pool has the sufficient funds, User1 can't completely drain the funds", async function(){
      if(!testConfiguration.enabledTests.successTests.poolDrainTest.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      await buyFromPool(walletId1, baseAssetId, 2800).catch(error=>{
        expect(error.message).to.contain('arithmetic');
      });
    });

    it('User1 can buy from the pool and router respects the constantProductFormula', async function() {
      if(!testConfiguration.enabledTests.successTests.buyTest.enabled){
        this.skip();
      }
      this.timeout(2 * 60 * 1000);
      const result = await buyFromPool(walletId1, baseAssetId, 30); 
      expect(result.accountId.toString()).to.be.equal(walletId1Account);
      //Expected amount is calculated based on the constantProductFormula which is 1:1 for this case. 
      expect(result.quoteAmount.toNumber()).to.be.equal(result.expectedConversion);
    });
    
    it('User1 can sell on the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.sellTest.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      const accountIdSeller = await sellToPool(walletId1, baseAssetId, 20);
      expect(accountIdSeller).to.be.equal(walletId1Account);
    });

    it('User2 can swap from the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.swapTest.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      const quotedAmount = 12;
      const result = await swapTokenPairs(walletId2,
        baseAssetId,
        quoteAssetId,
        quotedAmount,
      );
      expect(result.returnedQuoteAmount.toNumber()).to.be.equal(quotedAmount);
    });

    it('Owner of the pool receives owner fee on the transactions happened in the pool', async function(){
      if(!testConfiguration.enabledTests.successTests.ownerFeeTest.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      const ownerInitialTokens = await getUserTokens(walletId1, quoteAssetId);
      const result = await buyFromPool(walletId2, baseAssetId, 500);      
      const ownerAfterTokens = await getUserTokens(walletId1, quoteAssetId);
      //verifies the ownerFee to be added in the owner account.
      expect(ownerAfterTokens).to.be.equal(ownerInitialTokens+(result.ownerFee.toNumber()))
    });

    it('User1 can remove liquidity from the pool by using LP Tokens', async function(){
      if(!testConfiguration.enabledTests.successTests.removeLiquidityTest.enabled){
        this.skip();
      }
      this.timeout(2*60*1000);
      //Randomly checks an integer value that is always < mintedLPTokens. 
      const result = await removeLiquidityFromPool(walletId1, Math.floor(Math.random()*wallet1LpTokens));
      expect(result.remainingLpTokens.toNumber()).to.be.equal(result.expectedLPTokens);
    });
  });
})
