import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import { TokenId } from "tokens";
import { StoreSlice } from "../../../types";
import { TokenMetadata } from "../tokens/slice";
import BigNumber from "bignumber.js";

export interface TokenOption {
  tokenId: TokenId;
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
  tokenId: TokenId;
  recipients: Recipients;
  keepAlive: boolean;
  feeItem: TokenId;
  feeItemEd: BigNumber;
  hasFeeItem: boolean;
  existentialDeposit: BigNumber;
  feeToken: TokenId;
  selectedToken: TokenId;
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
  feeItemEd: new BigNumber(0),
  keepAlive: true,
  existentialDeposit: new BigNumber(0),
  feeToken: "pica",
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
  updateTokenId: (data: TokenId) => void;
  flipKeepAlive: () => void;
  toggleHasFee: () => void;
  setFeeItem: (data: TokenId) => void;
  updateFee: (data: {
    class: string;
    weight: BigNumber;
    partialFee: BigNumber;
  }) => void;
  tokenOptions: Array<TokenOption>;
  updateExistentialDeposit: (data: BigNumber) => void;
  updateFeeToken: (data: TokenId) => void;
  getFeeToken: (network: SubstrateNetworkId) => TokenMetadata;
  updateSelectedToken: (token: TokenId) => void;
  getTransferTokenBalance: () => BigNumber;
  isTokenBalanceZero: (tokenId: TokenId) => boolean;
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
    updateTokenId: (data: TokenId) => {
      set((state) => {
        state.transfers.tokenId = data;
      });
    },
    updateSelectedToken: (data: TokenId) => {
      set((state) => {
        state.transfers.selectedToken = data;
      });
    },
    flipKeepAlive: () => {
      set((state) => {
        state.transfers.keepAlive = !state.transfers.keepAlive;
      });
    },
    setFeeItem: (data: TokenId) =>
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
    updateFeeToken: (tokenId: TokenId) => {
      set((state) => {
        state.transfers.feeToken = tokenId;
      });
    },
    getFeeToken: (): TokenMetadata => {
      const tokens = get().substrateTokens.tokens;
      const tokenId = get().transfers.feeToken;
      return tokens[tokenId]
    },
    getTransferTokenBalance: () => {
      const from = get().transfers.networks.from;
      const tokenId = get().transfers.selectedToken;
      const balances = get().substrateBalances.balances;
      return balances[from][tokenId].balance;
    },
    isTokenBalanceZero: (tokenId: TokenId) => {
      const from = get().transfers.networks.from;
      const balances = get().substrateBalances.balances;
      return balances[from][tokenId].balance.eq(0);
    },
  },
});
