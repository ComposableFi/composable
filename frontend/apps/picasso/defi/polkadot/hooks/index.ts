import { useDotSamaContext, useParachainApi, useRelayChainApi } from "substrate-react";

export const usePicassoProvider = () => {
  const api = useParachainApi(
    "picasso"
  );
  return api;
};

export const useKaruraProvider = () => {
  const api = useParachainApi(
    "karura"
  );
  return api;
};

export const useKusamaProvider = () => {
  const api = useRelayChainApi("kusama");
  return api;
};

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
