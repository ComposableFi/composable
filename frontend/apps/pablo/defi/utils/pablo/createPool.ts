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