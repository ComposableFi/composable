import {
  PICASSO_STATEMINE_KSM_TRANSFER_FEE,
  PICASSO_SUPPORTED_TRANSFERS,
} from "@/defi/config";
import { SUBSTRATE_NETWORKS } from "@/defi/polkadot/Networks";
import { SubstrateNetworkId } from "@/defi/polkadot/types";
import BigNumber from "bignumber.js";
import { TokenId } from "tokens";
import { StoreSlice } from "@/stores/types";
import { XcmVersionedMultiLocation } from "@polkadot/types/lookup";
import {
  AcalaPrimitivesCurrencyCurrencyId,
  XcmVersionedMultiAsset,
  XcmVersionedMultiAssets,
} from "@acala-network/types/interfaces/types-lookup";
import { u128 } from "@polkadot/types-codec";
import { ApiPromise } from "@polkadot/api";
import { getAmountToTransfer } from "@/defi/polkadot/pallets/Transfer";
import { SubmittableExtrinsic } from "@polkadot/api/types";
import { TokenMetadata } from "../tokens/slice";

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
  existentialDeposit: BigNumber;
  feeToken: TokenId;
  selectedToken: TokenId;
  fee: {
    class: string;
    partialFee: BigNumber;
    weight: BigNumber;
  };
  tokenOptions: Array<TokenOption>;
  destinationMultiLocation: XcmVersionedMultiLocation | null;
  transferExtrinsic: null | ((...args: any[]) => any);
  multiAsset: SupportedTransferMultiAssets | null;
  hasFormError: boolean;
}

export type SupportedTransferMultiAssets =
  | u128
  | XcmVersionedMultiAsset
  | u128[]
  | u128[][]
  | AcalaPrimitivesCurrencyCurrencyId
  | XcmVersionedMultiAssets;

const networks = Object.keys(SUBSTRATE_NETWORKS)
  .map((networkId) => ({
    networkId: networkId as SubstrateNetworkId,
  }))
  .filter(({ networkId }) => PICASSO_SUPPORTED_TRANSFERS.includes(networkId));

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
  destinationMultiLocation: null,
  transferExtrinsic: null,
  multiAsset: null,
  hasFormError: false,
};

