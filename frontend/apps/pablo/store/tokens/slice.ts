import { TOKENS } from "tokens";
import { StoreSlice } from "../types";
import { SubstrateNetwork, TokensSlice } from "./types";
import { TokenId } from "tokens";
import { DEFI_CONFIG } from "@/defi/config";
import BigNumber from "bignumber.js";
import { Asset } from "shared";

const createAssetsSlice: StoreSlice<TokensSlice> = (set) => ({
  substrateTokens: {
    hasFetchedTokens: false,
    tokenBalances: DEFI_CONFIG.substrateNetworks.reduce((agg, curr) => {
      agg[curr] = Object.keys(TOKENS).reduce((agg, curr) => {
        agg[curr as TokenId] = new BigNumber(0);
        return agg;
      }, {} as Record<TokenId, BigNumber>);
      return agg;
    }, {} as Record<SubstrateNetwork, Record<TokenId, BigNumber>>),
    tokens: Object.values(TOKENS).reduce((agg, token) => {
      agg[token.id] = new Asset(
        token.symbol,
        token.symbol,
        token.icon,
      );
      return agg;
    }, {} as Record<TokenId, Asset>),
    setTokens: (tokenMetadata) => {
      set((state) => {
        tokenMetadata.picasso.list.forEach((listItem) => {
          /**
           * Here identifier is lowercased
           * name mapped as token id
           * might change later
           * update decimals and id
           */
          const identifier = listItem.name.toLowerCase();
          if (state.substrateTokens.tokens[identifier as TokenId]) {
            console.log("[Picasso] Found Supported Asset", identifier);
            state.substrateTokens.tokens[identifier as TokenId].setIdOnChain("picasso", listItem.id);
            state.substrateTokens.tokens[identifier as TokenId].setApi(tokenMetadata.picasso.api);
          }
        });
        state.substrateTokens.hasFetchedTokens = true;
        return state;
      });
    },
    setTokenBalance: (tokenId, network, balance) => {
      set((state) => {
        state.substrateTokens.tokenBalances[network][tokenId] = balance;
        return state;
      });
    },
  },
});

export default createAssetsSlice;
