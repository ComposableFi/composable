import { DEFI_CONFIG } from "@/defi/polkadot/config";
import {
  SUBSTRATE_NETWORK_IDS,
  SUBSTRATE_NETWORKS,
} from "@/defi/polkadot/Networks";
import { AssetId, SubstrateNetworkId } from "@/defi/polkadot/types";
import { Token, TokenId, TOKENS } from "tokens";

import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "../../../types";
import BigNumber from "bignumber.js";
import { AssetMetadata, Assets } from "@/defi/polkadot/Assets";

export interface SubstrateAsset {
  balance: string;
  price: number;
  value: number;
  change_24hr: number;
  icon: string;
  decimalsToDisplay: number;
  tokenId: TokenId;
  symbol: string;
}

const initialBaseTokens: {
  [chainId in SubstrateNetworkId]: SubstrateAsset;
} = DEFI_CONFIG.networkIds.reduce((prev, chain) => {
  return {
    ...prev,
    [chain]: {
      balance: "0",
      price: 0,
      value: 0,
      change_24hr: 0,
      decimalsToDisplay:
        TOKENS[SUBSTRATE_NETWORKS[chain].tokenId].decimalsToDisplay,
      icon: TOKENS[SUBSTRATE_NETWORKS[chain].tokenId].icon,
      tokenId: SUBSTRATE_NETWORKS[chain].tokenId,
      symbol: SUBSTRATE_NETWORKS[chain].symbol,
    },
  };
}, {} as { [chainId in SubstrateNetworkId]: SubstrateAsset });

type InitialState = {
  [chainId in SubstrateNetworkId]: {
    native: {
      balance: BigNumber;
      meta: Token;
    };
    assets: {
      [assetId in AssetId]: {
        balance: BigNumber;
        meta: AssetMetadata;
      };
    };
  };
};
const initialState: InitialState = <InitialState>SUBSTRATE_NETWORK_IDS.reduce(
  (prev, chain: SubstrateNetworkId) => {
    return {
      ...prev,
      [chain]: {
        native: {
          balance: new BigNumber(0),
          meta: TOKENS[SUBSTRATE_NETWORKS[chain].tokenId],
        },
        assets: Object.values(Assets).reduce((acc, asset) => {
          if (Object.keys(asset.supportedNetwork).includes(chain)) {
            return {
              ...acc,
              [asset.assetId]: {
                meta: asset,
                balance: new BigNumber(0),
              },
            };
          }
          return acc;
        }, {}),
      },
    };
  },
  {}
);

export interface SubstrateBalancesSlice {
  substrateBalances: InitialState & {
    updateBalance: (data: {
      substrateNetworkId: SubstrateNetworkId;
      balance: string;
    }) => void;
    clearBalance: () => void;
    updateAssetBalance: (data: {
      substrateNetworkId: SubstrateNetworkId;
      assetId: AssetId;
      balance: BigNumber;
    }) => void;
  };
}

export const createSubstrateBalancesSlice: StoreSlice<
  SubstrateBalancesSlice
> = (set: NamedSet<SubstrateBalancesSlice>) => ({
  substrateBalances: {
    ...initialState,
    updateBalance: ({
      substrateNetworkId,
      balance,
    }: {
      substrateNetworkId: SubstrateNetworkId;
      balance: string;
    }) => {
      set((state) => {
        state.substrateBalances[substrateNetworkId].native.balance =
          new BigNumber(balance);
        return state;
      });
    },
    clearBalance: () => {
      set((state) => {
        DEFI_CONFIG.networkIds.forEach((network) => {
          state.substrateBalances[network].native.balance = new BigNumber(0);
        });

        return state;
      });
    },
    updateAssetBalance: ({ substrateNetworkId, assetId, balance }) => {
      set((state) => {
        state.substrateBalances[substrateNetworkId].assets[assetId].balance =
          new BigNumber(balance);
        return state;
      });
    },
  },
});