interface TransferActions {
  updateNetworks: (data: Omit<Networks, "options">) => void;
  updateAmount: (data: BigNumber) => void;
  updateRecipient: (selected: string) => void;
  updateTokenId: (data: TokenId) => void;
  flipKeepAlive: () => void;
  setFeeItem: (data: TokenId) => void;
  updateFee: (data: {
    class: string;
    weight: BigNumber;
    partialFee: BigNumber;
  }) => void;
  tokenOptions: Array<TokenOption>;
  updateExistentialDeposit: (data: BigNumber) => void;
  setFeeItemEd: (value: BigNumber) => void;
  setFeeToken: (data: TokenId) => void;
  getFeeToken: (network: SubstrateNetworkId) => TokenMetadata;
  updateSelectedToken: (token: TokenId) => void;
  getTransferTokenBalance: () => BigNumber;
  isTokenBalanceZero: (tokenId: TokenId) => boolean;
  setDestinationMultiLocation: (
    destination: XcmVersionedMultiLocation | null
  ) => void;
  setTransferExtrinsic: (call: any) => void;
  setTransferMultiAsset: (asset: SupportedTransferMultiAssets | null) => void;
  makeTransferCall: (
    api: ApiPromise,
    targetAddress: string | undefined
  ) => SubmittableExtrinsic<"promise"> | undefined;
  getTransferAmount: (api: ApiPromise) => u128;
  setFormError: (value: boolean) => void;
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
    setFeeItemEd: (value: BigNumber) => {
      set((state) => {
        state.transfers.feeItemEd = value;
      });
    },
    setFeeItem: (data: TokenId) =>
      set((state) => {
        state.transfers.feeItem = data;
      }),
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
    setFeeToken: (tokenId: TokenId) => {
      set((state) => {
        state.transfers.feeToken = tokenId;
      });
    },
    getFeeToken: (): TokenMetadata => {
      const tokens = get().substrateTokens.tokens;
      const tokenId = get().transfers.feeToken;
      return tokens[tokenId];
    },
    getTransferTokenBalance: () => {
      const from = get().transfers.networks.from;
      const tokenId = get().transfers.selectedToken;
      const balances = get().substrateBalances.balances;

      return balances[from][tokenId].free.minus(balances[from][tokenId].locked);
    },
    getTransferAmount: (api: ApiPromise) => {
      return getAmountToTransfer({
        amount: get().transfers.amount,
        api,
        token: get().substrateTokens.tokens[get().transfers.selectedToken],
        balance: get().transfers.getTransferTokenBalance(),
        existentialDeposit: get().transfers.existentialDeposit,
        keepAlive: get().transfers.keepAlive,
        sourceChain: get().transfers.networks.from,
        targetChain: get().transfers.networks.to,
      });
    },
    isTokenBalanceZero: (tokenId: TokenId) => {
      const from = get().transfers.networks.from;
      const balances = get().substrateBalances.balances;
      return balances[from][tokenId].free.eq(0);
    },
    setTransferExtrinsic: (call) => {
      set((state) => {
        state.transfers.transferExtrinsic = call;
      });
    },
    setDestinationMultiLocation: (
      destination: XcmVersionedMultiLocation | null
    ) => {
      set((state) => {
        state.transfers.destinationMultiLocation = destination;
      });
    },
    setTransferMultiAsset: (asset: SupportedTransferMultiAssets | null) => {
      set((state) => {
        state.transfers.multiAsset = asset;
      });
    },
    makeTransferCall: (api: ApiPromise, targetAddress: string | undefined) => {
      const transferExtrinsic = get().transfers.transferExtrinsic;
      const selectedAddress = get().transfers.recipients.selected;
      const recipient = selectedAddress.length
        ? selectedAddress
        : targetAddress;
      const destWeight = api.createType("u64", 9000000000); // > 9000000000
      const transferAmount = get().transfers.getTransferAmount(api);

      try {
        if (
          get().transfers.multiAsset === null ||
          transferExtrinsic === null ||
          get().transfers.destinationMultiLocation === null
        ) {
          console.log("no multi asset or transferExtrinsic or location");
          return; // bail if required params are not available.
        }

        if (
          get().transfers.networks.from === "kusama" &&
          get().transfers.networks.to === "picasso"
        ) {
          const feeAssetItem = api.createType("u32", 0); // First item in the list.
          const beneficiary = api.createType("XcmVersionedMultiLocation", {
            V0: api.createType("XcmV0MultiLocation", {
              X1: api.createType("XcmV0Junction", {
                AccountId32: {
                  network: api.createType("XcmV0JunctionNetworkId", "Any"),
                  id: api.createType("AccountId32", recipient),
                },
              }),
            }),
          });

          const args = [
            get().transfers.destinationMultiLocation,
            beneficiary,
            get().transfers.multiAsset,
            feeAssetItem,
          ];

          return transferExtrinsic(...args) as SubmittableExtrinsic<"promise">;
        }

        if (
          get().transfers.networks.from === "karura" &&
          get().transfers.networks.to === "picasso"
        ) {
          return transferExtrinsic(
            get().transfers.multiAsset,
            transferAmount,
            get().transfers.destinationMultiLocation,
            destWeight
          ) as SubmittableExtrinsic<"promise">;
        }

        if (get().transfers.networks.from === "statemine") {
          const beneficiary = api.createType("XcmVersionedMultiLocation", {
            V1: {
              parents: 0,
              interior: {
                X1: {
                  AccountId32: {
                    id: api.createType("AccountId32", recipient),
                    network: "Any",
                  },
                },
              },
            },
          });
          const feeAssetItem = api.createType("u32", 0); // First item in the list.

          const args = [
            get().transfers.destinationMultiLocation,
            beneficiary,
            get().transfers.multiAsset,
            feeAssetItem,
            api.createType("XcmV2WeightLimit", "Unlimited"),
          ];

          return transferExtrinsic(...args) as SubmittableExtrinsic<"promise">;
        }

        if (
          get().transfers.networks.from === "picasso" &&
          get().transfers.networks.to === "statemine"
        ) {
          const fee = api.createType("XcmVersionedMultiAsset", {
            V1: api.createType("XcmV1MultiAsset", {
              id: api.createType("XcmV1MultiassetAssetId", {
                Concrete: api.createType("XcmV1MultiLocation", {
                  parents: api.createType("u8", 1),
                  interior: api.createType(
                    "XcmV1MultilocationJunctions",
                    "Here"
                  ),
                }),
              }),
              fun: api.createType("XcmV1MultiassetFungibility", {
                Fungible: api.createType(
                  "Compact<u128>",
                  PICASSO_STATEMINE_KSM_TRANSFER_FEE
                ),
              }),
            }),
          });

          return transferExtrinsic(
            get().transfers.multiAsset,
            fee,
            get().transfers.destinationMultiLocation,
            api.createType("u64", 90000000000)
          );
        }

        // Else state where from is Picasso
        const args = [
          get().transfers.multiAsset,
          get().transfers.getTransferAmount(api),
          get().transfers.destinationMultiLocation,
          destWeight,
        ];
        return transferExtrinsic(...args) as SubmittableExtrinsic<"promise">;
      } catch {
        return;
      }
    },
    setFormError: (value) => {
      set((state) => {
        state.transfers.hasFormError = value;
      });
    },
  },
});
