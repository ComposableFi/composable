import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import BigNumber from "bignumber.js";

import { TokenId, TOKEN_IDS } from "@/defi/Tokens";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

interface Networks {
  options: { networkId: SubstrateNetworkId }[];
  from: string;
  to: string;
}

interface Amount {
  options: { tokenId: TokenId }[];
  tokenId: TokenId;
  value: BigNumber | number;
  balance: BigNumber | number;
}

interface Recipient {
  value: string;
  label: string;
  icon: string;
}

interface Recipients {
  options: Recipient[];
  selected: string;
}

interface TransfersState {
  networks: Networks;
  amount: Amount;
  recipients: Recipients;
  fee: BigNumber | number;
  keepAlive: boolean;
}

const recipients = [
  {
    value: "select1",
    label: "Select 1",
    icon: "/tokens/eth-mainnet.svg",
  },
  {
    value: "select2",
    label: "Select 2",
    icon: "/tokens/eth-mainnet.svg",
  },
  {
    value: "select3",
    label: "Select 3",
    icon: "/tokens/eth-mainnet.svg",
  },
];

const networks = Object.keys(SUBSTRATE_NETWORKS).map((networkId) => ({
  networkId: networkId as SubstrateNetworkId,
}));

const tokens = TOKEN_IDS.map((tokenId) => ({ tokenId }));

const initialState: TransfersState = {
  networks: {
    options: networks,
    from: networks[0].networkId,
    to: networks[1].networkId,
  },
  amount: {
    options: tokens,
    tokenId: TOKEN_IDS[1],
    value: 0,
    balance: 350,
  },
  recipients: {
    options: recipients,
    selected: "select1",
  },
  fee: 0.5,
  keepAlive: true,
};

export const transfersSlice = createSlice({
  name: "Transfers",
  initialState,
  reducers: {
    updateNetworks: (
      state,
      action: PayloadAction<Omit<Networks, "options">>
    ) => {
      state.networks = { ...state.networks, ...action.payload };
    },
    updateAmount: (
      state,
      action: PayloadAction<Omit<Amount, "balance" | "options">>
    ) => {
      state.amount = { ...state.amount, ...action.payload };
    },
    updateRecipient: (state, action: PayloadAction<string>) => {
      state.recipients.selected = action.payload;
    },
    flipKeepAlive: (state) => {
      state.keepAlive = !state.keepAlive;
    },
  },
});

export const { updateNetworks, updateAmount, updateRecipient, flipKeepAlive } =
  transfersSlice.actions;

export default transfersSlice.reducer;
