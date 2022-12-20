import BigNumber from "bignumber.js";
import { option } from "fp-ts";
import { Asset } from "shared";

export type PoolKind = "dualAssetConstantPool" | "";
export type LPTokenId = number;
export type PoolId = BigNumber;
export type PoolConfig = {
  kind: PoolKind;
  poolId: PoolId;
  config: {
    lpToken: LPTokenId;
    owner: string;
    assetsWeights: {
      [assetId in number]: number;
    };
    assets: [Asset, Asset];
    feeConfig: {
      feeRate: number;
      ownerFeeRate: number;
      protocolFeeRate: number;
    };
  };
};
export type AssetAmountPair = {
  [key in string]: string;
};
export type PoolAmount = {
  [key in string]: AssetAmountPair;
};

export interface PoolSlice {
  pools: {
    poolAmount: PoolAmount;
    isLoaded: boolean;
    config: PoolConfig[];
    setConfig: (pools: PoolConfig[]) => void;
    getPoolById: (poolId: string) => option.Option<PoolConfig>;
    setPoolAmount: (poolId: string, payload: AssetAmountPair) => void;
    totalIssued: {
      [PoolId in string]: BigNumber;
    };
    setTotalIssued: (poolId: PoolId, totalIssued: BigNumber) => void;
  };
}

export type LPTokenState = {
  balance: {
    free: BigNumber;
    locked: BigNumber;
  };
  pair: [Asset, Asset];
  poolId: PoolId;
};
export type OwnedLiquidityTokens = {
  [key in LPTokenId]: LPTokenState;
};

export interface OwnedLiquiditySlice {
  ownedLiquidity: {
    tokens: OwnedLiquidityTokens;

    setOwnedLiquidity: (
      lpTokenId: LPTokenId,
      balance: {
        free: BigNumber;
        locked: BigNumber;
      },
      pair: [Asset, Asset],
      poolId: PoolId
    ) => void;
  };
}
