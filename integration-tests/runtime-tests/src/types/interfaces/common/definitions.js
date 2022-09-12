"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.default = {
    rpc: {},
    types: {
        SafeRpcWrapper: "String",
        CustomRpcCurrencyId: "SafeRpcWrapper",
        CustomRpcBalance: "SafeRpcWrapper",
        CurrencyId: "u128",
        AssetsBalance: "u128",
        ComposableTraitsDefiSellCurrencyId: "CurrencyId",
        ComposableTraitsDefiCurrencyPairCurrencyId: {
            base: "CurrencyId",
            quote: "CurrencyId"
        },
        ComposableTraitsXcmCumulusMethodId: "Null",
        ComposableTraitsXcmXcmSellRequest: "Null"
    }
};
//# sourceMappingURL=definitions.js.map