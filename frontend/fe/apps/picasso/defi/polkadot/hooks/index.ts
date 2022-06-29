import { useContext } from "react";
import { useParachainProvider } from "../context/hooks";
import { ParachainApi, ParachainContext } from "../context/ParachainContext";

export const usePicassoProvider = (): ParachainApi => {
  const api = useParachainProvider("kusama", 2019);
  return api;
};

export const useKusamaProvider = (): ParachainApi => {
  const api = useParachainProvider("kusama", 0);
  return api;
};

export const useSelectedAccount: () => ({ name: string; address: string } | undefined) = ():
  | { name: string; address: string }
  | undefined => {
  const { selectedAccount } = useContext(ParachainContext);
  const { accounts } = usePicassoProvider();
  return accounts.length && selectedAccount !== -1
    ? accounts[selectedAccount]
    : undefined;
};

export const useKusamaAccounts = (): { name: string; address: string }[] => {
  const { accounts } = useParachainProvider("kusama", 0);
  return accounts;
};
export * from "./useBlockInterval";
