import { StoreSlice } from "../../../types";
import BigNumber from "bignumber.js";
import { AssetId, SubstrateNetworkId } from "@/defi/polkadot/types";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { AssetMetadata } from "@/defi/polkadot/Assets";
import { Token } from "tokens";

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
  keepAlive: boolean;
  feeItem: AssetId | "";
  hasFeeItem: boolean;
  existentialDeposit: BigNumber;
  feeToken: number;
  fee: {
    class: string;
    partialFee: BigNumber;
    weight: BigNumber;
  };
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
  hasFeeItem: false,
  feeItem: "",
  keepAlive: true,
  existentialDeposit: new BigNumber(0),
  feeToken: 0,
  fee: {
    class: "Normal",
    partialFee: new BigNumber(0),
    weight: new BigNumber(0),
  },
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
    updateFee: (data: {
      class: string;
      weight: BigNumber;
      partialFee: BigNumber;
    }) => void;
    updateExistentialDeposit: (data: BigNumber) => void;
    updateFeeToken: (data: number) => void;
    getFeeToken: (network: SubstrateNetworkId) => AssetMetadata | Token;
  };
}

export const createTransfersSlice: StoreSlice<TransfersSlice> = (set, get) => ({
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
    updateFee: (data: {
      class: string;
      weight: BigNumber;
      partialFee: BigNumber;
    }) =>
      set((state) => {
        state.transfers.fee = data;

        return state;
      }),
    updateFeeToken: (assetId: number) => {
      set((state) => {
        state.transfers.feeToken = assetId;
      });
    },
    getFeeToken: (network: SubstrateNetworkId): AssetMetadata | Token => {
      const balances = get().substrateBalances.assets[network];
      const token = Object.values(balances.assets).find(({ meta }) => {
        return meta.supportedNetwork[network] === get().transfers.feeToken;
      })?.meta;

      return token ?? balances.native.meta;
    },
  },
});
