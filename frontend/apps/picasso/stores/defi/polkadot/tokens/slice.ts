import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { StoreSlice } from "@/stores/types";
import { Token, TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";
import { HumanizedKaruraAssetMetadata, PicassoRpcAsset } from "@/defi/polkadot/pallets/Assets";

export type TokenMetadata = Token & {
  picassoId: BigNumber | null;
  karuraId: string | null;
  kusamaId: number | null;
  decimals: Record<SubstrateNetworkId, number>;
};

type InitialState = {
  tokens: Record<TokenId, TokenMetadata>;
};

const initialState: InitialState["tokens"] = Object.keys(TOKENS).reduce(
  (agg, tokenId) => {
    return {
      ...agg,
      [tokenId]: {
        ...TOKENS[tokenId as TokenId],
        picassoId: null,
        kusamaId: null,
        karuraId: null,
        decimals: {
          kusama: null,
          picasso: 12,
          karura: null,
        },
      },
    };
  },
  {} as Record<TokenId, TokenMetadata>
);

interface TokensSliceReducers {
  updateTokens: (
    picassoList: Array<PicassoRpcAsset>,
    karuraList: Array<HumanizedKaruraAssetMetadata>
  ) => void;
}

export interface TokensSlice {
  substrateTokens: InitialState & TokensSliceReducers;
}

export const createTokensSlice: StoreSlice<TokensSlice> = (set) => ({
  substrateTokens: {
    tokens: initialState,
    updateTokens: (picassoList, karuraList) => {
      set((state) => {
        picassoList.forEach(listItem => {
          /**
           * Here identifier is lowercased
           * name mapped as token id
           * might change later
           * update decimals and id
           */
          const identifier = listItem.name.toLowerCase();
          if (state.substrateTokens.tokens[identifier as TokenId]) {
            console.log('[Picasso] Found Supported Asset', identifier);
            state.substrateTokens.tokens[identifier as TokenId].decimals.picasso = listItem.decimals ?? 12;
            state.substrateTokens.tokens[identifier as TokenId].picassoId = listItem.id;
          }
        });
        karuraList.forEach(listItem => {
          /**
           * Here identifier is lowercased
           * symbol mapped as token id
           * might change later
           * update decimals and id
           */
          const identifier = listItem.symbol.toLowerCase();
          if (state.substrateTokens.tokens[identifier as TokenId]) {
            console.log('[KARURA] Found Supported Asset', listItem.symbol);
            state.substrateTokens.tokens[identifier as TokenId].decimals.picasso = listItem.decimals ?? 12;
            state.substrateTokens.tokens[identifier as TokenId].karuraId = listItem.symbol;
          }
        });
        return state;
      })
    },
  },
});
