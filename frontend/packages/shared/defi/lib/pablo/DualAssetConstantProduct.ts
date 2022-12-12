import { Asset, humanizedBnToBn } from "shared";
import { ApiPromise } from "@polkadot/api";
import { BasePabloPool } from "./BasePabloPool";
import { PabloPoolFeeConfig } from "./PabloPoolFeeConfig";
import { PabloPoolAssets } from "./PabloPoolAssets";
import { LiquidityProviderToken } from "../LiquidityProviderToken";
import BigNumber from "bignumber.js";

export class DualAssetConstantProduct extends BasePabloPool {
  protected readonly __lpToken: LiquidityProviderToken;

  static fromJSON(
    poolIndex: BigNumber,
    api: ApiPromise,
    supportedAssets: Asset[],
    constantProductPoolJSON: any
  ): DualAssetConstantProduct {
    try {
      const lpTokenAssetId = humanizedBnToBn(constantProductPoolJSON.lpToken);
      const assets = PabloPoolAssets.fromJSON(constantProductPoolJSON.assetsWeights);
      const underlyingAssets = assets.intoAssets(supportedAssets);
      const lpToken = new LiquidityProviderToken(underlyingAssets, lpTokenAssetId, api);

      return new DualAssetConstantProduct(
        api,
        poolIndex,
        assets,
        PabloPoolFeeConfig.fromJSON(constantProductPoolJSON.feeConfig),
        constantProductPoolJSON.owner,
        lpToken
      );
    } catch (err: any) {
      console.error("[ConstantProductPool] ", err);
      throw new Error(err.message);
    }
  }

  constructor(
    api: ApiPromise,
    poolId: BigNumber,
    assets: PabloPoolAssets,
    feeConfig: PabloPoolFeeConfig,
    owner: string,
    lpToken: LiquidityProviderToken
  ) {
    super(api, poolId, assets, feeConfig, owner);
    this.__lpToken = lpToken;
  }

  getLiquidityProviderToken(): LiquidityProviderToken {
    return this.__lpToken;
  }
  /**
   * Calculate Spot Price
   * Uses math from: 
   * https://dev.balancer.fi/resources/pool-math/weighted-math
   * @param {BigNumber} tokenInId asset of token that will be provided to pool
   * @returns {BigNumber} spot price of the asset
   */
  async getSpotPrice(
    tokenInId: BigNumber
  ): Promise<BigNumber> {
    const assets = Object.keys(this.__assets.toJSON());
    const AssetInId = assets.find(asset => asset === tokenInId.toString());
    const AssetOutId = assets.find(asset => asset !== tokenInId.toString());
    if (AssetInId && AssetOutId) {
      const Wi = this.__assets.assets[AssetInId];
      const Wo = this.__assets.assets[AssetInId];

      const Bi = await this.getAssetLiquidity(new BigNumber(AssetInId));
      const Bo = await this.getAssetLiquidity(new BigNumber(AssetOutId));

      return new BigNumber(Bi.div(Wi)).div(new BigNumber(
        Bo.div(Wo)
      ));
    }

    return new BigNumber(0);
  }
}
