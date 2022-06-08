import { ApiPromise } from "@polkadot/api";

export const createConstantProductPool = (
  api: ApiPromise,
  pair: {
    base: number;
    quote: number;
  },
  fee: {
    fee: number;
    ownerFee: number;
  },
  walletAddress: string
) => {
    let pool = api.createType("PalletPabloPoolInitConfiguration", {
        ConstantProduct: {
          owner: api.createType("AccountId32", walletAddress),
          pair: api.createType("ComposableTraitsDefiCurrencyPairCurrencyId", {
            base: api.createType("u128", pair.base),
            quote: api.createType("u128", pair.quote),
          }),
          fee: api.createType("Permill", fee.fee),
          ownerFee: api.createType("Permill", fee.ownerFee),
        },
      })

  return api.tx.pablo.create(pool as any);
};
