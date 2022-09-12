import { KeyringPair } from "@polkadot/keyring/types";
import { u128 } from "@polkadot/types-codec";
import { AccountId32 } from "@polkadot/types/interfaces/runtime";
import { CustomRpcCurrencyId, PalletPabloPoolId } from "@composable/types/interfaces";
import { ApiPromise } from "@polkadot/api";
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
export declare function createConsProdPool(api: ApiPromise, walletId: KeyringPair, owner: KeyringPair, baseAssetId: number, quoteAssetId: number, fee: number, baseWeight: number): Promise<number>;
export declare function addFundstoThePool(api: ApiPromise, poolId: number, walletId: KeyringPair, baseAmount: bigint, quoteAmount: bigint): Promise<{
    returnedLPTokens: u128;
    baseAdded: u128;
    quoteAdded: u128;
    walletIdResult: AccountId32;
}>;
export declare function buyFromPool(api: ApiPromise, poolId: number, walletId: KeyringPair, assetId: number, amountToBuy: bigint): Promise<{
    accountId: AccountId32;
    ownerFee: u128;
    expectedConversion: bigint;
    quoteAmount: u128;
    baseAmount: u128;
}>;
export declare function sellToPool(api: ApiPromise, poolId: number, walletId: KeyringPair, assetId: number, amount: bigint): Promise<AccountId32>;
export declare function removeLiquidityFromPool(api: ApiPromise, poolId: number, walletId: KeyringPair, lpTokens: bigint): Promise<{
    resultBase: u128;
    resultQuote: u128;
}>;
export declare function swapTokenPairs(api: ApiPromise, poolId: number, wallet: KeyringPair, baseAssetId: number, quoteAssetId: number, quoteAmount: bigint, minReceiveAmount?: number): Promise<{
    returnedBaseAmount: u128;
    returnedQuoteAmount: u128;
}>;
export declare function createMultipleCPPools(api: ApiPromise, wallet: KeyringPair): Promise<void>;
export declare function getUserTokens(api: ApiPromise, walletId: KeyringPair, assetId: number): Promise<u128>;
export declare function getPoolInfo(api: ApiPromise, poolType: string, poolId: number): Promise<{
    lpTokenId: number;
}>;
export declare function rpcPriceFor(api: ApiPromise, poolId: PalletPabloPoolId, baseAssetId: CustomRpcCurrencyId, quoteAssetId: CustomRpcCurrencyId): Promise<import("@composable/types/interfaces").PalletPabloPriceAggregate>;
export declare function getPoolAddress(api: ApiPromise, poolId: number, walletId: KeyringPair, baseAmount: bigint, quoteAmount: bigint): Promise<string>;
export declare function getPoolBalance(api: ApiPromise, poolAddress: string, assetId: number): Promise<u128>;
export declare function transferTokens(api: ApiPromise, sender: KeyringPair, receiver: KeyringPair, assetId: number, amount: bigint): Promise<string>;
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
export declare function createLBPool(api: ApiPromise, sender: KeyringPair, owner: KeyringPair, baseAssetId: number, quoteAssetId: number, start: number, end: number, initialWeight: number, finalWeight: number, feeRate: number, ownerFeeRate: number, protocolFeeRate: number): Promise<{
    resultPoolId: number;
}>;
export declare function createMultipleLBPools(api: ApiPromise, wallet: KeyringPair): Promise<void>;
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
export declare function createStableSwapPool(api: ApiPromise, sender: KeyringPair, owner: KeyringPair, baseAssetId: number, quoteAssetId: number, ampCoefficient: number, fee: number): Promise<{
    resultPoolId: number;
}>;
export declare function createMultipleStableSwapPools(api: ApiPromise, wallet: KeyringPair): Promise<void>;
