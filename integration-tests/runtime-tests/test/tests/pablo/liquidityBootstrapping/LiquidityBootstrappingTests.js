"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const mintingHelper_1 = require("@composable/utils/mintingHelper");
const chai_1 = require("chai");
const connectionHelper_1 = require("@composable/utils/connectionHelper");
const walletHelper_1 = require("@composable/utils/walletHelper");
const polkadotjs_1 = require("@composable/utils/polkadotjs");
const pabloTestHelper_1 = require("@composabletests/tests/pablo/testHandlers/pabloTestHelper");
const test_configuration_json_1 = __importDefault(require("@composabletests/tests/pablo/liquidityBootstrapping/test_configuration.json"));
const test_configuration_json_2 = __importDefault(require("../testHandlers/test_configuration.json"));
/**
 * This suite includes tests for the Liquidity Bootstrapping Pool.
 * Tested functionalities are:
 * CreatePool - AddLiquidity - Buy - Sell - Swap - RemoveLiquidity with basic calculations
 */
describe("LiquidityBootsrapping Pool Test Suite", function () {
    if (!test_configuration_json_2.default.liquidityBootstrappingTests.enabled) {
        console.log("Liquidity Bootstrapping Tests are being skipped...");
        return;
    }
    this.timeout(3 * 60 * 1000);
    let api;
    let poolId1, poolId2, feeRate, ownerFeeRate, protocolFeeRate;
    let walletId1, walletId2, sudoKey;
    let baseAssetId, quoteAssetId, quoteAssetId2;
    let baseAmount, quoteAmount;
    let startTime, endTime, initialWeight, finalWeight;
    before("Given that users have sufficient balance", async function () {
        const { newClient, newKeyring } = await (0, connectionHelper_1.getNewConnection)();
        api = newClient;
        const { devWalletAlice, devWalletEve, devWalletFerdie } = (0, walletHelper_1.getDevWallets)(newKeyring);
        walletId1 = devWalletFerdie.derive("/test/lbpTests/wallet1");
        walletId2 = devWalletEve.derive("/test/lbpTests/wallet2");
        sudoKey = devWalletAlice;
        baseAssetId = 15;
        quoteAssetId = 20;
        quoteAssetId2 = 25;
        //Gets latest block number
        const latestHead = await api.rpc.chain.getFinalizedHead();
        const latestBlock = (await api.rpc.chain.getHeader(latestHead)).number;
        startTime = latestBlock.toNumber() + 24;
        //Should be a number between 216000 - 7200
        endTime = startTime + 7300;
        //Max Initial weight is 95%, here it is passed as 50%
        initialWeight = 900000;
        //Min final weight is 5%, here it is as passed as 6%
        finalWeight = 60000;
        //Sets 1% fee
        feeRate = 10000;
        ownerFeeRate = 200000;
        protocolFeeRate = 600000;
        baseAmount = (0, mintingHelper_1.Pica)(3000);
        quoteAmount = (0, mintingHelper_1.Pica)(10000);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId1, sudoKey, [1, baseAssetId, quoteAssetId]);
        await (0, mintingHelper_1.mintAssetsToWallet)(api, walletId2, sudoKey, [1, baseAssetId, quoteAssetId, quoteAssetId2]);
    });
    after("Closing the connection", async function () {
        await api.disconnect();
    });
    describe("LiquidityBootstrapping CreatePool and AddLiquidity Tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.createPoolAndAddLiquidityTests.enabled) {
            console.log("LiquidityBootsrapping createPool and addLiquidity tests are being skipped...");
            return;
        }
        this.timeout(2 * 60 * 1000);
        it("User1 can create a LBP", async function () {
            const result = await (0, pabloTestHelper_1.createLBPool)(api, walletId1, walletId1, baseAssetId, quoteAssetId, startTime, endTime, initialWeight, finalWeight, feeRate, ownerFeeRate, protocolFeeRate);
            poolId1 = result.resultPoolId;
            (0, chai_1.expect)(poolId1).to.be.a("number");
        });
        it("Users can create multiple LB Pools with the same token pairs", async function () {
            const result = await (0, pabloTestHelper_1.createLBPool)(api, walletId1, walletId1, baseAssetId, quoteAssetId, startTime, endTime, initialWeight, finalWeight, feeRate, ownerFeeRate, protocolFeeRate);
            (0, chai_1.expect)(result.resultPoolId).to.be.a("number");
        });
        it("Users can add liquidity to the LB pool before the sale started", async function () {
            const result = await (0, pabloTestHelper_1.addFundstoThePool)(api, poolId1, walletId1, baseAmount, quoteAmount);
            (0, chai_1.expect)(result.baseAdded.toBigInt()).to.be.equal(baseAmount);
            (0, chai_1.expect)(result.quoteAdded.toBigInt()).to.be.equal(quoteAmount);
        });
        it("Given that the pool created, Owner can enable twap", async function () {
            const { data: [result] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sudoKey, api.events.sudo.Sudid.is, api.tx.sudo.sudo(api.tx.pablo.enableTwap(api.createType("u128", poolId1))));
            (0, chai_1.expect)(result.isOk).to.be.true;
        });
        it("Given that the LB pool has liquidity in the pool, before the sale started, it returns 0 spot price", async function () {
            const rpcRes = await (0, pabloTestHelper_1.rpcPriceFor)(api, api.createType("PalletPabloPoolId", poolId1), api.createType("CustomRpcCurrencyId", baseAssetId), api.createType("CustomRpcCurrencyId", quoteAssetId));
            (0, chai_1.expect)(rpcRes.spotPrice.toString()).to.be.equal("0");
        });
        it("Given that the pool has sufficient liquidity, user1 can't buy from the pool before the sale starts", async function () {
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId1, walletId1, baseAssetId, (0, mintingHelper_1.Pica)(5)).catch(e => (0, chai_1.expect)(e.message).to.contain("InvalidSaleState"));
        });
        it("Users can create multiple LB Pools with random valid parameters", async function () {
            const prePoolCount = (await api.query.pablo.poolCount()).toNumber();
            await (0, pabloTestHelper_1.createMultipleLBPools)(api, walletId1);
            (0, chai_1.expect)((await api.query.pablo.poolCount()).toNumber()).to.be.equal(500 + prePoolCount);
        });
        it("The users can't create LB Pools with invalid params " +
            "{initial weight>95, end weight<5, saleDuration<7200, saleDuration>216000", async function () {
            this.timeout(3 * 60 * 1000);
            const weights = [950001, 49999];
            const durations = [7100, 216001];
            for (const weight of weights) {
                await (0, pabloTestHelper_1.createLBPool)(api, walletId1, walletId1, baseAssetId, quoteAssetId, startTime, endTime, weight, 50001, feeRate, ownerFeeRate, protocolFeeRate).catch(e => (0, chai_1.expect)(e.message).to.contain("Other"));
            }
            for (const duration of durations) {
                await (0, pabloTestHelper_1.createLBPool)(api, walletId1, walletId1, baseAssetId, quoteAssetId, startTime, startTime + duration, initialWeight, finalWeight, feeRate, ownerFeeRate, protocolFeeRate).catch(e => (0, chai_1.expect)(e.message).to.contain("Other"));
            }
        });
    });
    describe("LiquidityBootstrapping buy sell and swap tests", async function () {
        if (!test_configuration_json_1.default.enabledTests.buySellandSwapTests.enabled) {
            console.log("LiquidityBootsrapping buy,sell and swap tests are being skipped...");
            return;
        }
        this.timeout(3 * 60 * 1000);
        it("User2 can buy from the pool once the sale started", async function () {
            const result = await (0, pabloTestHelper_1.buyFromPool)(api, poolId1, walletId2, quoteAssetId, (0, mintingHelper_1.Pica)(30));
            (0, chai_1.expect)(result.baseAmount.toBn()).to.be.bignumber.closeTo((0, mintingHelper_1.Pica)(30).toString(), (0, mintingHelper_1.Pica)(10).toString());
        });
        it("Given that users have sufficient funds, user1 can't buy an asset not listed in the pool", async function () {
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId1, walletId2, quoteAssetId2, (0, mintingHelper_1.Pica)(10)).catch(error => (0, chai_1.expect)(error.message).to.contain("InvalidAsset"));
        });
        it("User1 can sell to the pool once the sale started", async function () {
            const result = await (0, pabloTestHelper_1.sellToPool)(api, poolId1, walletId1, baseAssetId, (0, mintingHelper_1.Pica)(50));
            (0, chai_1.expect)(result.toString()).to.be.equal(api.createType("AccountId32", walletId1.address).toString());
        });
        it("User can't buy more than the amount available in the pool", async function () {
            await (0, pabloTestHelper_1.buyFromPool)(api, poolId2, walletId2, baseAssetId, (0, mintingHelper_1.Pica)("4000")).catch(error => (0, chai_1.expect)(error.message).to.contain("InvalidAsset"));
        });
        it("User1 can swap from the pool once the sale started", async function () {
            const result = await (0, pabloTestHelper_1.swapTokenPairs)(api, poolId1, walletId1, baseAssetId, quoteAssetId, (0, mintingHelper_1.Pica)(60));
            (0, chai_1.expect)(result.returnedQuoteAmount.toBigInt()).to.be.equal((0, mintingHelper_1.Pica)(60));
        });
        it("User2 can't swap from the pool with nonexisting assetId's", async function () {
            await (0, pabloTestHelper_1.swapTokenPairs)(api, poolId1, walletId1, baseAssetId, quoteAssetId2, (0, mintingHelper_1.Pica)(50)).catch(error => (0, chai_1.expect)(error.message).to.contain("PairMismatch"));
        });
    });
});
//# sourceMappingURL=LiquidityBootstrappingTests.js.map