import { sendAndWaitForSuccess } from '@composable/utils/polkadotjs';
import {KeyringPair} from "@polkadot/keyring/types";
import { u128 } from '@polkadot/types-codec';

/**
 *Contains handler methods for the constantProductDex Tests. 
 */
let poolId: number;  
let constantProductk: number;
let baseAmountTotal: number;
let quoteAmountTotal: number;
let mintedLPTokens: number;
baseAmountTotal = 0;
quoteAmountTotal = 0;
mintedLPTokens = 0;

export async function createPool(walletId: KeyringPair, baseAssetId: number, quoteAssetId: number, ownerFee: number){
  const pair = api.createType('ComposableTraitsDefiCurrencyPair', {
    base: api.createType('u128', baseAssetId),
    quote: api.createType('u128', quoteAssetId)
  });
  const fee = api.createType('Permill', 0);
  const ownerFees = api.createType('Permill', ownerFee);
  const {data: [resultPoolId],} = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.constantProductDex.PoolCreated.is,
    api.tx.constantProductDex.create(pair, fee, ownerFees)
  );
  poolId = resultPoolId.toNumber();
  return poolId;
}
export async function addFundstoThePool(walletId:KeyringPair, baseAmount:number, quoteAmount:number){
  const baseAmountParam = api.createType('u128', baseAmount);
  const quoteAmountParam = api.createType('u128', quoteAmount);
  const keepAliveParam = api.createType('bool', true);
  const minMintAmountParam = api.createType('u128', 0);
  const {data: [,walletIdResult,baseAdded, quoteAdded,returnedLPTokens]} =await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.constantProductDex.LiquidityAdded.is,
    api.tx.constantProductDex.addLiquidity(poolId, 
      baseAmountParam, 
      quoteAmountParam, 
      minMintAmountParam, 
      keepAliveParam
    )
  );
  mintedLPTokens += returnedLPTokens.toNumber();
  baseAmountTotal += baseAdded.toNumber();
  quoteAmountTotal += quoteAdded.toNumber();
  return {walletIdResult, baseAdded, quoteAdded, returnedLPTokens};
}

export async function buyFromPool(walletId: KeyringPair, assetId:number, amountToBuy: number){
  const poolIdParam = api.createType('u128', poolId);
  const assetIdParam = api.createType('u128', assetId);
  const amountParam = api.createType('u128', amountToBuy);
  const keepAlive = api.createType('bool', true);
  constantProductk = baseAmountTotal*quoteAmountTotal;
  let expectedConversion = Math.floor((constantProductk/(baseAmountTotal-amountToBuy)))-quoteAmountTotal;
  const {data: [accountId,poolArg,quoteArg,swapArg,amountgathered,quoteAmount,ownerFee] } = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.constantProductDex.Swapped.is,
    api.tx.constantProductDex.buy(
      poolIdParam,
      assetIdParam,
      amountParam,
      keepAlive
    )
  );
  return {accountId, quoteAmount, expectedConversion, ownerFee};
}

export async function sellToPool(walletId: KeyringPair, assetId: number, amount:number){
  const poolIdParam = api.createType('u128', poolId);
  const assetIdParam = api.createType('u128', assetId);
  const amountParam = api.createType('u128', amount);
  const keepAliveParam = api.createType('bool', false);
  const {data: [result, ...rest]} = await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.constantProductDex.Swapped.is,
    api.tx.constantProductDex.sell(
      poolIdParam,
      assetIdParam,
      amountParam,
      keepAliveParam
    )
  )
  return result.toString();        
}

export async function removeLiquidityFromPool(walletId: KeyringPair, lpTokens:number){
  const expectedLPTokens = mintedLPTokens-lpTokens;
  const poolIdParam = api.createType('u128', poolId);
  const lpTokenParam = api.createType('u128', lpTokens);
  const minBaseParam = api.createType('u128', 0);
  const minQuoteAmountParam = api.createType('u128', 0);
  const {data: [resultPoolId,resultWallet,resultBase,resultQuote,remainingLpTokens]}=await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.constantProductDex.LiquidityRemoved.is,
    api.tx.constantProductDex.removeLiquidity(
      poolIdParam,
      lpTokenParam,
      minBaseParam,
      minQuoteAmountParam
    )
  );   
  return {remainingLpTokens, expectedLPTokens}
}

export async function swapTokenPairs(wallet: KeyringPair, 
  baseAssetId: number,
  quoteAssetId:number,
  quoteAmount: number,
  minReceiveAmount: number = 0
  ){
    const poolIdParam = api.createType('u128', poolId);
    const currencyPair = api.createType('ComposableTraitsDefiCurrencyPair', {
      base: api.createType('u128', baseAssetId),
      quote: api.createType('u128',quoteAssetId)
    });
    const quoteAmountParam = api.createType('u128', quoteAmount);
    const minReceiveParam = api.createType('u128', minReceiveAmount);
    const keepAliveParam = api.createType('bool', true);
    const {data: [resultPoolId,resultWallet,resultQuote,resultBase,resultBaseAmount,returnedQuoteAmount,]}= await sendAndWaitForSuccess(
      api,
      wallet,
      api.events.constantProductDex.Swapped.is,
      api.tx.constantProductDex.swap(
        poolIdParam,
        currencyPair,
        quoteAmountParam,
        minReceiveParam,
        keepAliveParam
      )
    );
    return {returnedQuoteAmount};
}

export async function getUserTokens(walletId: KeyringPair, assetId: number){
  const {free, reserved, frozen} = await api.query.tokens.accounts(walletId.address, assetId); 
  return free.toNumber();
}

export async function getOwnerFee(poolId: number){
  const result = await api.query.constantProductDex.pools(api.createType('u128', poolId));
  return result.unwrap().ownerFee.toNumber();
}