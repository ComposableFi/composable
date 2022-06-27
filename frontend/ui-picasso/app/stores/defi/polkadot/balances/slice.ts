import { DEFI_CONFIG } from "@/defi/polkadot/config";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenId, TOKENS } from "@/defi/Tokens";

import { NamedSet } from "zustand/middleware";
import { AllSlices, StoreSlice } from "../../../types";

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

const initialState: { [chainId in SubstrateNetworkId]: SubstrateAsset } =
  DEFI_CONFIG.networkIds.reduce((prev, curr, ind) => {
    return {
      ...prev,
      [curr]: {
        balance: "0",
        price: 0,
        value: 0,
        change_24hr: 0,
        decimalsToDisplay:
          TOKENS[SUBSTRATE_NETWORKS[curr].tokenId].decimalsToDisplay,
        icon: TOKENS[SUBSTRATE_NETWORKS[curr].tokenId].icon,
        tokenId: SUBSTRATE_NETWORKS[curr].tokenId,
        symbol: SUBSTRATE_NETWORKS[curr].symbol,
      },
    };
  }, {} as { [chainId in SubstrateNetworkId]: SubstrateAsset });

export interface SubstrateBalancesSlice {
  substrateBalances: { [chainId in SubstrateNetworkId]: SubstrateAsset } & {
    updateBalance: (data: {
      substrateNetworkId: SubstrateNetworkId;
      balance: string;
    }) => void;
    clearBalance: () => void;
  };
}

export const createSubstrateBalancesSlice: StoreSlice<SubstrateBalancesSlice> =
  (set: NamedSet<SubstrateBalancesSlice>) => ({
    substrateBalances: {
      ...initialState,
      updateBalance: (data: {
        substrateNetworkId: SubstrateNetworkId;
        balance: string;
      }) => {
        set((state: AllSlices) => {
          const { substrateNetworkId, balance } = data;
          state.substrateBalances[substrateNetworkId].balance = balance;
        });
      },
      clearBalance: () => {
        set((state: AllSlices) => {
          DEFI_CONFIG.networkIds.forEach((network) => {
            state.substrateBalances[network].balance = "0";
          });
        });
      },
    },
  });
