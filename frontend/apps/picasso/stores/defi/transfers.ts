import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "../types";
import BigNumber from "bignumber.js";

import { TOKEN_IDS, TokenId } from "tokens";
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
    icon: "/tokens/eth-mainnet.svg"
  },
  {
    value: "select2",
    label: "Select 2",
    icon: "/tokens/eth-mainnet.svg"
  },
  {
    value: "select3",
    label: "Select 3",
    icon: "/tokens/eth-mainnet.svg"
  }
];

const networks = Object.keys(SUBSTRATE_NETWORKS).map(networkId => ({
  networkId: networkId as SubstrateNetworkId
}));

const tokens = TOKEN_IDS.map(tokenId => ({ tokenId }));

const initialState: TransfersState = {
  networks: {
    options: networks,
    from: networks[0].networkId,
    to: networks[1].networkId
  },
  amount: {
    options: tokens,
    tokenId: TOKEN_IDS[1],
    value: 0,
    balance: 350
  },
  recipients: {
    options: recipients,
    selected: "select1"
  },
  fee: 0.5,
  keepAlive: true
};

export interface TransfersSlice {
  transfers: TransfersState & {
    updateNetworks: (data: Omit<Networks, "options">) => void;
    updateAmount: (data: Omit<Amount, "balance" | "options">) => void;
    updateRecipient: (selected: string) => void;
    flipKeepAlive: () => void;
  };
}

export const createTransfersSlice: StoreSlice<TransfersSlice> = (
  set: NamedSet<TransfersSlice>
) => ({
  transfers: {
    ...initialState,

    updateNetworks: (data: Omit<Networks, "options">) => {
      set(state => {
        state.transfers.networks = { ...state.transfers.networks, ...data };

        return state;
      });
    },
    updateAmount: (data: Omit<Amount, "balance" | "options">) => {
      set(state => {
        state.transfers.amount = { ...state.transfers.amount, ...data };

        return state;
      });
    },
    updateRecipient: (data: string) => {
      set(state => {
        state.transfers.recipients.selected = data;

        return state;
      });
    },
    flipKeepAlive: () => {
      set(state => {
        state.transfers.keepAlive = !state.transfers.keepAlive;

        return state;
      });
    }
  }
});
