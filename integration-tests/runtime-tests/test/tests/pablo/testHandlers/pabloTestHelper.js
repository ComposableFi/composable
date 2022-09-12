"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.createMultipleStableSwapPools = exports.createStableSwapPool = exports.createMultipleLBPools = exports.createLBPool = exports.transferTokens = exports.getPoolBalance = exports.getPoolAddress = exports.rpcPriceFor = exports.getPoolInfo = exports.getUserTokens = exports.createMultipleCPPools = exports.swapTokenPairs = exports.removeLiquidityFromPool = exports.sellToPool = exports.buyFromPool = exports.addFundstoThePool = exports.createConsProdPool = void 0;
const polkadotjs_1 = require("@composable/utils/polkadotjs");
/**
 *Contains handler methods for the pabloPallet Tests.
 * StableSwap ConstantProduct and LiquidityBootsrapping Pools
 */
let constantProductk;
let baseAmountTotal;
let quoteAmountTotal;
let mintedLPTokens;
baseAmountTotal = BigInt(0);
quoteAmountTotal = BigInt(0);
mintedLPTokens = BigInt(0);
/**
 * Creates Constant Product Pool
 * @param api
 * @param walletId
 * @param owner
 * @param baseAssetId
 * @param quoteAssetId
 * @param fee
 * @param ownerFee
 */
