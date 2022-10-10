import { ApiPromise, WsProvider } from "@polkadot/api";
import { Signer } from "@polkadot/api/types";
import {
  ChainIds,
  ChainId,
  ConnectedAccount,
  DotSamaExtensionStatus,
  SubstrateChainApi,
} from "../../types";
import { Networks } from "../../Networks";
import create from "zustand";
import type {
    InjectedExtension,
} from "@polkadot/extension-inject/types";

export interface SubstrateReactSlice {
  hasInitialized: boolean,
  chainApi: Record<ChainId, SubstrateChainApi>;
  extensionStatus: DotSamaExtensionStatus;
  selectedAccount: ConnectedAccount | undefined;
  signer: Signer | undefined;
  injectedExtension: InjectedExtension | undefined;
}

export const useSubstrateReact = create<SubstrateReactSlice>(() => ({
  hasInitialized: false,
  chainApi: ChainIds.reduce((agg, chainId) => {
    const provider = new WsProvider(Networks[chainId].wsUrl);
    agg[chainId] = {
        ...Networks[chainId],
      apiStatus: "initializing",
      api: new ApiPromise({ provider }),
      connectedAccounts: [],
    } as SubstrateChainApi;
    return agg;
  }, {} as Record<ChainId, SubstrateChainApi>),
  extensionStatus: "initializing",
  selectedAccount: undefined,
  signer: undefined,
  injectedExtension: undefined
}));

export const setSelectedAccount = (selectedAccount: ConnectedAccount | undefined) =>
  useSubstrateReact.setState((state) => {
    state.selectedAccount = selectedAccount;
    return state;
  });

export const setChainApi = (chainId: ChainId, chainApi: SubstrateChainApi) =>
  useSubstrateReact.setState((state) => ({
    chainApi: {
      ...state.chainApi,
      [chainId]: chainApi,
    },
  }));

export const setChainConnectedAccounts = (chainId: ChainId, connectedAccounts: ConnectedAccount[]) =>
  useSubstrateReact.setState((state) => {
    state.chainApi[chainId].connectedAccounts = connectedAccounts;
    return state;
  });

export const setExtensionStatus = (extensionStatus: DotSamaExtensionStatus) =>
  useSubstrateReact.setState((state) => {
    state.extensionStatus = extensionStatus;
    return state;
  });

export const setInjectedExtension = (injectedExtension: InjectedExtension) =>
  useSubstrateReact.setState((state) => {
    state.injectedExtension = injectedExtension;
    state.signer = injectedExtension.signer;
    return state;
  });

export const setHasInitialized = (hasInitialized: boolean) =>
  useSubstrateReact.setState((state) => {
    state.hasInitialized = hasInitialized;
    return state;
  });

export const setState = (substrateReactState: Partial<SubstrateReactSlice>) =>
  useSubstrateReact.setState((state) => ({
    ...state,
    ...substrateReactState,
  }));
