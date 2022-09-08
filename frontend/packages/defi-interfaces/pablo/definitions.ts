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
          isOptional: true
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
          type: "BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        }
      ],
      type: "CustomRpcBalance"
    },
    simulateRemoveLiquidity: {
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
          name: "lpAmount",
          type: "CustomRpcBalance"
        },
        {
          name: "minExpectedAmounts",
          type: "BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        }
      ],
      type: "RemoveLiquiditySimulationResult"
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
          fee: "Permill"
        },
        ConstantProduct: {
          owner: "AccountId32",
          pair: "ComposableTraitsDefiCurrencyPairCurrencyId",
          fee: "Permill",
          baseWeight: "Permill"
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
          feeConfig: {
            feeRate: "Permill",
            ownerFeeRate: "Permill",
            protocolFeeRate: "Permill"
          }
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
      spotPrice: "CustomRpcBalance"
    },
    ComposableTraitsDexFee: {
      fee: "u128",
      lp_fee: "u128",
      owner_fee: "u128",
      protocol_fee: "u128",
      asset_id: "u128"
    },
    RemoveLiquiditySimulationResult: {
      assets: "BTreeMap<SafeRpcWrapper<AssetId>, SafeRpcWrapper<Balance>>"
    }
  }
};