async function createConsProdPool(api, walletId, owner, baseAssetId, quoteAssetId, fee, baseWeight) {
    const pool = api.createType("PalletPabloPoolInitConfiguration", {
        ConstantProduct: {
            owner: api.createType("AccountId32", owner.address),
            pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
                base: api.createType("u128", baseAssetId),
                quote: api.createType("u128", quoteAssetId)
            }),
            fee: api.createType("Permill", fee),
            baseWeight: api.createType("Permill", baseWeight)
        }
    });
    const { data: [resultPoolId] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId, api.events.pablo.PoolCreated.is, api.tx.pablo.create(pool));
    return resultPoolId.toNumber();
}
exports.createConsProdPool = createConsProdPool;
async function addFundstoThePool(api, poolId, walletId, baseAmount, quoteAmount) {
    const pool = api.createType("u128", poolId);
    const baseAmountParam = api.createType("u128", baseAmount);
    const quoteAmountParam = api.createType("u128", quoteAmount);
    const keepAliveParam = api.createType("bool", true);
    const minMintAmountParam = api.createType("u128", 0);
    const { data: [walletIdResult, addedPool, baseAdded, quoteAdded, returnedLPTokens] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId, api.events.pablo.LiquidityAdded.is, api.tx.pablo.addLiquidity(pool, baseAmountParam, quoteAmountParam, minMintAmountParam, keepAliveParam));
    mintedLPTokens += BigInt(returnedLPTokens.toString(10));
    baseAmountTotal += BigInt(baseAdded.toString(10));
    quoteAmountTotal += BigInt(quoteAdded.toString(10));
    return { walletIdResult, baseAdded, quoteAdded, returnedLPTokens };
}
exports.addFundstoThePool = addFundstoThePool;
async function buyFromPool(api, poolId, walletId, assetId, amountToBuy) {
    const poolIdParam = api.createType("u128", poolId);
    const assetIdParam = api.createType("u128", assetId);
    const amountParam = api.createType("u128", amountToBuy);
    const keepAlive = api.createType("bool", true);
    const minMintAmount = api.createType("u128", 0);
    constantProductk = baseAmountTotal * quoteAmountTotal;
    const expectedConversion = constantProductk / (baseAmountTotal - amountToBuy) - quoteAmountTotal;
    const { data: [retPoolId, accountId, baseArg, quoteArg, baseAmount, quoteAmount, ownerFee] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId, api.events.pablo.Swapped.is, api.tx.pablo.buy(poolIdParam, assetIdParam, amountParam, minMintAmount, keepAlive));
    return { accountId, baseAmount, quoteAmount, expectedConversion, ownerFee: ownerFee.fee };
}
exports.buyFromPool = buyFromPool;
async function sellToPool(api, poolId, walletId, assetId, amount) {
    const poolIdParam = api.createType("u128", poolId);
    const assetIdParam = api.createType("u128", assetId);
    const amountParam = api.createType("u128", amount);
    const minMintAmount = api.createType("u128", 0);
    const keepAliveParam = api.createType("bool", false);
    const { data: [result, ownerAccount, ...rest] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId, api.events.pablo.Swapped.is, api.tx.pablo.sell(poolIdParam, assetIdParam, amountParam, minMintAmount, keepAliveParam));
    return ownerAccount;
}
exports.sellToPool = sellToPool;
async function removeLiquidityFromPool(api, poolId, walletId, lpTokens) {
    const poolIdParam = api.createType("u128", poolId);
    const lpTokenParam = api.createType("u128", lpTokens);
    const minBaseParam = api.createType("u128", 0);
    const minQuoteAmountParam = api.createType("u128", 0);
    const { data: [resultPoolId, resultWallet, resultBase, resultQuote, remainingLpTokens] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId, api.events.pablo.LiquidityRemoved.is, api.tx.pablo.removeLiquidity(poolIdParam, lpTokenParam, minBaseParam, minQuoteAmountParam));
    return { resultBase, resultQuote };
}
exports.removeLiquidityFromPool = removeLiquidityFromPool;
async function swapTokenPairs(api, poolId, wallet, baseAssetId, quoteAssetId, quoteAmount, minReceiveAmount = 0) {
    const poolIdParam = api.createType("u128", poolId);
    const currencyPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("CurrencyId", baseAssetId),
        quote: api.createType("CurrencyId", quoteAssetId)
    });
    const quoteAmountParam = api.createType("u128", quoteAmount);
    const minReceiveParam = api.createType("u128", minReceiveAmount);
    const keepAliveParam = api.createType("bool", true);
    const { data: [resultPoolId, resultWallet, baseAsset, quoteAsset, returnedBaseAmount, returnedQuoteAmount, feeInfo] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, wallet, api.events.pablo.Swapped.is, api.tx.pablo.swap(poolIdParam, currencyPair, quoteAmountParam, minReceiveParam, keepAliveParam));
    return { returnedBaseAmount, returnedQuoteAmount };
}
exports.swapTokenPairs = swapTokenPairs;
async function createMultipleCPPools(api, wallet) {
    const tx = [];
    for (let i = 0; i < 500; i++) {
        const owner = wallet.derive("/test/ConstantProduct/deriveWallet");
        const pool = api.createType("PalletPabloPoolInitConfiguration", {
            ConstantProduct: {
                owner: api.createType("AccountId32", owner.address),
                pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
                    base: api.createType("u128", Math.floor(Math.random() * 10000)),
                    quote: api.createType("u128", Math.floor(Math.random() * 10000))
                }),
                fee: api.createType("Permill", Math.floor(Math.random() * 100000)),
                baseWeight: api.createType("Permill", Math.floor(Math.random() * 100000))
            }
        });
        tx.push(api.tx.pablo.create(pool));
    }
    await (0, polkadotjs_1.sendWithBatchAndWaitForSuccess)(api, wallet, api.events.pablo.PoolCreated.is, tx, false);
}
exports.createMultipleCPPools = createMultipleCPPools;
async function getUserTokens(api, walletId, assetId) {
    const { free } = await api.query.tokens.accounts(walletId.address, assetId);
    return free;
}
exports.getUserTokens = getUserTokens;
async function getPoolInfo(api, poolType, poolId) {
    const result = await api.query.pablo.pools(api.createType("u128", poolId));
    const pool = result.unwrap();
    const poolS = "as" + poolType;
    const lpTokenId = pool[poolS].lpToken.toNumber();
    return { lpTokenId };
}
exports.getPoolInfo = getPoolInfo;
async function rpcPriceFor(api, poolId, baseAssetId, quoteAssetId) {
    return await api.rpc.pablo.pricesFor(poolId, baseAssetId, quoteAssetId, api.createType("CustomRpcBalance", 10000 /* unit */));
}
exports.rpcPriceFor = rpcPriceFor;
async function getPoolAddress(api, poolId, walletId, baseAmount, quoteAmount) {
    const pool = api.createType("u128", poolId);
    const baseAmountParam = api.createType("u128", baseAmount);
    const quoteAmountParam = api.createType("u128", quoteAmount);
    const keepAliveParam = api.createType("bool", true);
    const minMintAmountParam = api.createType("u128", 0);
    const { data: [, AccountId] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, walletId, api.events.tokens.Endowed.is, api.tx.pablo.addLiquidity(pool, baseAmountParam, quoteAmountParam, minMintAmountParam, keepAliveParam));
    return AccountId.toString();
}
exports.getPoolAddress = getPoolAddress;
async function getPoolBalance(api, poolAddress, assetId) {
    const { free } = await api.query.tokens.accounts(poolAddress, assetId);
    return free;
}
exports.getPoolBalance = getPoolBalance;
async function transferTokens(api, sender, receiver, assetId, amount) {
    const { data: [, accountId] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sender, api.events.tokens.Endowed.is, api.tx.assets.transfer(api.createType("u128", assetId), api.createType("MultiAddress", {
        id: api.createType("AccountId", receiver.address.toString())
    }), api.createType("u128", amount), api.createType("bool", false)));
    return accountId.toString();
}
exports.transferTokens = transferTokens;
/***
 * Creates LiquidityBootstrappingPool
 * @param sender
 * @param owner
 * @param baseAssetId
 * @param quoteAssetId
 * @param start
 * @param end
 * @param initialWeight
 * @param finalWeight
 * @param fee
 * @returns Newly Created pool Id
 */
