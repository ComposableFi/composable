"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const test_configuration_json_1 = __importDefault(require("../testHandlers/test_configuration.json"));
const test_configuration_json_2 = __importDefault(require("./test_configuration.json"));
const chai_1 = require("chai");
const mintingHelper_1 = require("@composable/utils/mintingHelper");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
const pabloTestHelper_1 = require("@composabletests/tests/pablo/testHandlers/pabloTestHelper");
/**
 * This suite includes tests for the constantProductDex Pallet.
 * Tested functionalities are:
 * Create - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations with constantProductFormula
 *    and OwnerFee.
 * Mainly consists of happy path testing.
 */
describe("tx.constantProductDex Tests", function () {
    if (!test_configuration_json_1.default.constantProductTests.enabled) {
        console.log("Constant Product Tests are being skipped...");
        return;
    }
    this.timeout(3 * 60 * 1000);
    let api;
    let walletId1, walletId2, walletId3, sudoKey;
    let poolId, poolId2, baseAssetId, baseAsset2, quoteAssetId, falseQuoteAsset, fee, baseWeight;
    let baseAmount, quoteAmount;
    let transferredTokens;
    let walletId1Account, walletId2Account;
    before("Initialize variables", async function () {
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice, devWalletEve, devWalletFerdie } = (0, walletHelper_1.getDevWallets)(newKeyring);
        sudoKey = devWalletAlice;
        walletId1 = devWalletEve.derive("/test/constantProductDex/walletId1");
        walletId2 = devWalletFerdie.derive("/test/constantProductDex/walletId2");
        walletId3 = devWalletAlice.derive("/test/constantProductDex/walletId3");
        walletId1Account = api.createType("AccountId32", walletId1.address).toString();
        walletId2Account = api.createType("AccountId32", walletId2.address).toString();
        baseAssetId = 2;
        quoteAssetId = 3;
        baseAsset2 = 7;
        falseQuoteAsset = 23;
        baseAmount = (0, mintingHelper_1.Pica)(250000);
        quoteAmount = (0, mintingHelper_1.Pica)(250000);
        //sets the fee to 1.00%/Type Permill
        fee = 10000;
        //sets the weight of the asset pairs to 50.00%/Type Permill
        baseWeight = 500000;
    });
    before("Minting assets", async function () {
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId1, sudoKey, [1, baseAssetId, quoteAssetId, baseAsset2, falseQuoteAsset]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId2, sudoKey, [1, baseAssetId, quoteAssetId, baseAsset2, falseQuoteAsset]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId3, sudoKey, [1]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    describe("tx.constantProductDex Create Pool Tests", function () {
        if (!test_configuration_json_2.default.enabledTests.createPoolTests.enabled) {
            console.log("ConstantProduct create pools tests are being skipped...");
            return;
        }
        this.timeout(2 * 60 * 1000);
        it("[SHORT] Given that users are on the chain, users can create a ConstantProduct pool", async function () {
            poolId = await (0, pabloTestHelper_1.createConsProdPool)(api, walletId1, walletId1, baseAssetId, quoteAssetId, fee, baseWeight);
            //verify if the pool is created
            (0, chai_1.expect)(poolId).to.be.a("number");
        });
        it("Given that users are on the chain, users can create another ConstantProduct pool with different assetIds", async function () {
            poolId2 = await (0, pabloTestHelper_1.createConsProdPool)(api, walletId2, walletId2, baseAssetId, baseAsset2, fee, baseWeight);
            //verify if the pool is created
            (0, chai_1.expect)(poolId2).to.be.a("number");
        });
        it("Given that users have no active balance on assets, users can create ConstantProduct Pool", async function () {
            const result = await (0, pabloTestHelper_1.createConsProdPool)(api, walletId2, walletId2, 50, 60, fee, baseWeight);
            (0, chai_1.expect)(result).to.be.a("number");
        });
        it("Given that the chain is up, users can create pools-" +
            " test creates up to 500 Constant Product pools with valid random fees, random owner fees and random assetIds", async function () {
            await (0, pabloTestHelper_1.createMultipleCPPools)(api, walletId1);
            (0, chai_1.expect)((await api.query.pablo.poolCount()).toNumber()).to.be.greaterThan(500);
        });
    });
    describe("ConstantProductDex Add Liquidity Tests", async function () {
        if (!test_configuration_json_2.default.enabledTests.addLiquidityTests.enabled) {
            console.log("ConstantProductDex add liquidity tests are being skipped...");
            return;
        }
        this.timeout(2 * 60 * 1000);
        it("Given that users has sufficient balance, User1 can send funds to pool", async function () {
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId, walletId1, baseAmount, quoteAmount);
            (0, chai_1.expect)(BigInt(result.baseAdded.toString(10))).to.be.equal(baseAmount);
            (0, chai_1.expect)(BigInt(result.quoteAdded.toString(10))).to.be.equal(quoteAmount);
            (0, chai_1.expect)(result.walletIdResult.toString()).to.be.equal(walletId1Account);
        });
        it("Given that users have LPTokens, users can transfer LP Tokens to another user", async function () {
            const { lpTokenId } = await (0, pabloTestHelper_1.getPoolInfo)(api, "ConstantProduct", poolId);
            await (0, pabloTestHelper_1.transferTokens)(api, walletId1, walletId3, lpTokenId, (0, mintingHelper_1.Pica)(7));
            transferredTokens = (await (0, pabloTestHelper_1.getUserTokens)(api, walletId3, lpTokenId)).toBn();
            (0, chai_1.expect)(transferredTokens).to.be.bignumber.greaterThan("0");
        });
        it("Given that users has sufficient balance, users can send funds to pool2", async function () {
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId2, walletId2, baseAmount, quoteAmount);
            //Once funds added to the pool, User is deposited with LP Tokens.
            (0, chai_1.expect)(BigInt(result.baseAdded.toString(10))).to.be.equal(baseAmount);
            (0, chai_1.expect)(BigInt(result.quoteAdded.toString(10))).to.be.equal(quoteAmount);
            (0, chai_1.expect)(result.walletIdResult.toString()).to.be.equal(walletId2Account);
        });
        it("Given that users has sufficient balance, users can add liquidity to the pool and deposited" +
            " amount is adjusted to maintain asset ratio", async function () {
            const assetAmount = (0, mintingHelper_1.Pica)(30);
            const quoteAmount = (0, mintingHelper_1.Pica)(100);
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId, walletId2, assetAmount, quoteAmount);
            //The deposited amount should be maintained by the dex router hence should maintain 1:1
            (0, chai_1.expect)(result.quoteAdded.toBigInt()).to.be.equal(assetAmount);
            (0, chai_1.expect)(result.walletIdResult.toString()).to.be.equal(walletId2Account);
        });
        it("Given that users have sufficient balance, users can't provide liquidity with specifying " + "only quote asset", async function () {
            const baseAmount = (0, mintingHelper_1.Pica)(0);
            const quoteAmount = (0, mintingHelper_1.Pica)(10000);
            await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId2, walletId1, baseAmount, quoteAmount).catch(error => (0, chai_1.expect)(error.message).to.contain("InvalidAmount"));
        });
        it("Given that users have sufficient balance, " +
            "Users can provide liquidity with specifying only base asset and quote amount is calculated and received", async function () {
            const baseAmount = (0, mintingHelper_1.Pica)(250);
            const quoteAmount = (0, mintingHelper_1.Pica)(0);
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId2, walletId1, baseAmount, quoteAmount);
            (0, chai_1.expect)(result.quoteAdded.toBn()).to.be.bignumber.greaterThan("0");
        });
    });
    describe("ConstantProductDex buy and sell tests", async function () {
        if (!test_configuration_json_2.default.enabledTests.buyAndSellTests.enabled) {
            console.log("ConstantProductDex buy and sell tests are being skipped...");
            return;
        }
        this.timeout(2 * 60 * 1000);
        it("Given the pool has sufficient funds, User1 can't completely drain the funds", async function () {
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId, walletId1, baseAssetId, (0, mintingHelper_1.Pica)(2530)).catch(error => (0, chai_1.expect)(error.message).to.contain("arithmetic"));
        });
        it("Given that the pool has sufficient funds, " +
            "user1 can buy from the pool and amounts are adjusted by the constantProductFormula", async function () {
            const result = await (0, pabloTestHelper_1.buyFromPool)(api, poolId, walletId1, baseAssetId, (0, mintingHelper_1.Pica)(30));
            (0, chai_1.expect)(result.accountId.toString()).to.be.equal(walletId1Account);
            //Expected amount is calculated based on the constantProductFormula which is 1:1 for this case.
            (0, chai_1.expect)(result.quoteAmount.toBn()).to.be.bignumber.closeTo(result.expectedConversion.toString(), (0, mintingHelper_1.Pica)(1).toString());
        });
        it("Given that there is available liquidity in the pool, " +
            "users can't buy from the pool with amounts greater than the available liquidity", async function () {
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId2, walletId2, baseAsset2, (0, mintingHelper_1.Pica)(5000000)).catch(error => (0, chai_1.expect)(error.message).to.contain("Overflow"));
        });
        it("Given that users have available funds, users can sell on the pool", async function () {
            const accountIdSeller = await (0, pabloTestHelper_1.sellToPool)(api, poolId, walletId1, baseAssetId, (0, mintingHelper_1.Pica)(20));
            (0, chai_1.expect)(accountIdSeller.toString()).to.be.equal(walletId1Account);
        });
        it("Given that users have available funds, users can swap from the pool", async function () {
            const quotedAmount = (0, mintingHelper_1.Pica)(12);
            const result = await (0, pabloTestHelper_1.swapTokenPairs)(api, poolId, walletId2, baseAssetId, quoteAssetId, quotedAmount);
            (0, chai_1.expect)(result.returnedQuoteAmount.toBigInt()).to.be.equal(quotedAmount);
        });
    });
    describe("ConstantProductDex Fee and Other Tests", async function () {
        if (!test_configuration_json_2.default.enabledTests.feeAndOtherTests.enabled) {
            console.log("ConstantProductDex fee and other tests are being skipped...");
            return;
        }
        this.timeout(2 * 60 * 1000);
        it("Given that the pool has liquidity and the users have LPTokens, " +
            "users can remove liquidity from the pool by using LP Tokens", async function () {
            const result = await (0, pabloTestHelper_1.removeLiquidityFromPool)(api, poolId, walletId1, (0, mintingHelper_1.Pica)(500));
            (0, chai_1.expect)(result.resultBase.toBn()).to.be.bignumber.closeTo(result.resultQuote.toBn(), (0, mintingHelper_1.Pica)(15).toString());
        });
        it("Given that LPTokens are transferred to another user, other user can removeLiquidity", async function () {
            const result = await (0, pabloTestHelper_1.removeLiquidityFromPool)(api, poolId, walletId3, BigInt(transferredTokens.toString()) - (0, mintingHelper_1.Pica)(5));
            (0, chai_1.expect)(result.resultQuote.toBn()).to.be.bignumber.greaterThan("0");
        });
        it("Given that the users have sufficient balance, " + "users can't buy assets that is not listed in the pool", async function () {
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId, walletId2, falseQuoteAsset, (0, mintingHelper_1.Pica)(10)).catch(error => (0, chai_1.expect)(error.message).to.contain("InvalidAsset"));
        });
        it("Given that the users have sufficient balance," +
            " users can't swap illegal token pairs(Non existing assetId in the pool)", async function () {
            const quotedAmount = (0, mintingHelper_1.Pica)(1200);
            // trying to swap from poolId1 between 2 and 23 which should revert with an error
            await (0, pabloTestHelper_1.swapTokenPairs)(api, poolId, walletId2, baseAssetId, falseQuoteAsset, quotedAmount).catch(error => (0, chai_1.expect)(error.message).to.contain("PairMismatch"));
        });
    });
});
//# sourceMappingURL=constantProductTests.js.map