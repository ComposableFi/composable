export const DAY_IN_MS = 24 * 60 * 60 * 1000;

export type AssetId = "1" | "4" | "130";
export type CoingeckoIds = "kusama" | "tether";

export type AssetInfo = {
  assetId: AssetId;
  coingeckoId?: CoingeckoIds;
  // Asset used for deriving prices from the spot price
  spotPriceBaseAsset?: {
    assetId: AssetId; // assetId used to calculate spot prices
    poolId: string; // poolId to be used to calculate spot prices
    coingeckoId: CoingeckoIds; // coingeckoId of the base asset
  };
};

export const assetList: Array<AssetInfo> = [
  {
    assetId: "1",
    spotPriceBaseAsset: {
      assetId: "4",
      coingeckoId: "kusama",
      poolId: "2"
    }
  },
  {
    assetId: "4",
    coingeckoId: "kusama"
  },
  {
    assetId: "130",
    coingeckoId: "tether"
  }
];

type Prices = { usd: number };

export type CoingeckoPrices = {
  [Property in CoingeckoIds]: Prices;
};
