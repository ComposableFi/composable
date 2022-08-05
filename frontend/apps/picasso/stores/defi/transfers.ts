import { NamedSet } from "zustand/middleware";
import { StoreSlice } from "../types";
import BigNumber from "bignumber.js";
import { AssetId, SubstrateNetworkId } from "@/defi/polkadot/types";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";

interface Networks {
  options: { networkId: SubstrateNetworkId }[];
  from: SubstrateNetworkId;
  to: SubstrateNetworkId;
}

interface Recipients {
  selected: string;
}

interface TransfersState {
  networks: Networks;
  amount: BigNumber;
  tokenId: AssetId;
  recipients: Recipients;
  fee: BigNumber | number;
  keepAlive: boolean;
  feeItem: AssetId | "";
  hasFeeItem: boolean;
  existentialDeposit: BigNumber;
}

const networks = Object.keys(SUBSTRATE_NETWORKS).map((networkId) => ({
  networkId: networkId as SubstrateNetworkId,
}));

const initialState: TransfersState = {
  networks: {
    options: networks,
    from: networks[0].networkId,
    to: networks[1].networkId,
  },
  tokenId: "ksm",
  amount: new BigNumber(0),
  recipients: {
    selected: "",
  },
  fee: 0.5,
  hasFeeItem: false,
  feeItem: "",
  keepAlive: true,
  existentialDeposit: new BigNumber(0),
};

export interface TransfersSlice {
  transfers: TransfersState & {
    updateNetworks: (data: Omit<Networks, "options">) => void;
    updateAmount: (data: BigNumber) => void;
    updateRecipient: (selected: string) => void;
    updateTokenId: (data: AssetId) => void;
    flipKeepAlive: () => void;
    toggleHasFee: () => void;
    setFeeItem: (data: AssetId) => void;
    updateExistentialDeposit: (data: BigNumber) => void;
  };
}

export const createTransfersSlice: StoreSlice<TransfersSlice> = (
  set: NamedSet<TransfersSlice>
) => ({
  transfers: {
    ...initialState,

    updateNetworks: (data: Omit<Networks, "options">) => {
      set((state) => {
        state.transfers.networks = { ...state.transfers.networks, ...data };

        return state;
      });
    },
    updateAmount: (data: BigNumber) =>
      set((state) => {
        state.transfers.amount = data;

        return state;
      }),
    updateRecipient: (data: string) => {
      set((state) => {
        state.transfers.recipients.selected = data;

        return state;
      });
    },
    updateTokenId: (data: AssetId) => {
      set((state) => {
        state.transfers.tokenId = data;

        return state;
      });
    },
    flipKeepAlive: () => {
      set((state) => {
        state.transfers.keepAlive = !state.transfers.keepAlive;

        return state;
      });
    },
    setFeeItem: (data: AssetId) =>
      set((state) => {
        state.transfers.feeItem = data;

        return state;
      }),
    toggleHasFee: () => {
      set((state) => {
        state.transfers.hasFeeItem = !state.transfers.hasFeeItem;

        if (!state.transfers.hasFeeItem) {
          state.transfers.feeItem = "";
        }
        return state;
      });
    },
    updateExistentialDeposit: (data: BigNumber) =>
      set((state) => {
        state.transfers.existentialDeposit = data;

        return state;
      }),
  },
});
