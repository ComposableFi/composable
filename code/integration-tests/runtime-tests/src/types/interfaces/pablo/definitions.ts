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
      description: "Simulate removal liquidity",
      params: [
        {
          name: "who",
          type: "SafeRpcWrapper<AccountId>"
        },
        {
          name: "poolId",
          type: "SafeRpcWrapper<PoolId>"
        },
        {
          name: "amounts",
          type: "BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>"
        }
      ],
      type: "SafeRpcWrapper<Balance>"
    },
    simulateRemoveLiquidity: {
      description: "Get the price(in quote asset) for the given asset pair in the given pool for the given amount",
      params: [
        {
          name: "who",
          type: "SafeRpcWrapper<AccountId>"
        },
        {
          name: "poolId",
          type: "SafeRpcWrapper<PoolId>"
        },
        {
          name: "lpAmount",
          type: "SafeRpcWrapper<Balance>"
        },
        {
          name: "minExpectedAmounts",
          type: "BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>"
        }
      ],
      type: "RemoveLiquiditySimulationResult<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>"
    }
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
    ComposableTraitsDexStakingRewardPool: "Null",
    RemoveLiquiditySimulationResult: {
      assets: "BTreeMap<AssetId, Balance>"
    }
  }
};
