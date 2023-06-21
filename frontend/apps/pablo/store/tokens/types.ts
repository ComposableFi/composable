import { TokenId } from "tokens";
import { DEFI_CONFIG } from "../../defi/config/index";
import { Asset, PicassoRpcAsset, SubstrateNetworkId } from "shared";
import { ApiPromise } from "@polkadot/api";

export type SubstrateNetwork = typeof DEFI_CONFIG.substrateNetworks[number];
export type PicassoAssetsRPCMetadata = Array<PicassoRpcAsset>;

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
    getTokenById: (
      onChainId: string,
      network: SubstrateNetworkId
    ) => Asset | undefined;
  };
}
