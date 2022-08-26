export default {
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
