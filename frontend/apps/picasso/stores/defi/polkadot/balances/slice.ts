import { SUBSTRATE_NETWORK_IDS } from "@/defi/polkadot/Networks";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { StoreSlice } from "@/stores/types";
import { TokenId, TOKENS } from "tokens";
import BigNumber from "bignumber.js";

export type TokenBalance = {
  balance: BigNumber;
  existentialDeposit: BigNumber;
};

type InitialState = {
  balances: {
    [chainId in SubstrateNetworkId]: {
      [assetId in TokenId]: TokenBalance;
    };
  };
};
const initialState: InitialState = SUBSTRATE_NETWORK_IDS.reduce(
  (prev, chain: SubstrateNetworkId) => {
    return {
      balances: {
        ...prev.balances,
        [chain]: Object.keys(TOKENS).reduce((agg, tokenId) => {
          agg[tokenId as TokenId] = {
            balance: new BigNumber(0),
            existentialDeposit: new BigNumber(0),
          };

          return agg;
        }, {} as { [assetId in TokenId]: TokenBalance }),
      },
    };
  },
  {} as InitialState
);
export interface SubstrateBalancesActions {
  updateBalance: (data: {
    network: SubstrateNetworkId;
    tokenId: TokenId;
    balance: BigNumber;
    existentialDeposit?: BigNumber;
  }) => void;
  clearBalance: () => void;
  getBalance: (token: TokenId, network: SubstrateNetworkId) => BigNumber;
}

export interface SubstrateBalancesSlice {
  substrateBalances: InitialState & SubstrateBalancesActions;
}

export const createSubstrateBalancesSlice: StoreSlice<
  SubstrateBalancesSlice
> = (set, get) => ({
  substrateBalances: {
    ...initialState,
    updateBalance: ({
      network,
      tokenId,
      balance,
      existentialDeposit,
    }: {
      network: SubstrateNetworkId;
      tokenId: TokenId;
      balance: BigNumber;
      existentialDeposit?: BigNumber;
    }) => {
      set((state) => {
        state.substrateBalances.balances[network][tokenId].balance = balance;
        state.substrateBalances.balances[network][tokenId].existentialDeposit =
          existentialDeposit ??
          state.substrateBalances.balances[network][tokenId].existentialDeposit;
        return state;
      });
    },
    clearBalance: () => {
      set((state) => {
        for (const network in state.substrateBalances.balances) {
          for (const token in state.substrateBalances.balances[
            network as SubstrateNetworkId
          ]) {
            state.substrateBalances.balances[network as SubstrateNetworkId][
              token as TokenId
            ].balance = new BigNumber(0);
          }
        }

        return state;
      });
    },
    getBalance: (token: TokenId, network: SubstrateNetworkId): BigNumber => {
      return get().substrateBalances.balances[network][token].balance;
    },
    getAssetBalance: (tokenId: TokenId, network: SubstrateNetworkId) => {
      return get().substrateBalances.balances[network][tokenId].balance
    },
  },
});