async function createLBPool(api, sender, owner, baseAssetId, quoteAssetId, start, end, initialWeight, finalWeight, feeRate, ownerFeeRate, protocolFeeRate) {
    const pool = api.createType("PalletPabloPoolInitConfiguration", {
        LiquidityBootstrapping: {
            owner: api.createType("AccountId32", owner.address),
            pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
                base: api.createType("u128", baseAssetId),
                quote: api.createType("u128", quoteAssetId)
            }),
            sale: api.createType("ComposableTraitsDexSale", {
                start: api.createType("u32", start),
                end: api.createType("u32", end),
                initialWeight: api.createType("Permill", initialWeight),
                finalWeight: api.createType("Permill", finalWeight)
            }),
            feeConfig: api.createType("ComposableTraitsDexFeeConfig", {
                feeRate: api.createType("Permill", feeRate),
                ownerFeeRate: api.createType("Permill", ownerFeeRate),
                protocolFeeRate: api.createType("Permill", protocolFeeRate)
            })
        }
    });
    const { data: [returnedPoolId] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sender, api.events.pablo.PoolCreated.is, api.tx.pablo.create(pool));
    const resultPoolId = returnedPoolId.toNumber();
    return { resultPoolId };
}
exports.createLBPool = createLBPool;
async function createMultipleLBPools(api, wallet) {
    const tx = [];
    for (let i = 0; i < 500; i++) {
        const owner = wallet.derive("/test/ConstantProduct/deriveWallet");
        const pool = api.createType("PalletPabloPoolInitConfiguration", {
            LiquidityBootstrapping: {
                owner: api.createType("AccountId32", owner.address),
                pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
                    base: api.createType("u128", Math.floor(Math.random() * 10000)),
                    quote: api.createType("u128", Math.floor(Math.random() * 10000))
                }),
                sale: api.createType("ComposableTraitsDexSale", {
                    start: api.createType("u32", Math.floor(Math.random() * 50000) + 300),
                    end: api.createType("u32", Math.floor(Math.random() * 100000) + 100000),
                    initialWeight: api.createType("Permill", Math.floor(Math.random() * 800000) + 150000),
                    finalWeight: api.createType("Permill", Math.floor(Math.random() * 100000) + 50000)
                }),
                feeConfig: api.createType("ComposableTraitsDexFeeConfig", {
                    feeRate: api.createType("Permill", Math.floor(Math.random() * 150000)),
                    ownerFeeRate: api.createType("Permill", Math.floor(Math.random() * 150000)),
                    protocolFeeRate: api.createType("Permill", Math.floor(Math.random() * 150000))
                })
            }
        });
        tx.push(api.tx.pablo.create(pool));
    }
    await (0, polkadotjs_1.sendWithBatchAndWaitForSuccess)(api, wallet, api.events.pablo.PoolCreated.is, tx, false);
}
exports.createMultipleLBPools = createMultipleLBPools;
/***
 Creates stableSwapPool
 @param sender: User sending tx- KeyringPair
 @param owner: Owner of the pool - KeyringPair
 @param baseAssetId: CurencyId
 @param quoteAssetId: CurrencyId
 @param ampCoefficient: Amplification Coefficient, for details see curve.fi stable swap dex
 @param fee: Total fee to be charged for each transaction in the pool
 @returns resultPoolId: the number of the created pool
 */
async function createStableSwapPool(api, sender, owner, baseAssetId, quoteAssetId, ampCoefficient, fee) {
    const pool = api.createType("PalletPabloPoolInitConfiguration", {
        StableSwap: {
            owner: api.createType("AccountId32", owner.address),
            pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
                base: api.createType("u128", baseAssetId),
                quote: api.createType("u128", quoteAssetId)
            }),
            amplification_coefficient: api.createType("u16", ampCoefficient),
            fee: api.createType("Permill", fee)
        }
    });
    const { data: [returnedPoolId] } = await (0, polkadotjs_1.sendAndWaitForSuccess)(api, sender, api.events.pablo.PoolCreated.is, api.tx.pablo.create(pool));
    const resultPoolId = returnedPoolId.toNumber();
    return { resultPoolId };
}
exports.createStableSwapPool = createStableSwapPool;
async function createMultipleStableSwapPools(api, wallet) {
    const tx = [];
    for (let i = 0; i < 50; i++) {
        const owner = wallet.derive("/test/ConstantProduct/deriveWallet");
        const pool = api.createType("PalletPabloPoolInitConfiguration", {
            StableSwap: {
                owner: api.createType("AccountId32", owner.address),
                pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
                    base: api.createType("u128", Math.floor(Math.random() * 10000)),
                    quote: api.createType("u128", Math.floor(Math.random() * 10000))
                }),
                amplification_coefficient: api.createType("u16", Math.floor(Math.random() * 20000)),
                fee: api.createType("Permill", Math.floor(Math.random() * 990000))
            }
        });
        tx.push(api.tx.pablo.create(pool));
    }
    await (0, polkadotjs_1.sendWithBatchAndWaitForSuccess)(api, wallet, api.events.pablo.PoolCreated.is, tx, false);
}
exports.createMultipleStableSwapPools = createMultipleStableSwapPools;
//# sourceMappingURL=pabloTestHelper.js.map