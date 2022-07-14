export default {
  rpc: {
    pricesFor: {
      description: "Get the price(in quote asset) for the given asset pair in the given pool for the given amount",
      params: [
        {
          name: "poolId",
          type: "PalletPabloPoolId"
        },
        {
          name: "baseAssetId",
          type: "CustomRpcCurrencyId"
        },
        {
          name: "quoteAssetId",
          type: "CustomRpcCurrencyId"
        },
        {
          name: "amount",
          type: "CustomRpcBalance"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        }
      ],
      type: "PalletPabloPriceAggregate"
    },
    simulateAddLiquidity: {
      description: "Get the price(in quote asset) for the given asset pair in the given pool for the given amount",
      params: [
        {
          name: "who",
          type: "AccountId32"
        },
        {
          name: "poolId",
          type: "PalletPabloPoolId"
        },
        {
          name: "amounts",
          type: "BTreeMap<AssetId, Balance>"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        }
      ]
    },
  },
  types: {
    PalletPabloPoolInitConfiguration: "PalletPabloPoolConfiguration",
    PalletPabloPoolConfiguration: {
      _enum: {
        StableSwap: {
          owner: "AccountId32",
          pair: "ComposableTraitsDefiCurrencyPairCurrencyId",
          amplification_coefficient: "u16",
          fee: "Permill",
          ownerFee: "Permill"
        },
        ConstantProduct: {
          owner: "AccountId32",
          pair: "ComposableTraitsDefiCurrencyPairCurrencyId",
          fee: "Permill",
          ownerFee: "Permill"
        },
        LiquidityBootstrapping: {
          owner: "AccountId32",
          pair: "ComposableTraitsDefiCurrencyPairCurrencyId",
          sale: {
            start: "BlockNumber",
            end: "BlockNumber",
            initial_weight: "Permill",
            final_weight: "Permill"
          },
          fee: "Permill",
        }
      }
    },
    PalletPabloPriceCumulative: "Null",
    PalletPabloTimeWeightedAveragePrice: "Null",
    PalletPabloPoolId: "SafeRpcWrapper",
    PalletPabloPriceAggregate: {
      poolId: "PalletPabloPoolId",
      baseAssetId: "CustomRpcCurrencyId",
      quoteAssetId: "CustomRpcCurrencyId",
      spotPrice: "CustomRpcBalance",
    },
  },
};
