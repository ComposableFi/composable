import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { StoreSlice } from "@/stores/types";
import { Token, TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";
import {
  HumanizedKaruraAssetMetadata,
  PicassoRpcAsset,
} from "@/defi/polkadot/pallets/Assets";

export type TokenMetadata = Token & {
  chainId: {
    picasso: BigNumber | null;
    karura: string | null;
    kusama: string | null;
  };
  decimals: Record<SubstrateNetworkId, number>;
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
      },
    };
  }, {} as Record<TokenId, TokenMetadata>),
  isLoaded: false,
};

interface TokensActions {
  updateTokens: (
    picassoList: Array<PicassoRpcAsset>,
    karuraList: Array<HumanizedKaruraAssetMetadata>
  ) => void;
}

export interface TokensSlice {
  substrateTokens: TokensState & TokensActions;
}

export const createTokensSlice: StoreSlice<TokensSlice> = (set) => ({
  substrateTokens: {
    tokens: initialState.tokens,
    isLoaded: initialState.isLoaded,
    updateTokens: (picassoList, karuraList) => {
      set((state) => {
        picassoList.forEach((listItem) => {
          /**
           * Here identifier is in lower case
           * name mapped as token id
           * might change later
           * update decimals and id
           */
          const identifier = listItem.name.toLowerCase();
          if (state.substrateTokens.tokens[identifier as TokenId]) {
            console.log("[Picasso] Found Supported Asset", identifier);
            state.substrateTokens.tokens[
              identifier as TokenId
            ].decimals.picasso = listItem.decimals ?? 12;
            state.substrateTokens.tokens[identifier as TokenId].chainId[
              "picasso"
            ] = listItem.id;
          }
        });
        karuraList.forEach((listItem) => {
          /**
           * Here identifier is in lower case
           * symbol mapped as token id
           * might change later
           * update decimals and id
           */
          const identifier = listItem.symbol.toLowerCase();
          if (state.substrateTokens.tokens[identifier as TokenId]) {
            console.log("[KARURA] Found Supported Asset", listItem.symbol);
            state.substrateTokens.tokens[
              identifier as TokenId
            ].decimals.picasso = listItem.decimals ?? 12;
            state.substrateTokens.tokens[identifier as TokenId].chainId[
              "karura"
            ] = listItem.symbol;
          }
        });
        if (picassoList.length + karuraList.length > 0) {
          state.substrateTokens.isLoaded = true;
        }
      });
    },
  },
});
