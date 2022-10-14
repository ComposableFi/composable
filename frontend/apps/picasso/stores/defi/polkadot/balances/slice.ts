import { DEFI_CONFIG } from "@/defi/polkadot/config";
import {
  SUBSTRATE_NETWORK_IDS,
  SUBSTRATE_NETWORKS,
} from "@/defi/polkadot/Networks";
import { AssetId, SubstrateNetworkId } from "@/defi/polkadot/types";
import { Token, TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";
import { AssetMetadata, Assets } from "@/defi/polkadot/Assets";
import { StoreSlice } from "@/stores/types";

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

type InitialState = {
  assets: {
    [chainId in SubstrateNetworkId]: {
      native: {
        balance: BigNumber;
        meta: Token;
        existentialDeposit: BigNumber;
      };
      assets: {
        [assetId in AssetId]: {
          balance: BigNumber;
          meta: AssetMetadata;
        };
      };
    };
  };
};
const initialState: InitialState = SUBSTRATE_NETWORK_IDS.reduce(
  (prev, chain: SubstrateNetworkId) => {
    return {
      assets: {
        ...prev.assets,
        [chain]: {
          native: {
            balance: new BigNumber(0),
            meta: TOKENS[SUBSTRATE_NETWORKS[chain].tokenId],
            existentialDeposit: new BigNumber(0),
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
      },
    };
  },
  {} as InitialState
);

export interface SubstrateBalancesSlice {
  substrateBalances: InitialState & {
    updateBalance: (data: {
      substrateNetworkId: SubstrateNetworkId;
      balance: string;
      existentialDeposit: BigNumber;
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
> = (set) => ({
  substrateBalances: {
    ...initialState,
    updateBalance: ({
      substrateNetworkId,
      balance,
      existentialDeposit,
    }: {
      substrateNetworkId: SubstrateNetworkId;
      balance: string;
      existentialDeposit: BigNumber;
    }) => {
      set((state) => {
        state.substrateBalances.assets[substrateNetworkId].native.balance =
          new BigNumber(balance);
        state.substrateBalances.assets[
          substrateNetworkId
        ].native.existentialDeposit = existentialDeposit;
        return state;
      });
    },
    clearBalance: () => {
      set((state) => {
        DEFI_CONFIG.networkIds.forEach((network) => {
          state.substrateBalances.assets[network].native.balance =
            new BigNumber(0);
        });

        return state;
      });
    },
    updateAssetBalance: ({ substrateNetworkId, assetId, balance }) => {
      set((state) => {
        state.substrateBalances.assets[substrateNetworkId].assets[
          assetId
        ].balance = new BigNumber(balance);
        return state;
      });
    },
  },
});
