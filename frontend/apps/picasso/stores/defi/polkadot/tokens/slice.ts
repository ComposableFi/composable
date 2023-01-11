import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { StoreSlice } from "@/stores/types";
import { Token, TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";
import {
  AssetRatio,
  PicassoRpcAsset,
  StatemineAssetMetadata,
} from "@/defi/polkadot/pallets/Assets";
import { KusamaAsset } from "@/defi/polkadot/pallets/Assets/kusama";
import { ParachainNetworks } from "substrate-react";

export type TokenMetadata = Token & {
  chainId: {
    picasso: BigNumber | null;
    karura: string | null;
    kusama: string | null;
    statemine: string | null;
  };
  ratio: Record<SubstrateNetworkId, AssetRatio | null>;
  decimals: Record<SubstrateNetworkId, number | null>;
  existentialDeposit: Record<SubstrateNetworkId, BigNumber | null>;
};

type TokensState = {
  tokens: Record<TokenId, TokenMetadata>;
  isLoaded: boolean;
};

const initialState = {
  tokens: Object.keys(TOKENS).reduce((agg, tokenId) => {
    return {
      ...agg,
      [tokenId]: {
        ...TOKENS[tokenId as TokenId],
        chainId: {
          picasso: null,
          kusama: null,
          karura: null,
        },
        decimals: {
          kusama: null,
          picasso: 12,
          karura: null,
        },
        existentialDeposit: {
          kusama: null,
          picasso: null,
          karura: null,
        },
        ratio: {
          kusama: null,
          picasso: null,
          karura: null,
        },
      },
    };
  }, {} as Record<TokenId, TokenMetadata>),
  isLoaded: false,
};

interface TokensActions {
  updateTokens: (
    picassoList: Array<PicassoRpcAsset> | null,
    // karuraList: Array<HumanizedKaruraAssetMetadata>,
    statemineList: Array<StatemineAssetMetadata> | null,
    kusamaAssetMetadata: KusamaAsset | null
  ) => void;
}

export interface TokensSlice {
  substrateTokens: TokensState & TokensActions;
}

export const createTokensSlice: StoreSlice<TokensSlice> = (set) => ({
  substrateTokens: {
    tokens: initialState.tokens,
    isLoaded: initialState.isLoaded,
    updateTokens: (
      picassoList,
      // karuraList,
      statemineList,
      kusamaAssetMetadata
    ) => {
      set((state) => {
        if (statemineList) {
          statemineList.forEach((listItem) => {
            if (listItem.id === "1984" || listItem.id === "ksm") {
              const token =
                state.substrateTokens.tokens[
                  listItem.id === "1984" ? "usdt" : ("ksm" as TokenId)
                ];
              console.log("[Statemine] Found supported asset", token.id);
              token.decimals.statemine =
                listItem.decimals ?? ParachainNetworks.statemine.decimals;
              token.chainId.statemine = listItem.id;
              token.existentialDeposit.statemine = listItem.existentialDeposit;
            }
          });
        }
        
        if (picassoList) {
          picassoList.forEach((listItem) => {
            /**
             * Here identifier is in lower case
             * name mapped as token id
             * might change later
             * update decimals and id
             */
            const identifier = listItem.name.toLowerCase();
            const token = state.substrateTokens.tokens[identifier as TokenId];
            if (token) {
              token.decimals.picasso = listItem.decimals ?? 12;
              token.chainId.picasso = listItem.id;
              token.existentialDeposit.picasso = listItem.existentialDeposit;
              token.ratio.picasso = listItem.ratio;
            }
          });
        }
        
        // karuraList.forEach(listItem => {
        //   /**
        //    * Here identifier is in lower case
        //    * symbol mapped as token id
        //    * might change later
        //    * update decimals and id
        //    */
        //   const identifier = listItem.symbol.toLowerCase();
        //   const token = state.substrateTokens.tokens[identifier as TokenId];
        //   if (token && listItem.decimals) {
        //     console.log("[KARURA] Found Supported Asset", listItem.symbol);
        //     token.decimals.karura = listItem.decimals;
        //     token.chainId.karura = listItem.symbol;
        //     token.existentialDeposit.karura = fromChainIdUnit(
        //       unwrapNumberOrHex(listItem.minimalBalance),
        //       listItem.decimals
        //     );
        //   }
        // });

        // This is done only for Kusama chain
        // If more tokens are imported, this needs a dedicated function
        if (kusamaAssetMetadata) {
          state.substrateTokens.tokens.ksm.decimals.kusama =
          kusamaAssetMetadata.decimals;

          state.substrateTokens.tokens.ksm.chainId.kusama =
          kusamaAssetMetadata.chainId;
          
          state.substrateTokens.tokens.ksm.existentialDeposit.kusama =
          kusamaAssetMetadata.existentialDeposit;
        }


        // TODO : enable this condition when karura is supported
        // if (picassoList.length + karuraList.length > 0) {
        //   state.substrateTokens.isLoaded = true;
        // }
        if (picassoList && picassoList.length > 0) {
          state.substrateTokens.isLoaded = true;
        }
      });
    },
  },
});
