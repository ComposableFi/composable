import { TokenId } from "tokens";
import { DEFI_CONFIG } from "../../defi/config/index";
import { ComposableTraitsAssetsXcmAssetLocation } from "defi-interfaces";
import BigNumber from "bignumber.js";
import { Asset } from "shared";
import { ApiPromise } from "@polkadot/api";

export type SubstrateNetwork = typeof DEFI_CONFIG.substrateNetworks[number];
export type PicassoAssetsRPCMetadata = Array<{
    id: BigNumber;
    name: string;
    decimals: number;
    foreignId: ComposableTraitsAssetsXcmAssetLocation | null
}>;
export type KaruraAssetsRPCMetadata = Array<{
    name: string;
    symbol: string;
    decimals: number;
    minimalBalance: string;
}>;

export interface TokensSlice {
    substrateTokens: {
        hasFetchedTokens: boolean,
        tokens: Record<TokenId, Asset>;
        tokenBalances: Record<SubstrateNetwork, Record<TokenId, BigNumber>>;
        setTokens: (
            tokenMetadata: {
                picasso: {
                    list: PicassoAssetsRPCMetadata,
                    api: ApiPromise
                }
            }
        ) => void;
        setTokenBalance: (
            tokenId: TokenId,
            network: SubstrateNetwork,
            balance: BigNumber
        ) => void;
    }
}