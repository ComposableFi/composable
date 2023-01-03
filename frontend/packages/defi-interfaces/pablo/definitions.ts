export default {
  rpc: {
    pricesFor: {
      description:
        "Get the price(in quote asset) for the given asset pair in the given pool for the given amount",
      params: [
        {
          name: "poolId",
          type: "PalletPabloPoolId",
        },
        {
          name: "baseAssetId",
          type: "CustomRpcCurrencyId",
        },
        {
          name: "quoteAssetId",
          type: "CustomRpcCurrencyId",
        },
        {
          name: "amount",
          type: "CustomRpcBalance",
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "PalletPabloPriceAggregate",
    },
    simulateAddLiquidity: {
      description: "Simulate add liquidity",
      params: [
        {
          name: "who",
          type: "SafeRpcWrapper<AccountId>",
        },
        {
          name: "poolId",
          type: "SafeRpcWrapper<PoolId>",
        },
        {
          name: "amounts",
          type: "BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>",
        },
      ],
      type: "SafeRpcWrapper<Balance>",
    },
    simulateRemoveLiquidity: {
      description:
        "Get the price(in quote asset) for the given asset pair in the given pool for the given amount",
      params: [
        {
          name: "who",
          type: "SafeRpcWrapper<AccountId>",
        },
        {
          name: "poolId",
          type: "SafeRpcWrapper<PoolId>",
        },
        {
          name: "lpAmount",
          type: "SafeRpcWrapper<Balance>",
        },
        {
          name: "minExpectedAmounts",
          type: "BTreeMap<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>",
        },
      ],
      type: "RemoveLiquiditySimulationResult<SafeRpcWrapper<CurrencyId>, SafeRpcWrapper<Balance>>",
    },
  },
  types: {
    PalletPabloPoolInitConfiguration: "PalletPabloPoolConfiguration",
    PalletPabloPoolConfiguration: {
      _enum: {
        DualAssetConstantProduct: {
          owner: "AccountId32",
          assetsWeights: "BoundedBTreeMap<T::AssetId, Permill, ConstU32<2>>",
          lpToken: "u128",
          feeConfig: {
            feeRate: "Permill",
            ownerFeeRate: "Permill",
            protocolFeeRate: "Permill",
          },
        },
      },
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
    ComposableTraitsDexFee: {
      fee: "u128",
      lpFee: "u128",
      ownerFee: "u128",
      protocolFee: "u128",
      assetId: "u128",
    },
    ComposableTraitsDexStakingRewardPool: "Null",
    RemoveLiquiditySimulationResult: "BTreeMap<AssetId, Balance>",
  },
};
