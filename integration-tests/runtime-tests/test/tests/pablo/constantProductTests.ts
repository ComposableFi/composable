import testConfiguration from './test_configuration.json';
import {expect} from "chai";
import {KeyringPair} from "@polkadot/keyring/types";
import {
  addFundstoThePool,
  buyFromPool,
  createPool,
  getOwnerFee,
  getUserTokens,
  removeLiquidityFromPool,
  rpcPriceFor,
  sellToPool,
  swapTokenPairs
} from './testHandlers/PabloDexHelper';
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

    it('Users can create a constantProduct pool & query RPC', async function() {
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
      //verify if the pool is created
      expect(poolId).to.be.a('number');
      //Verify if the pool is created with specified owner Fee
      expect(returnedOwnerFee).to.be.equal(ownerFee);
      const rpcRes = await rpcPriceFor(api.createType('PalletPabloPoolId', poolId),
          api.createType('CustomRpcCurrencyId', baseAssetId),
          api.createType('CustomRpcCurrencyId', quoteAssetId));
      // expect(rpcRes.spotPrice.).to.be.eq('0');
      expect(rpcRes.poolId.toString()).to.be.eq(api.createType('PalletPabloPoolId', poolId).toString());
      expect(rpcRes.baseAssetId.toString()).to.be.eq(api.createType('CustomRpcCurrencyId', baseAssetId).toString());
      expect(rpcRes.quoteAssetId.toString()).to.be.eq(api.createType('CustomRpcCurrencyId', quoteAssetId).toString());
      expect(rpcRes.spotPrice.toString()).to.be.eq('0');
    })
  });
})
