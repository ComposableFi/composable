import pabloTestConfiguration from "../testHandlers/test_configuration.json";
import testConfiguration from "./test_configuration.json";
import {expect} from "chai";
import {KeyringPair} from "@polkadot/keyring/types";
import {mintAssetsToWallet, Pica} from "@composable/utils/mintingHelper";
import BN from "bn.js";
import {getNewConnection} from "@composable/utils/connectionHelper";
import {getDevWallets} from "@composable/utils/walletHelper";
import {ApiPromise} from "@polkadot/api";
import {
  addFundstoThePool, buyFromPool,
  createConsProdPool,
  createMultipleCPPools, getPoolAddress, getPoolBalance,
  getPoolInfo, getUserTokens, removeLiquidityFromPool, sellToPool, swapTokenPairs, transferTokens
} from "@composabletests/tests/pablo/testHandlers/pabloTestHelper";

/**
 * This suite includes tests for the constantProductDex Pallet.
 * Tested functionalities are:
 * Create - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations with constantProductFormula
 *    and OwnerFee.
 * Mainly consists of happy path testing.
 */
describe("tx.constantProductDex Tests", function () {
  if(!pabloTestConfiguration.constantProductTests.enabled){
    console.log("Constant Product Tests are being skipped...");
    return;
  }
  this.timeout(2 * 60 * 1000);
  let walletId1: KeyringPair, walletId2: KeyringPair, walletId3: KeyringPair, sudoKey: KeyringPair;
  let poolId: number, poolId2: number,
    baseAssetId: number, baseAsset2: number,
    quoteAssetId: number, quoteAsset2: number,
    walletLpTokens: bigint,
    baseAmount: bigint,
    quoteAmount: bigint,
    fee: number,
    ownerFee: number,
    transferredTokens: BN,
    walletId1Account: string,
    walletId2Account: string,
    poolAddress: string,
    api: ApiPromise;

  before("Initialize variables", async function () {
    const {newClient, newKeyring} = await getNewConnection();
    api = newClient;
    const {devWalletAlice, devWalletEve, devWalletFerdie} = getDevWallets(newKeyring);
    sudoKey = devWalletAlice;
    walletId1 = devWalletEve.derive("/test/constantProductDex/walletId1");
    walletId2 = devWalletFerdie.derive("/test/constantProductDex/walletId2");
    walletId3 = devWalletAlice.derive("/test/constantProductDex/walletId3");
    walletId1Account = api.createType("AccountId32", walletId1.address).toString();
    walletId2Account = api.createType("AccountId32", walletId2.address).toString();
    baseAssetId = 2;
    quoteAssetId = 3;
    baseAsset2 = 7;
    quoteAsset2 = 23;
    baseAmount = Pica(250000);
    quoteAmount = Pica(250000);
    fee = 10000;
    //sets the owner fee to 1.00%/Type Permill
    ownerFee = 50000;
  });

  before("Minting assets", async function () {
    await mintAssetsToWallet(api, walletId1, sudoKey, [1, baseAssetId, quoteAssetId, baseAsset2, quoteAsset2]);
    await mintAssetsToWallet(api, walletId2, sudoKey, [1, baseAssetId, quoteAssetId, baseAsset2, quoteAsset2]);
    await mintAssetsToWallet(api, walletId3, sudoKey, [1]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("tx.constantProductDex Create Pool Tests", function () {
    if (!testConfiguration.enabledTests.createPoolTests.enabled) {
      console.log("ConstantProduct create pools tests are being skipped...");
      return;
    }
    this.timeout(2 * 60 * 1000);

    it("Given that users are on the chain, users can create a ConstantProduct pool", async function () {
      this.timeout(2 * 60 * 1000);
      poolId = await createConsProdPool(api, walletId1, walletId1, baseAssetId, quoteAssetId, fee, ownerFee);
      const {ownFee} = await getPoolInfo(api, "ConstantProduct", poolId);
      //verify if the pool is created
      expect(poolId).to.be.a("number");
      //Verify if the pool is created with specified owner Fee
      expect(ownFee).to.be.equal(ownerFee);
    });

    it("Given that users are on the chain, users can create another ConstantProduct pool with different assetIds",
      async function () {
        this.timeout(2 * 60 * 1000);
        poolId2 = await createConsProdPool(api, walletId2, walletId2, baseAssetId, quoteAsset2, fee, ownerFee);
        const {ownFee} = await getPoolInfo(api, "ConstantProduct", poolId2);
        //verify if the pool is created
        expect(poolId2).to.be.a("number");
        //Verify if the pool is created with specified owner Fee
        expect(ownFee).to.be.equal(ownerFee);
    });

    it("Given that users have no active balance on assets, users can create ConstantProduct Pool", async function () {
      this.timeout(2 * 60 * 1000);
      const result = await createConsProdPool(api, walletId2, walletId2, 50, 60, fee, ownerFee);
      expect(result).to.be.a("number");
    });

    it("Given that the chain is up, users can create pools-" +
      " test creates up to 500 Constant Product pools with valid random fees, random owner fees and random assetIds",
      async function () {
        this.timeout(2 * 60 * 1000);
        await createMultipleCPPools(api, walletId1);
        expect((await api.query.pablo.poolCount()).toNumber()).to.be.greaterThan(500);
    });
  });

  describe("ConstantProductDex Add Liquidity Tests", async function () {
    if (!testConfiguration.enabledTests.addLiquidityTests.enabled) {
      console.log("ConstantProductDex add liquidity tests are being skipped...");
      return;
    }
    this.timeout(2 * 60 * 1000);

    it("Given that users has sufficient balance, User1 can send funds to pool", async function () {
      poolAddress = await getPoolAddress(api, poolId, walletId1, Pica(1), Pica(1));
      const result = await addFundstoThePool(api, poolId, walletId1, baseAmount, quoteAmount);
      //Once funds added to the pool, User is deposited with LP Tokens.
      walletLpTokens = BigInt(result.returnedLPTokens.toString());
      expect(BigInt(result.baseAdded.toString(10))).to.be.equal(baseAmount);
      expect(BigInt(result.quoteAdded.toString(10))).to.be.equal(quoteAmount);
      expect(result.walletIdResult.toString()).to.be.equal(walletId1Account);
    });

    it("Given that users have LPTokens, users can transfer LP Tokens to another user", async function () {
      const {lpTokenId} = await getPoolInfo(api, "ConstantProduct", poolId);
      await transferTokens(api, walletId1, walletId3, lpTokenId, Pica(7));
      transferredTokens = (await getUserTokens(api, walletId3, lpTokenId)).toBn();
      expect(transferredTokens).to.be.bignumber.greaterThan("0");
    });

    it("Given that users has sufficient balance, users can send funds to pool2", async function () {
      const result = await addFundstoThePool(api, poolId2, walletId2, baseAmount, quoteAmount);
      //Once funds added to the pool, User is deposited with LP Tokens.
      expect(BigInt(result.baseAdded.toString(10))).to.be.equal(baseAmount);
      expect(BigInt(result.quoteAdded.toString(10))).to.be.equal(quoteAmount);
      expect(result.walletIdResult.toString()).to.be.equal(walletId2Account);
    });

    it("Given that users has sufficient balance, users can add liquidity to the pool and deposited" +
      " amount is adjusted to maintain asset ratio", async function () {
      const assetAmount = Pica(30);
      const quoteAmount = Pica(100);
      const result = await addFundstoThePool(api, poolId, walletId2, assetAmount, quoteAmount);
      //The deposited amount should be maintained by the dex router hence should maintain 1:1
      expect(result.quoteAdded.toBigInt()).to.be.equal(assetAmount);
      expect(result.walletIdResult.toString()).to.be.equal(walletId2Account);
    });

    it("Given that users have sufficient balance, users can't provide liquidity with specifying " +
      "only quote asset", async function () {
      const baseAmount = Pica(0);
      const quoteAmount = Pica(10000);
      await addFundstoThePool(api, poolId2, walletId1, baseAmount, quoteAmount).catch(e =>
        expect(e.message).to.contain("InvalidAmount")
      );
    });

    it("Given that users have sufficient balance, " +
      "Users can provide liquidity with specifying only base asset and quote amount is calculated and received",
      async function () {
        const baseAmount = Pica(250);
        const quoteAmount = Pica(0);
        const result = await addFundstoThePool(api, poolId2, walletId1, baseAmount, quoteAmount);
        expect(result.quoteAdded.toBn()).to.be.bignumber.greaterThan("0");
      });
  });

  describe("ConstantProductDex buy and sell tests", async function () {
    if (!testConfiguration.enabledTests.buyAndSellTests.enabled) {
      console.log("ConstantProductDex buy and sell tests are being skipped...");
      return;
    }
    this.timeout(2 * 60 * 1000);

    it("Given the pool has sufficient funds, User1 can't completely drain the funds", async function () {
      await buyFromPool(api, poolId, walletId1, baseAssetId, Pica(2530)).catch(error => {
        expect(error.message).to.contain("arithmetic");
      });
    });

    it("Given that the pool has sufficient funds, " +
      "user1 can buy from the pool and amounts are adjusted by the constantProductFormula", async function () {
      const result = await buyFromPool(api, poolId, walletId1, baseAssetId, Pica(30));
      expect(result.accountId.toString()).to.be.equal(walletId1Account);
      //Expected amount is calculated based on the constantProductFormula which is 1:1 for this case.
      expect(result.quoteAmount.toBn()).to.be.bignumber.closeTo(result.expectedConversion.toString(), Pica(1).toString());
    });

    it("Given that there is available liquidity in the pool, " +
      "users can't buy from the pool with amounts greater than the available liquidity", async function () {
      await buyFromPool(api, poolId2, walletId2, quoteAssetId, Pica(5000000)).catch(e =>
        expect(e.message).to.contain("Overflow")
      );
    });

    it("Given that users have available funds, users can sell on the pool", async function () {
      const accountIdSeller = await sellToPool(api, poolId, walletId1, baseAssetId, Pica(20));
      expect(accountIdSeller.toString()).to.be.equal(walletId1Account);
    });

    it("Given that users have available funds, users can swap from the pool", async function () {
      const quotedAmount = Pica(12);
      const result = await swapTokenPairs(api, poolId, walletId2, baseAssetId, quoteAssetId, quotedAmount);
      expect(result.returnedQuoteAmount.toBigInt()).to.be.equal(quotedAmount);
    });
  });

  describe("ConstantProductDex Fee and Other Tests", async function () {
    if (!testConfiguration.enabledTests.feeAndOtherTests.enabled) {
      console.log("ConstantProductDex fee and other tests are being skipped...");
      return;
    }
    this.timeout(2 * 60 * 1000);

    it("Given that there have been previous transactions on the pool, " +
      "owner of the pool receives owner fee", async function () {
      const ownerInitialTokens = await getUserTokens(api, walletId1, quoteAssetId);
      const result = await buyFromPool(api, poolId, walletId2, baseAssetId, Pica(500));
      const ownerAfterTokens = await getUserTokens(api, walletId1, quoteAssetId);
      //verifies the ownerFee to be added in the owner account.
      expect((ownerAfterTokens.toBn())).to.be.bignumber.greaterThan(ownerInitialTokens.toBn());
    });

    it("Given that the pool has liquidity and the users have LPTokens, " +
      "users can remove liquidity from the pool by using LP Tokens", async function () {
      const result = await removeLiquidityFromPool(api, poolId, walletId1, Pica(500));
      expect(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), Pica(15).toString());
    });

    it("Given that LPTokens are transferred to another user, other user can removeLiquidity", async function () {
      const result = await removeLiquidityFromPool(api, poolId, walletId3, BigInt(transferredTokens.toString()) - Pica(5));
      expect(result.resultQuote.toBn()).to.be.bignumber.greaterThan("0");
    });

    it("Given that the users have sufficient balance, " +
      "users can't buy assets that is not listed in the pool", async function () {
      const result = await buyFromPool(api, poolId, walletId2, quoteAsset2, Pica(10)).catch(e =>
        console.log(e.message)
      );
      expect(result.quoteAmount.toBn()).to.be.bignumber.equal("0");
    });

    it("Given that the users have sufficient balance," +
      " users can't swap illegal token pairs(Non existing assetId in the pool)", async function () {
      const quotedAmount = Pica(1200);
      const preSwapPoolAssets = await getPoolBalance(api, poolAddress, baseAssetId);
      const preSwapPoolAssets2 = await getPoolBalance(api, poolAddress, quoteAssetId);
      console.log(preSwapPoolAssets.toString());
      console.log(preSwapPoolAssets2.toString());
      // trying to swap from poolId1 between 2 and 23 which should revert with an error
      const result = await swapTokenPairs(api, poolId, walletId2, baseAssetId, quoteAsset2, quotedAmount);
      const afterSwapPoolAssets = await getPoolBalance(api, poolAddress, baseAssetId);
      const afterSwapPoolAssets2 = await getPoolBalance(api, poolAddress, quoteAsset2);
      const afterSwapPoolAsset3 = await getPoolBalance(api, poolAddress, quoteAssetId);
      //Here it purges existing base asset and creates another pair assetId with
      console.log(afterSwapPoolAssets.toString());
      console.log(afterSwapPoolAssets2.toString());
      console.log(afterSwapPoolAsset3.toString());
      const tryOut = await buyFromPool(api, poolId, walletId1, baseAssetId, Pica(1));
      expect(tryOut.returnedQuoteAmount.toBigInt()).to.be.equal(quotedAmount);
    })
  });
});

