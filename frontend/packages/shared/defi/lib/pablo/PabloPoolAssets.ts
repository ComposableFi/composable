import { Asset, fromPermill } from "shared";
import BigNumber from "bignumber.js";

type PoolAssets = Record<string, BigNumber>;

export class PabloPoolAssets {
  readonly assets: PoolAssets;

  static fromJSON(assets: any): PabloPoolAssets {
    try {
      const poolAssets: PoolAssets = Object.entries(assets).reduce(
        (agg, [assetId, permillWeight]) => {
          agg[assetId] = fromPermill(permillWeight as string);
          return agg;
        },
        {} as PoolAssets
      );

      return new PabloPoolAssets(poolAssets);
    } catch (err: any) {
      console.error("[PabloPoolPair] ", err.message);
      throw new Error(err.message);
    }
  }

  constructor(assets: PoolAssets) {
    this.assets = assets;
  }

  toJSON(): PoolAssets {
    return this.assets;
  }

  getAsset(assetId: string): PoolAssets {
    if (this.assets[assetId]) {
      return { [assetId]: this.assets[assetId] };
    }
    throw new Error("Asset not part of the pool");
  }

  intoAssets(assets: Asset[]): Asset[] {
    return assets.filter(
      (asset) => (asset.getPicassoAssetId() as string) in this.assets
    );
  }
}
