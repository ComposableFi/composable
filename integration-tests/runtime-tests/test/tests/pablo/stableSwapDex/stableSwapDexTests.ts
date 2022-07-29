import { KeyringPair } from "@polkadot/keyring/types";
import { mintAssetsToWallet, Pica } from "@composable/utils/mintingHelper";
import BN from "bn.js";
import { expect } from "chai";
import { getNewConnection } from "@composable/utils/connectionHelper";
import { getDevWallets } from "@composable/utils/walletHelper";
import { ApiPromise } from "@polkadot/api";
import {
  addFundstoThePool,
  buyFromPool,
  createStableSwapPool,
  getPoolInfo,
  getUserTokens,
  removeLiquidityFromPool,
  rpcPriceFor,
  sellToPool,
  swapTokenPairs,
  transferTokens
} from "@composabletests/tests/pablo/testHandlers/pabloTestHelper";
import testConfiguration from "@composabletests/tests/pablo/stableSwapDex/test_configuration.json";
import pabloTestConfiguration from "@composabletests/tests/pablo/testHandlers/test_configuration.json";

/**
 * This suite includes tests for the stableSwapDex.
 * Tested functionalities are:
 * Create - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations of amplification coefficient.
 */

describe("StableSwapDex Test Suite", function () {
  if (!pabloTestConfiguration.stableSwapTests.enabled) {
    console.log("Liquidity Bootstrapping Tests are being skipped...");
    return;
  }
  this.timeout(3 * 60 * 1000);
  let api: ApiPromise;
  let walletId1: KeyringPair, walletId2: KeyringPair, sudoKey: KeyringPair, walletId3: KeyringPair;
  let baseStableAssetId: number,
    quoteStableAssetId: number,
    ampCoefficient: number,
    poolId1: number,
    poolId2: number,
    quoteStableAssetId2: number,
    falseQuoteAssetId: number;
  let baseAssetAmount: bigint, quoteAssetAmount: bigint, lpTokens: bigint;
  let fee: number, ownerFee: number;
  let spotPrice1: number, spotPrice2: number;
  let removedAssetAmount1: BN, removedAssetAmount2: BN;

  before("Initialize variables and add funds", async function () {
    const { newClient, newKeyring } = await getNewConnection();
    api = newClient;
    const { devWalletAlice, devWalletEve, devWalletFerdie } = getDevWallets(newKeyring);
    walletId1 = devWalletEve.derive("/test/StableSwapDex/walletId1");
    walletId2 = devWalletFerdie.derive("/test/StableSwapDex/walletId2");
    walletId3 = walletId1.derive("/test/StableSwapDex/walletId3");
    sudoKey = devWalletAlice;
    baseStableAssetId = 11;
    quoteStableAssetId = 12;
    quoteStableAssetId2 = 13;
    falseQuoteAssetId = 21;
    ampCoefficient = 24;
    baseAssetAmount = Pica(250000);
    quoteAssetAmount = Pica(250000);
    fee = 10000;
    ownerFee = 50000;
    await mintAssetsToWallet(api, walletId1, sudoKey, [1, baseStableAssetId, quoteStableAssetId, quoteStableAssetId2]);
    await mintAssetsToWallet(api, walletId2, sudoKey, [1, baseStableAssetId, quoteStableAssetId, quoteStableAssetId2]);
    await mintAssetsToWallet(api, walletId3, sudoKey, [1]);
  });

  after("Closing the connection", async function () {
    await api.disconnect();
  });

  describe("StableSwapDex Create Pool Tests", async function () {
    if (!testConfiguration.enabledTests.createPoolTests.enabled) {
      console.log("StableSwapDex create pool tests are being skipped...");
      return;
    }
    this.timeout(3 * 60 * 1000);

    it("[SHORT] Given that users have sufficient funds, User1 can create StableswapPool", async function () {
      const result = await createStableSwapPool(
        api,
        sudoKey,
        walletId1,
        baseStableAssetId,
        quoteStableAssetId,
        ampCoefficient,
        fee
      );
      poolId1 = result.resultPoolId;
      expect(result.resultPoolId).to.be.a("number");
    });

    it("Given that users have sufficient funds, User2 can create StableswapPool", async function () {
      const result = await createStableSwapPool(
        api,
        sudoKey,
        walletId2,
        baseStableAssetId,
        quoteStableAssetId2,
        ampCoefficient + 40000,
        fee
      );
      poolId2 = result.resultPoolId;
      expect(result.resultPoolId).to.be.a("number");
    });

    it("Given that users have no funds in their wallets, they can create a StableSwap Pool", async function () {
      const result = await createStableSwapPool(api, sudoKey, walletId2, 140, 150, ampCoefficient + 40000, fee);
      expect(result.resultPoolId).to.be.a("number");
    });
  });

  describe("StableSwapDex Add Liquidity Tests", async function () {
    if (!testConfiguration.enabledTests.addLiquidityTests.enabled) {
      console.log("StableSwapDex addLiquidity tests are being skipped...");
      return;
    }
    this.timeout(3 * 60 * 1000);

    it("Given that users have sufficient funds, User1 can addLiquidity to StableSwapPool", async function () {
      const result = await addFundstoThePool(api, poolId1, walletId1, baseAssetAmount, quoteAssetAmount);
      lpTokens = result.returnedLPTokens.toBigInt();
      expect(result.quoteAdded.toBigInt()).to.be.equal(quoteAssetAmount);
    });

    it("Given that users have sufficient funds, User2 can addLiquidity to StableSwapPool", async function () {
      const result = await addFundstoThePool(api, poolId2, walletId2, baseAssetAmount, quoteAssetAmount);
      expect(result.quoteAdded.toBigInt()).to.be.equal(quoteAssetAmount);
    });

    it(
      "Given that users have sufficient funds, User2 can add liquidity and the amount added not adjusted by " +
        "Constantproduct Formula if asset amounts are close to eachother",
      async function () {
        const result = await addFundstoThePool(api, poolId2, walletId2, Pica(100), Pica(500));
        expect(result.quoteAdded.toBigInt()).to.be.equal(Pica(500));
      }
    );

    it(
      "Given that users have sufficient funds, User1 can add liquidity and the amount added not adjusted by " +
        "Constantproduct Formula",
      async function () {
        const result = await addFundstoThePool(api, poolId1, walletId1, Pica(100), Pica(500));
        expect(result.quoteAdded.toBigInt()).to.be.equal(Pica(500));
      }
    );

    it(
      "Given that user has provided liquidity to the pool and received LP Tokens, they can transfer tokens" +
        "to another user",
      async function () {
        const { lpTokenId } = await getPoolInfo(api, "StableSwap", poolId1);
        await transferTokens(api, walletId1, walletId3, lpTokenId, Pica(5));
        const transferredTokens = (await getUserTokens(api, walletId3, lpTokenId)).toBn();
        expect(transferredTokens).to.be.bignumber.greaterThan("0");
      }
    );
  });

  describe("StableSwapDex buy,sell and swap tests", async function () {
    if (!testConfiguration.enabledTests.buySellAndSwapTests.enabled) {
      console.log("StableSwap buy, sell and swap tests are being skipped...");
      return;
    }
    this.timeout(3 * 60 * 1000);

    it("Given that users have sufficient funds, User1 can buy from StableSwapPool", async function () {
      const result = await buyFromPool(api, poolId1, walletId1, quoteStableAssetId, Pica(25000));
      await buyFromPool(api, poolId1, walletId1, quoteStableAssetId, Pica(25000));
      const rpcRes = await rpcPriceFor(
        api,
        api.createType("PalletPabloPoolId", poolId1),
        api.createType("CustomRpcCurrencyId", baseStableAssetId),
        api.createType("CustomRpcCurrencyId", quoteStableAssetId)
      );
      spotPrice1 = parseInt(rpcRes.spotPrice.toString());
      expect(result.quoteAmount.toBn()).to.be.bignumber.closeTo(Pica(25000).toString(), Pica(1000).toString());
    });

    it(
      "Given that users have sufficient funds in their balance, User2 can't buy a not listed" + " asset in the pool",
      async function () {
        await buyFromPool(api, poolId1, walletId2, falseQuoteAssetId, Pica(30)).catch(error =>
          expect(error.message).to.contain("InvalidAsset")
        );
      }
    );

    it(
      "Given that users have sufficient funds in their balance, User2 can't sell a not listed" + " asset in the pool",
      async function () {
        await sellToPool(api, poolId1, walletId2, falseQuoteAssetId, Pica(30)).catch(error =>
          expect(error.message).to.contain("InvalidAsset")
        );
      }
    );

    it(
      "Given that users have sufficient funds, User2 can buy from StableSwapPool and amounts are " +
        "adjusted by amplificationCoefficient",
      async function () {
        const result = await buyFromPool(api, poolId2, walletId2, quoteStableAssetId2, Pica(25000));
        await buyFromPool(api, poolId2, walletId2, quoteStableAssetId2, Pica(25000));
        const rpcRes = await rpcPriceFor(
          api,
          api.createType("PalletPabloPoolId", poolId2),
          api.createType("CustomRpcCurrencyId", baseStableAssetId),
          api.createType("CustomRpcCurrencyId", quoteStableAssetId2)
        );
        spotPrice2 = parseInt(rpcRes.spotPrice.toString());
        expect(spotPrice1).to.be.greaterThan(spotPrice2);
        expect(result.quoteAmount.toBn()).to.be.bignumber.closeTo(Pica(25000).toString(), Pica(1000).toString());
      }
    );

    it("Given that users have sufficient funds, User1 can swap from the pool", async function () {
      const result = await swapTokenPairs(api, poolId1, walletId1, baseStableAssetId, quoteStableAssetId, Pica(150));
      expect(result.returnedQuoteAmount.toBigInt()).to.be.equal(Pica(150));
    });

    it("Given that users have sufficient funds, User2 can swap from the pool", async function () {
      const result = await swapTokenPairs(api, poolId2, walletId2, baseStableAssetId, quoteStableAssetId2, Pica(150));
      expect(result.returnedQuoteAmount.toBigInt()).to.be.equal(Pica(150));
    });

    it("Given that users have sufficient funds, User1 can sell to the pool", async function () {
      const result = await sellToPool(api, poolId1, walletId2, baseStableAssetId, Pica(30));
      expect(result.toString()).to.be.equal(api.createType("AccountId32", walletId2.address).toString());
    });
  });

  describe("StableSwapDex liquidityremoval and other tests", async function () {
    if (!testConfiguration.enabledTests.liquidityRemovalandOtherTests.enabled) {
      console.log("StableSwap liquidity removal and other tests are being skipped...");
      return;
    }
    this.timeout(2 * 60 * 1000);

    it("Given that users have sufficient LPTokens, User1 can remove liquidity from the pool", async function () {
      const result = await removeLiquidityFromPool(api, poolId1, walletId1, lpTokens - Pica(10000));
      expect(result.resultBase.toBigInt()).to.be.a("bigint");
      //The rate stays in 10% range based on the liquidity in the pool
      expect(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), Pica(100000).toString());
      removedAssetAmount1 = new BN((result.resultBase.toBigInt() - result.resultQuote.toBigInt()).toString());
    });

    it("Given that users have received lpTokens, they can remove liquidity from the pool", async function () {
      const result = await removeLiquidityFromPool(api, poolId1, walletId3, Pica(5));
      expect(result.resultQuote.toBn()).to.be.bignumber.greaterThan("0");
    });

    it("Given that users have sufficient LPTokens, User2 can remove liquidity from the pool", async function () {
      const result = await removeLiquidityFromPool(api, poolId2, walletId2, lpTokens - Pica(10000));
      expect(result.resultBase.toBigInt()).to.be.a("bigint");
      //The rate stays in 30% range based on the liquidity in the pool
      expect(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), Pica(100000).toString());
      removedAssetAmount2 = new BN((result.resultBase.toBigInt() - result.resultQuote.toBigInt()).toString());
      //Verify that the difference between assets should be higher in higher coefficient amplification pool
      expect(removedAssetAmount2).to.be.bignumber.gte(removedAssetAmount1);
    });

    it("Given that users have sufficient funds, User1 can't swap two tokens not listed in the pool", async function () {
      //Expected behavior is to reject with error
      await swapTokenPairs(api, poolId1, walletId1, baseStableAssetId, falseQuoteAssetId, Pica(50)).catch(error =>
        expect(error.message).to.contain("Mismatch")
      );
    });
  });
});
