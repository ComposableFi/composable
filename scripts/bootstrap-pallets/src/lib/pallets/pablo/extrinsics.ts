import { ComposableTraitsDefiCurrencyPairCurrencyId } from "@composable/common";
import { PalletPabloPoolInitConfiguration } from "@composable/pablo";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import BigNumber from "bignumber.js";
import { sendAndWaitForSuccess, sendAndWaitFor } from "@bootstrap-pallets/lib";

export async function createLiquidityBootstrappingPool(
  api: ApiPromise,
  ownerKey: KeyringPair,
  config: PalletPabloPoolInitConfiguration
) {
  return await sendAndWaitForSuccess(api, ownerKey, api.events.pablo.PoolCreated.is, api.tx.pablo.create(config));
}

export async function updateDexRoute(
  api: ApiPromise,
  walletSudo: KeyringPair,
  pair: ComposableTraitsDefiCurrencyPairCurrencyId,
  poolId: number
) {
  let dexRoute = [poolId];
  return await sendAndWaitForSuccess(
    api,
    walletSudo,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.dexRouter.updateRoute(pair, dexRoute))
  );
}

export async function createConstantProductPool(
  api: ApiPromise,
  sudoKey: KeyringPair,
  config: PalletPabloPoolInitConfiguration
) {
  return await sendAndWaitForSuccess(api, sudoKey, api.events.pablo.PoolCreated.is, api.tx.pablo.create(config));
}

export async function createStableSwapPool(
  api: ApiPromise,
  sudoKey: KeyringPair,
  baseAssetId: number,
  quoteAssetId: number,
  amplificationCoefficient: number,
  feeRate: number
) {
  const pool = api.createType("PalletPabloPoolInitConfiguration", {
    StableSwap: {
      owner: api.createType("AccountId32", sudoKey.publicKey),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", baseAssetId),
        quote: api.createType("u128", quoteAssetId)
      }),
      amplificationCoefficient: api.createType("u16", amplificationCoefficient),
      fee: api.createType("Permill", feeRate)
    }
  });
  return await sendAndWaitForSuccess(api, sudoKey, api.events.pablo.PoolCreated.is, api.tx.pablo.create(pool));
}

export async function addLiquidity(
  api: ApiPromise,
  wallet: KeyringPair,
  poolId: BigNumber,
  baseAmount: string,
  quoteAmount: string
) {
  const baseAmountParam = api.createType("u128", baseAmount);
  const quoteAmountParam = api.createType("u128", quoteAmount);
  const keepAliveParam = api.createType("bool", false);
  return await sendAndWaitFor(
    api,
    wallet,
    api.events.pablo.LiquidityAdded.is,
    api.tx.pablo.addLiquidity(
      poolId.toString(),
      baseAmountParam,
      quoteAmountParam,
      0, // min mint amount
      keepAliveParam
    )
  );
}

export async function buyFromPool(
  api: ApiPromise,
  walletId: KeyringPair,
  poolId: number,
  assetId: number,
  amountToBuy: number
) {
  const poolIdParam = api.createType("u128", poolId);
  const assetIdParam = api.createType("u128", assetId);
  const amountParam = api.createType("u128", amountToBuy);
  const keepAlive = api.createType("bool", true);
  return await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.pablo.Swapped.is,
    api.tx.liquidityBootstrapping.buy(
      poolIdParam,
      assetIdParam,
      amountParam, // convert amount to buy from price
      keepAlive
    )
  );
}

export async function sellToPool(
  api: ApiPromise,
  walletId: KeyringPair,
  poolId: number,
  assetId: number,
  amount: number
) {
  const poolIdParam = api.createType("u128", poolId);
  const assetIdParam = api.createType("u128", assetId);
  const amountParam = api.createType("u128", amount);
  const keepAliveParam = api.createType("bool", false);
  return await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.pablo.Swapped.is,
    api.tx.liquidityBootstrapping.sell(poolIdParam, assetIdParam, amountParam, keepAliveParam)
  );
}

export async function removeLiquidityFromPool(api: ApiPromise, walletId: KeyringPair, poolId: number) {
  const poolIdParam = api.createType("u128", poolId);
  return await sendAndWaitForSuccess(
    api,
    walletId,
    api.events.liquidityBootstrapping.PoolDeleted.is, // Doesn't Exist!
    api.tx.liquidityBootstrapping.removeLiquidity(poolIdParam)
  );
}

export async function swapTokenPairs(
  api: ApiPromise,
  wallet: KeyringPair,
  poolId: number,
  baseAssetId: number,
  quoteAssetId: number,
  quoteAmount: number,
  minReceiveAmount = 0
) {
  const poolIdParam = api.createType("u128", poolId);
  const currencyPair = api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
    base: api.createType("u128", baseAssetId),
    quote: api.createType("u128", quoteAssetId)
  });
  const quoteAmountParam = api.createType("u128", quoteAmount);
  const minReceiveParam = api.createType("u128", minReceiveAmount);
  const keepAliveParam = api.createType("bool", true);

  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.pablo.Swapped.is,
    api.tx.pablo.swap(poolIdParam, currencyPair as any, quoteAmountParam, minReceiveParam, keepAliveParam)
  );
}

export async function enableTwap(api: ApiPromise, walletSudo: KeyringPair, poolId: number) {
  const poolIdParam = api.createType("u128", poolId);

  return await sendAndWaitForSuccess(
    api,
    walletSudo,
    api.events.sudo.Sudid.is,
    api.tx.sudo.sudo(api.tx.pablo.enableTwap(poolIdParam))
  );
}
