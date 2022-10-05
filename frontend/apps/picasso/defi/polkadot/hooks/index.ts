import {
  useDotSamaContext,
  useParachainApi,
  useRelayChainApi
} from "substrate-react";

export const usePicassoProvider = () => useParachainApi("picasso");

export const useKaruraProvider = () => useParachainApi("karura");

export const useKusamaProvider = () => useRelayChainApi("kusama");

export const useSelectedAccount: () =>
  | { name: string; address: string }
  | undefined = (): { name: string; address: string } | undefined => {
  const { selectedAccount } = useDotSamaContext();
  const { accounts } = usePicassoProvider();
  return accounts.length && selectedAccount !== -1
    ? accounts[selectedAccount]
    : undefined;
};

export const useKusamaAccounts = (): { name: string; address: string }[] => {
  const { accounts } = useKusamaProvider();
  return accounts;
};

export * from "./useBlockInterval";
export * from "./useExistentialDeposit";
export * from "./useTransfer";
