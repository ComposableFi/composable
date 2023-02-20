import {
  useConnectedAccounts,
  useDotSamaContext,
  useParachainApi,
  useRelayChainApi,
} from "substrate-react";
import { DEFAULT_NETWORK_ID } from "../constants";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";

export const usePicassoProvider = () => useParachainApi("picasso");

export const useKaruraProvider = () => useParachainApi("karura");

export const useKusamaProvider = () => useRelayChainApi("kusama");

export const useSelectedAccount: () =>
  | InjectedAccountWithMeta
  | undefined = (): InjectedAccountWithMeta | undefined => {
  const { selectedAccount } = useDotSamaContext();
  const accounts = useConnectedAccounts(DEFAULT_NETWORK_ID);
  return accounts.length && selectedAccount !== -1
    ? accounts[selectedAccount]
    : undefined;
};

export const usePicassoAccounts = (): InjectedAccountWithMeta[] => {
  const accounts = useConnectedAccounts("picasso");
  return accounts;
};

export const useKusamaAccounts = (): InjectedAccountWithMeta[] => {
  const accounts = useConnectedAccounts("kusama");
  return accounts;
};

export * from "./useBlockInterval";
export * from "./useExistentialDeposit";
export * from "./useTransfer";
