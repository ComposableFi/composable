import { Asset, fromPerbill, humanizedBnToBn } from "shared";
import { ApiPromise } from "@polkadot/api";
import { BasePabloPool } from "./BasePabloPool";
import { PabloPoolFeeConfig } from "./PabloPoolFeeConfig";
import { PabloPoolPair } from "./PabloPoolPair";
import { LiquidityProviderToken } from "../LiquidityProviderToken";
import BigNumber from "bignumber.js";

export class PabloConstantProductPool extends BasePabloPool {
  protected __owner: string;
  protected __lpToken: LiquidityProviderToken;
  protected __baseWeight: BigNumber;

  static fromJSON(
    poolIndex: BigNumber,
    api: ApiPromise,
    supportedAssets: Asset[],
    constantProductPoolJSON: any
  ): PabloConstantProductPool {
    try {
      const lpTokenAssetId = humanizedBnToBn(constantProductPoolJSON.lpToken);
      const pair = PabloPoolPair.fromJSON(constantProductPoolJSON.pair);

      const underlyingAssets = supportedAssets.filter(
        (a) =>
          (a.getPicassoAssetId(true) as BigNumber).eq(pair.getBaseAsset()) ||
          (a.getPicassoAssetId(true) as BigNumber).eq(pair.getQuoteAsset())
      );

      const lpToken = new LiquidityProviderToken(api, underlyingAssets, lpTokenAssetId);
      const baseWeight = fromPerbill(constantProductPoolJSON.baseWeight);
      return new PabloConstantProductPool(
        api,
        poolIndex,
        PabloPoolPair.fromJSON(constantProductPoolJSON.pair),
        PabloPoolFeeConfig.fromJSON(constantProductPoolJSON.feeConfig),
        constantProductPoolJSON.owner,
        lpToken,
        baseWeight
      );
    } catch (err: any) {
      console.error("[LiquidityBootstrappingPool] ", err);
      throw new Error(err.message);
    }
  }

  constructor(
    api: ApiPromise,
    poolId: BigNumber,
    pair: PabloPoolPair,
    feeConfig: PabloPoolFeeConfig,
    owner: string,
    lpToken: LiquidityProviderToken,
    baseWeight: BigNumber
  ) {
    super(api, poolId, pair, feeConfig);
    this.__owner = owner;
    this.__lpToken = lpToken;
    this.__baseWeight = baseWeight;
  }

  getLiquidityProviderToken(): LiquidityProviderToken {
    return this.__lpToken;
  }
}
