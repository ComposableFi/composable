import BigNumber from "bignumber.js";
import { option } from "fp-ts";
import { TokenMetadata } from "@/stores/defi/polkadot/tokens/slice";

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
    assets: [TokenMetadata, TokenMetadata];
    feeConfig: {
      feeRate: number;
      ownerFeeRate: number;
      protocolFeeRate: number;
    };
  };
};
export type AssetAmountPair = Record<string, string>;
export type PoolAmount = Record<string, AssetAmountPair>;

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
  pair: [TokenMetadata, TokenMetadata];
  poolId: PoolId;
};
export type OwnedLiquidityTokens = {
  [key in LPTokenId]: LPTokenState;
};

export interface OwnedLiquiditySlice {
  ownedLiquidity: {
    tokens: OwnedLiquidityTokens;
    isLoaded: boolean;

    setOwnedLiquidity: (
      lpTokenId: LPTokenId,
      balance: {
        free: BigNumber;
        locked: BigNumber;
      },
      pair: [TokenMetadata, TokenMetadata],
      poolId: PoolId
    ) => void;
  };
}
