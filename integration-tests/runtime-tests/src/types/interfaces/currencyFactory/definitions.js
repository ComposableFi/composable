"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.default = {
    rpc: {},
    types: {
        PalletCurrencyFactoryRanges: {
            ranges: "BoundedVec<PalletCurrencyFactoryRangesRange<AssetId>, MaxRanges>"
        },
        PalletCurrencyFactoryRangesRange: {
            current: "AssetId",
            end: "AssetId"
        }
    }
};
//# sourceMappingURL=definitions.js.map