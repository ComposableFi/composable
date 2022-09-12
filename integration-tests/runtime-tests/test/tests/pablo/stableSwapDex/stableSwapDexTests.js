"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const mintingHelper_1 = require("@composable/utils/mintingHelper");
const bn_js_1 = __importDefault(require("bn.js"));
const chai_1 = require("chai");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
const pabloTestHelper_1 = require("@composabletests/tests/pablo/testHandlers/pabloTestHelper");
const test_configuration_json_1 = __importDefault(require("@composabletests/tests/pablo/stableSwapDex/test_configuration.json"));
const test_configuration_json_2 = __importDefault(require("@composabletests/tests/pablo/testHandlers/test_configuration.json"));
/**
 * This suite includes tests for the stableSwapDex.
 * Tested functionalities are:
 * Create - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations of amplification coefficient.
 */
describe("StableSwapDex Test Suite", function () {
    if (!test_configuration_json_2.default.stableSwapTests.enabled) {
        console.log("Liquidity Bootstrapping Tests are being skipped...");
        return;
    }
    this.timeout(3 * 60 * 1000);
    let api;
    let walletId1, walletId2, sudoKey, walletId3;
    let baseStableAssetId, quoteStableAssetId, ampCoefficient, poolId1, poolId2, quoteStableAssetId2, falseQuoteAssetId;
    let baseAssetAmount, quoteAssetAmount, lpTokens;
    let fee, ownerFee;
    let spotPrice1, spotPrice2;
    let removedAssetAmount1, removedAssetAmount2;
    before("Initialize variables and add funds", async function () {
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice, devWalletEve, devWalletFerdie } = (0, walletHelper_1.getDevWallets)(newKeyring);
        walletId1 = devWalletEve.derive("/test/StableSwapDex/walletId1");
        walletId2 = devWalletFerdie.derive("/test/StableSwapDex/walletId2");
        walletId3 = walletId1.derive("/test/StableSwapDex/walletId3");
        sudoKey = devWalletAlice;
        baseStableAssetId = 11;
        quoteStableAssetId = 12;
        quoteStableAssetId2 = 13;
        falseQuoteAssetId = 21;
        ampCoefficient = 24;
        baseAssetAmount = (0, mintingHelper_1.Pica)(250000);
        quoteAssetAmount = (0, mintingHelper_1.Pica)(250000);
        fee = 10000;
        ownerFee = 50000;
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId1, sudoKey, [1, baseStableAssetId, quoteStableAssetId, quoteStableAssetId2]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId2, sudoKey, [1, baseStableAssetId, quoteStableAssetId, quoteStableAssetId2]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId3, sudoKey, [1]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    describe("StableSwapDex Create Pool Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.createPoolTests.enabled) {
            console.log("StableSwapDex create pool tests are being skipped...");
            return;
        }
        this.timeout(3 * 60 * 1000);
        it("[SHORT] Given that users have sufficient funds, User1 can create StableswapPool", async function () {
            const result = await (0, pabloTestHelper_1.createStableSwapPool)(api, walletId1, walletId1, baseStableAssetId, quoteStableAssetId, ampCoefficient, fee);
            poolId1 = result.resultPoolId;
            (0, chai_1.expect)(result.resultPoolId).to.be.a("number");
        });
        it("Given that users have sufficient funds, User2 can create StableswapPool", async function () {
            const result = await (0, pabloTestHelper_1.createStableSwapPool)(api, walletId2, walletId2, baseStableAssetId, quoteStableAssetId2, ampCoefficient + 40000, fee);
            poolId2 = result.resultPoolId;
            (0, chai_1.expect)(result.resultPoolId).to.be.a("number");
        });
        it("Given that users have no funds in their wallets, they can create a StableSwap Pool", async function () {
            const result = await (0, pabloTestHelper_1.createStableSwapPool)(api, walletId2, walletId2, 140, 150, ampCoefficient + 40000, fee);
            (0, chai_1.expect)(result.resultPoolId).to.be.a("number");
        });
    });
    describe("StableSwapDex Add Liquidity Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.addLiquidityTests.enabled) {
            console.log("StableSwapDex addLiquidity tests are being skipped...");
            return;
        }
        this.timeout(3 * 60 * 1000);
        it("Given that users have sufficient funds, User1 can addLiquidity to StableSwapPool", async function () {
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId1, walletId1, baseAssetAmount, quoteAssetAmount);
            lpTokens = result.returnedLPTokens.toBigInt();
            (0, chai_1.expect)(result.quoteAdded.toBigInt()).to.be.equal(quoteAssetAmount);
        });
        it("Given that users have sufficient funds, User2 can addLiquidity to StableSwapPool", async function () {
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId2, walletId2, baseAssetAmount, quoteAssetAmount);
            (0, chai_1.expect)(result.quoteAdded.toBigInt()).to.be.equal(quoteAssetAmount);
        });
        it("Given that users have sufficient funds, User2 can add liquidity and the amount added not adjusted by " +
            "Constantproduct Formula if asset amounts are close to eachother", async function () {
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId2, walletId2, (0, mintingHelper_1.Pica)(100), (0, mintingHelper_1.Pica)(500));
            (0, chai_1.expect)(result.quoteAdded.toBigInt()).to.be.equal((0, mintingHelper_1.Pica)(500));
        });
        it("Given that users have sufficient funds, User1 can add liquidity and the amount added not adjusted by " +
            "Constantproduct Formula", async function () {
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId1, walletId1, (0, mintingHelper_1.Pica)(100), (0, mintingHelper_1.Pica)(500));
            (0, chai_1.expect)(result.quoteAdded.toBigInt()).to.be.equal((0, mintingHelper_1.Pica)(500));
        });
        it("Given that user has provided liquidity to the pool and received LP Tokens, they can transfer tokens" +
            "to another user", async function () {
            const { lpTokenId } = await (0, pabloTestHelper_1.getPoolInfo)(api, "StableSwap", poolId1);
            await (0, pabloTestHelper_1.transferTokens)(api, walletId1, walletId3, lpTokenId, (0, mintingHelper_1.Pica)(5));
            const transferredTokens = (await (0, pabloTestHelper_1.getUserTokens)(api, walletId3, lpTokenId)).toBn();
            (0, chai_1.expect)(transferredTokens).to.be.bignumber.greaterThan("0");
        });
    });
    describe("StableSwapDex buy,sell and swap tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.buySellAndSwapTests.enabled) {
            console.log("StableSwap buy, sell and swap tests are being skipped...");
            return;
        }
        this.timeout(3 * 60 * 1000);
        it("Given that users have sufficient funds, User1 can buy from StableSwapPool", async function () {
            const result = await (0, pabloTestHelper_1.buyFromPool)(api, poolId1, walletId1, quoteStableAssetId, (0, mintingHelper_1.Pica)(25000));
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId1, walletId1, quoteStableAssetId, (0, mintingHelper_1.Pica)(25000));
            const rpcRes = await (0, pabloTestHelper_1.rpcPriceFor)(api, api.createType("PalletPabloPoolId", poolId1), api.createType("CustomRpcCurrencyId", baseStableAssetId), api.createType("CustomRpcCurrencyId", quoteStableAssetId));
            spotPrice1 = parseInt(rpcRes.spotPrice.toString());
            (0, chai_1.expect)(result.quoteAmount.toBn()).to.be.bignumber.closeTo((0, mintingHelper_1.Pica)(25000).toString(), (0, mintingHelper_1.Pica)(1000).toString());
        });
        it("Given that users have sufficient funds in their balance, User2 can't buy a not listed" + " asset in the pool", async function () {
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId1, walletId2, falseQuoteAssetId, (0, mintingHelper_1.Pica)(30)).catch(error => (0, chai_1.expect)(error.message).to.contain("InvalidAsset"));
        });
        it("Given that users have sufficient funds in their balance, User2 can't sell a not listed" + " asset in the pool", async function () {
            await (0, pabloTestHelper_1.sellToPool)(api, poolId1, walletId2, falseQuoteAssetId, (0, mintingHelper_1.Pica)(30)).catch(error => (0, chai_1.expect)(error.message).to.contain("InvalidAsset"));
        });
        it("Given that users have sufficient funds, User2 can buy from StableSwapPool and amounts are " +
            "adjusted by amplificationCoefficient", async function () {
            const result = await (0, pabloTestHelper_1.buyFromPool)(api, poolId2, walletId2, quoteStableAssetId2, (0, mintingHelper_1.Pica)(25000));
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId2, walletId2, quoteStableAssetId2, (0, mintingHelper_1.Pica)(25000));
            const rpcRes = await (0, pabloTestHelper_1.rpcPriceFor)(api, api.createType("PalletPabloPoolId", poolId2), api.createType("CustomRpcCurrencyId", baseStableAssetId), api.createType("CustomRpcCurrencyId", quoteStableAssetId2));
            spotPrice2 = parseInt(rpcRes.spotPrice.toString());
            (0, chai_1.expect)(spotPrice1).to.be.greaterThan(spotPrice2);
            (0, chai_1.expect)(result.quoteAmount.toBn()).to.be.bignumber.closeTo((0, mintingHelper_1.Pica)(25000).toString(), (0, mintingHelper_1.Pica)(1000).toString());
        });
        it("Given that users have sufficient funds, User1 can swap from the pool", async function () {
            const result = await (0, pabloTestHelper_1.swapTokenPairs)(api, poolId1, walletId1, baseStableAssetId, quoteStableAssetId, (0, mintingHelper_1.Pica)(150));
            (0, chai_1.expect)(result.returnedQuoteAmount.toBigInt()).to.be.equal((0, mintingHelper_1.Pica)(150));
        });
        it("Given that users have sufficient funds, User2 can swap from the pool", async function () {
            const result = await (0, pabloTestHelper_1.swapTokenPairs)(api, poolId2, walletId2, baseStableAssetId, quoteStableAssetId2, (0, mintingHelper_1.Pica)(150));
            (0, chai_1.expect)(result.returnedQuoteAmount.toBigInt()).to.be.equal((0, mintingHelper_1.Pica)(150));
        });
        it("Given that users have sufficient funds, User1 can sell to the pool", async function () {
            const result = await (0, pabloTestHelper_1.sellToPool)(api, poolId1, walletId2, baseStableAssetId, (0, mintingHelper_1.Pica)(30));
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", walletId2.address).toString());
        });
    });
    describe("StableSwapDex liquidityremoval and other tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.liquidityRemovalandOtherTests.enabled) {
            console.log("StableSwap liquidity removal and other tests are being skipped...");
            return;
        }
        this.timeout(2 * 60 * 1000);
        it("Given that users have sufficient LPTokens, User1 can remove liquidity from the pool", async function () {
            const result = await (0, pabloTestHelper_1.removeLiquidityFromPool)(api, poolId1, walletId1, lpTokens - (0, mintingHelper_1.Pica)(10000));
            (0, chai_1.expect)(result.resultBase.toBigInt()).to.be.a("bigint");
            //The rate stays in 10% range based on the liquidity in the pool
            (0, chai_1.expect)(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), (0, mintingHelper_1.Pica)(100000).toString());
            removedAssetAmount1 = new bn_js_1.default((result.resultBase.toBigInt() - result.resultQuote.toBigInt()).toString());
        });
        it("Given that users have received lpTokens, they can remove liquidity from the pool", async function () {
            const result = await (0, pabloTestHelper_1.removeLiquidityFromPool)(api, poolId1, walletId3, (0, mintingHelper_1.Pica)(5));
            (0, chai_1.expect)(result.resultQuote.toBn()).to.be.bignumber.greaterThan("0");
        });
        it("Given that users have sufficient LPTokens, User2 can remove liquidity from the pool", async function () {
            const result = await (0, pabloTestHelper_1.removeLiquidityFromPool)(api, poolId2, walletId2, lpTokens - (0, mintingHelper_1.Pica)(10000));
            (0, chai_1.expect)(result.resultBase.toBigInt()).to.be.a("bigint");
            //The rate stays in 30% range based on the liquidity in the pool
            (0, chai_1.expect)(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), (0, mintingHelper_1.Pica)(100000).toString());
            removedAssetAmount2 = new bn_js_1.default((result.resultBase.toBigInt() - result.resultQuote.toBigInt()).toString());
            //Verify that the difference between assets should be higher in higher coefficient amplification pool
            (0, chai_1.expect)(removedAssetAmount2).to.be.bignumber.gte(removedAssetAmount1);
        });
        it("Given that users have sufficient funds, User1 can't swap two tokens not listed in the pool", async function () {
            //Expected behavior is to reject with error
            await (0, pabloTestHelper_1.swapTokenPairs)(api, poolId1, walletId1, baseStableAssetId, falseQuoteAssetId, (0, mintingHelper_1.Pica)(50)).catch(error => (0, chai_1.expect)(error.message).to.contain("Mismatch"));
        });
    });
});
//# sourceMappingURL=stableSwapDexTests.js.map