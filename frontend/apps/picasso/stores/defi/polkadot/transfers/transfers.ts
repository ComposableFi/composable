import { StoreSlice } from "../../../types";
import BigNumber from "bignumber.js";
import { AssetId, SubstrateNetworkId } from "@/defi/polkadot/types";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { AssetMetadata } from "@/defi/polkadot/Assets";
import { Token } from "tokens";

export interface TokenOption {
  tokenId: AssetId;
  symbol: string;
  icon: string;
  disabled?: boolean;
  // balance: BigNumber;
}

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
  feeItem: AssetId;
  hasFeeItem: boolean;
  existentialDeposit: BigNumber;
  feeToken: number;
  selectedToken: AssetId;
  fee: {
    class: string;
    partialFee: BigNumber;
    weight: BigNumber;
  };
  tokenOptions: Array<TokenOption>;
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
  feeItem: "pica",
  keepAlive: true,
  existentialDeposit: new BigNumber(0),
  feeToken: 0,
  tokenOptions: [],
  selectedToken: "pica",
  fee: {
    class: "Normal",
    partialFee: new BigNumber(0),
    weight: new BigNumber(0),
  },
};

interface TransferActions {
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
  tokenOptions: Array<TokenOption>;
  updateExistentialDeposit: (data: BigNumber) => void;
  updateFeeToken: (data: number) => void;
  getFeeToken: (network: SubstrateNetworkId) => AssetMetadata | Token;
  updateSelectedToken: (token: AssetId) => void;
  getTransferTokenBalance: () => BigNumber;
  isTokenBalanceZero: (tokenId: AssetId) => boolean;
}

export interface TransfersSlice {
  transfers: TransfersState & TransferActions;
}

export const createTransfersSlice: StoreSlice<TransfersSlice> = (set, get) => ({
  transfers: {
    ...initialState,

    updateNetworks: (data: Omit<Networks, "options">) => {
      set((state) => {
        state.transfers.networks = { ...state.transfers.networks, ...data };
      });
    },
    updateAmount: (data: BigNumber) =>
      set((state) => {
        state.transfers.amount = data;
      }),
    updateRecipient: (data: string) => {
      set((state) => {
        state.transfers.recipients.selected = data;
      });
    },
    updateTokenId: (data: AssetId) => {
      set((state) => {
        state.transfers.tokenId = data;
      });
    },
    updateSelectedToken: (data: AssetId) => {
      set((state) => {
        state.transfers.selectedToken = data;
      });
    },
    flipKeepAlive: () => {
      set((state) => {
        state.transfers.keepAlive = !state.transfers.keepAlive;
      });
    },
    setFeeItem: (data: AssetId) =>
      set((state) => {
        state.transfers.feeItem = data;
      }),
    toggleHasFee: () => {
      set((state) => {
        state.transfers.hasFeeItem = !state.transfers.hasFeeItem;

        if (!state.transfers.hasFeeItem) {
          state.transfers.feeItem = "pica";
        }
      });
    },
    updateExistentialDeposit: (data: BigNumber) =>
      set((state) => {
        state.transfers.existentialDeposit = data;
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

    getTransferTokenBalance: () => {
      const from = get().transfers.networks.from;
      const assets = get().substrateBalances.assets[from].assets;
      const native = get().substrateBalances.assets[from].native;
      const tokenId = get().transfers.selectedToken;
      const isTokenNative = assets[tokenId].meta.supportedNetwork[from] === 1;
      return isTokenNative ? native.balance : assets[tokenId].balance;
    },
    isTokenBalanceZero: (tokenId: AssetId) => {
      const from = get().transfers.networks.from;
      const assets = get().substrateBalances.assets[from].assets;
      const native = get().substrateBalances.assets[from].native;
      const isTokenNative = assets[tokenId].meta.supportedNetwork[from] === 1;

      const balance = isTokenNative ? native.balance : assets[tokenId].balance;
      return balance.eq(0);
    },
  },
});
