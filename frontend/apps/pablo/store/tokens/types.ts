import { TokenId } from "tokens";
import { DEFI_CONFIG } from "../../defi/config/index";
import { Asset, PicassoRpcAsset } from "shared";
import { ApiPromise } from "@polkadot/api";
import BigNumber from "bignumber.js";

export type SubstrateNetwork = typeof DEFI_CONFIG.substrateNetworks[number];
export type PicassoAssetsRPCMetadata = Array<PicassoRpcAsset>;
export type KaruraAssetsRPCMetadata = Array<{
  name: string;
  symbol: string;
  decimals: number;
  minimalBalance: string;
  existentialDeposit: BigNumber;
  ratio: {
    n: BigNumber;
    d: BigNumber;
  };
}>;

export interface TokensSlice {
  substrateTokens: {
    hasFetchedTokens: boolean;
    tokens: Record<TokenId, Asset>;
    setTokens: (tokenMetadata: {
      picasso: {
        list: PicassoAssetsRPCMetadata;
        api: ApiPromise;
      };
    }) => void;
  };
}
