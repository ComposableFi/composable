declare const _default: {
    rpc: {
        pricesFor: {
            description: string;
            params: ({
                name: string;
                type: string;
                isOptional?: undefined;
            } | {
                name: string;
                type: string;
                isOptional: boolean;
            })[];
            type: string;
        };
    };
    types: {
        PalletPabloPoolInitConfiguration: string;
        PalletPabloPoolConfiguration: {
            _enum: {
                StableSwap: {
                    owner: string;
                    pair: string;
                    amplification_coefficient: string;
                    fee: string;
                };
                ConstantProduct: {
                    owner: string;
                    pair: string;
                    fee: string;
                    baseWeight: string;
                };
                LiquidityBootstrapping: {
                    owner: string;
                    pair: string;
                    sale: {
                        start: string;
                        end: string;
                        initial_weight: string;
                        final_weight: string;
                    };
                    feeConfig: {
                        feeRate: string;
                        ownerFeeRate: string;
                        protocolFeeRate: string;
                    };
                };
            };
        };
        PalletPabloPriceCumulative: string;
        PalletPabloTimeWeightedAveragePrice: string;
        PalletPabloPoolId: string;
        PalletPabloPriceAggregate: {
            poolId: string;
            baseAssetId: string;
            quoteAssetId: string;
            spotPrice: string;
        };
        ComposableTraitsDexFee: {
            fee: string;
            lp_fee: string;
            owner_fee: string;
            protocol_fee: string;
            asset_id: string;
        };
    };
};
export default _default;
