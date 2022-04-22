export default {
  rpc: {},
  types: {
    PalletPabloPoolInitConfiguration: "PalletPabloPoolConfiguration",
    PalletPabloPoolConfiguration: {
      _enum: {
        StableSwap: "StableSwap",
        ConstantProduct: "ConstantProduct",
        LiquidityBootstrapping: "LiquidityBootstrapping"
      }
    },
    PalletPabloPriceCumulative: "Null",
    PalletPabloTimeWeightedAveragePrice: "Null",
    ConstantProduct: {
      owner: "AccountId32",
      pair: "ComposableTraitsDefiCurrencyPairCurrencyId",
      fee: "Permill",
      ownerFee: "Permill"
    },
    StableSwap: "Null",
    LiquidityBootstrapping: "Null"
  },
};
