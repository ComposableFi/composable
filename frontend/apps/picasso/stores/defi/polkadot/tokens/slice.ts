import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { StoreSlice } from "@/stores/types";
import { Token, TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";

const ACALA_IDENTIFIERS: { [tokenId in TokenId]: string | null } =  {
  "eth": null,
  "matic": null,
  "avax": null,
  "weth": null,
  "usdc": null,
  "dot": null,
  "uni": null,
  "ftm": null,
  "pica": null,
  "movr": null,
  "ksm": "KSM",
  "pblo": null,
  "angl": null,
  "chaos": null,
  "usdt": null,
  "kar": null,
  "ausd": "AUSD",
  "kusd": "KUSD",
}


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
        karuraId: ACALA_IDENTIFIERS[tokenId as TokenId],
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
    list: Array<{
      name: string;
      id: BigNumber;
      foreignId?: string;
      decimals?: number;
    }>
  ) => void;
}

export interface TokensSlice {
  substrateTokens: InitialState & TokensSliceReducers;
}

export const createTokensSlice: StoreSlice<TokensSlice> = (set) => ({
  substrateTokens: {
    tokens: initialState,
    updateTokens: (list) => {
      set((state) => {
        list.forEach(asset => {
          const assetId = asset.name.toLowerCase();
          if (state.substrateTokens.tokens[assetId as TokenId]) {
            state.substrateTokens.tokens[assetId as TokenId].picassoId = asset.id;
          }
        });
        return state;
      })
    },
  },
});
