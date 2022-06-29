import { DEFI_CONFIG } from "@/defi/polkadot/config";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenId, TOKENS } from "@/defi/Tokens";

import { RootState } from "@/stores/root";
import { createSlice, PayloadAction } from "@reduxjs/toolkit";

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

export const substrateBalancesSlice = createSlice({
  name: "SubstrateBalances",
  initialState,
  reducers: {
    updateBalance: (
      state,
      action: PayloadAction<{
        substrateNetworkId: SubstrateNetworkId;
        balance: string;
      }>
    ) => {
      const { substrateNetworkId, balance } = action.payload;
      state[substrateNetworkId].balance = balance;
    },
    clearBalance: (state) => {
      const networks = Object.keys(state).map(
        (substrateNetworkIdStr) => substrateNetworkIdStr as SubstrateNetworkId
      );
      for (
        let networkIndex = 0;
        networkIndex < networks.length;
        networkIndex++
      ) {
        state[networks[networkIndex]].balance = "0";
      }
    },
  },
});

export const { updateBalance, clearBalance } = substrateBalancesSlice.actions;

export const selectSubstrateBalances = (state: RootState) =>
  state.substrateBalances;

export default substrateBalancesSlice.reducer;
