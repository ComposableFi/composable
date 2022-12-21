import BigNumber from "bignumber.js";
import { option } from "fp-ts";
import { Asset } from "shared";

export type PoolKind = "dualAssetConstantPool" | "";

export type PoolConfig = {
  kind: PoolKind;
  poolId: BigNumber;
  config: {
    lpToken: number;
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

export interface PoolSlice {
  pools: {
    isLoaded: boolean;
    config: PoolConfig[];
    setConfig: (pools: PoolConfig[]) => void;
    getPoolById: (poolId: string) => option.Option<PoolConfig>;
  };
}
