"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.default = {
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
        }
    }
};
//# sourceMappingURL=definitions.js.map