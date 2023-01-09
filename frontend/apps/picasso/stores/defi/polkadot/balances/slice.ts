import { StoreSlice } from "@/stores/types";
import { TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";
import { SubstrateNetworkId } from "shared";
import { SUBSTRATE_NETWORKS } from "shared/defi/constants";

export type TokenBalance = {
  free: BigNumber;
  locked: BigNumber;
};

type SubstrateBalancesState = {
  balances: {
    [chainId in SubstrateNetworkId]: {
      [assetId in TokenId]: TokenBalance;
    };
  };
};
const initialState: SubstrateBalancesState = Object.keys(
  SUBSTRATE_NETWORKS
).reduce((prev, chain) => {
  return {
    balances: {
      ...prev.balances,
      [chain]: Object.keys(TOKENS).reduce((agg, tokenId) => {
        agg[tokenId as TokenId] = {
          free: new BigNumber(0),
            locked: new BigNumber(0),
        };

        return agg;
      }, {} as { [assetId in TokenId]: TokenBalance }),
    },
  };
}, {} as SubstrateBalancesState);

export interface SubstrateBalancesActions {
  updateBalance: (data: {
    network: SubstrateNetworkId;
    tokenId: TokenId;
    balance: TokenBalance;
  }) => void;
  clearBalance: () => void;
  getBalance: (token: TokenId, network: SubstrateNetworkId) => TokenBalance;
}

export interface SubstrateBalancesSlice {
  substrateBalances: SubstrateBalancesState & SubstrateBalancesActions;
}

export const createSubstrateBalancesSlice: StoreSlice<
  SubstrateBalancesSlice
> = (set, get) => ({
  substrateBalances: {
    ...initialState,
    clearBalance: () => {
      set((state) => {
        for (const network in state.substrateBalances.balances) {
          for (const token in state.substrateBalances.balances[
            network as SubstrateNetworkId
          ]) {
            state.substrateBalances.balances[network as SubstrateNetworkId][
              token as TokenId
            ].free = new BigNumber(0);
            state.substrateBalances.balances[network as SubstrateNetworkId][
              token as TokenId
            ].locked = new BigNumber(0);
          }
        }

        return state;
      });
    },
    updateBalance: ({
      network,
      tokenId,
      balance,
    }: {
      network: SubstrateNetworkId;
      tokenId: TokenId;
      balance: TokenBalance;
    }) => {
      set((state) => {
        state.substrateBalances.balances[network][tokenId] = balance;
        return state;
      });
    },
    getBalance: (token: TokenId, network: SubstrateNetworkId): TokenBalance => {
      return get().substrateBalances.balances[network][token];
    },
  },
});
