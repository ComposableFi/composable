import { TokenId, TOKENS } from "tokens";
import { StoreSlice } from "../types";
import { TokensSlice } from "./types";
import { Asset } from "shared";

const createTokensSlice: StoreSlice<TokensSlice> = (set) => ({
  substrateTokens: {
    hasFetchedTokens: false,
    tokens: Object.values(TOKENS).reduce((agg, token) => {
      agg[token.id] = new Asset(
        token.symbol,
        token.symbol,
        token.icon,
        token.id
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
          if (
            state.substrateTokens.tokens[identifier as TokenId] &&
            listItem.decimals
          ) {
            console.log("[Pablo] Found Supported Asset", identifier);
            state.substrateTokens.tokens[identifier as TokenId].setIdOnChain(
              "picasso",
              listItem.id
            );
            state.substrateTokens.tokens[identifier as TokenId].setApi(
              tokenMetadata.picasso.api
            );

            state.substrateTokens.tokens[identifier as TokenId].setDecimals(
              "picasso",
              listItem.decimals
            );

            state.substrateTokens.tokens[
              identifier as TokenId
            ].setExistentialDeposit("picasso", listItem.existentialDeposit);
            state.substrateTokens.tokens[identifier as TokenId].setRatio(
              listItem.ratio
            );
          }
        });
        state.substrateTokens.hasFetchedTokens = true;
        return state;
      });
    },
  },
});

export default createTokensSlice;
