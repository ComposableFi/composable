import { ApiPromise } from "@polkadot/api";

export function createConstantProductPool(
  api: ApiPromise,
  pair: {
    base: number;
    quote: number;
  },
  feeRate: number,
  walletAddress: string,
  baseWeight: number = 50 * 10000
) {
  let pool = api.createType("PalletPabloPoolInitConfiguration", {
    ConstantProduct: {
      owner: api.createType("AccountId32", walletAddress),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", pair.base),
        quote: api.createType("u128", pair.quote),
      }),
      fee: api.createType("Permill", feeRate),
      baseWeight: api.createType("Permill", baseWeight),
    },
  });

  return api.tx.pablo.create(pool as any);
}

export function createStableSwapPool(
  api: ApiPromise,
  pair: {
    base: number;
    quote: number;
  },
  feeRate: number,
  walletAddress: string,
  amplificationCoefficient: number = 10000 // default as 1
) {
  let pool = api.createType("PalletPabloPoolInitConfiguration", {
    StableSwap: {
      owner: api.createType("AccountId32", walletAddress),
      pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
        base: api.createType("u128", pair.base),
        quote: api.createType("u128", pair.quote),
      }),
      amplificationCoefficient: api.createType("u16", amplificationCoefficient),
      fee: api.createType("Permill", feeRate),
    },
  });

  return api.tx.pablo.create(pool as any);
}
