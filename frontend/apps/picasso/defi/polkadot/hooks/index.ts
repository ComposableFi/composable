import { useConnectedAccounts, useDotSamaContext } from "substrate-react";
import type { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import config from "@/constants/config";

export const usePicassoAccount: () => InjectedAccountWithMeta | undefined = ():
  | InjectedAccountWithMeta
  | undefined => {
  const { selectedAccount } = useDotSamaContext();
  const accounts = useConnectedAccounts(config.defaultNetworkId);
  return accounts.length && selectedAccount !== -1
    ? accounts[selectedAccount]
    : undefined;
};

export * from "../../../../../packages/substrate-react/src/dotsama/hooks/useBlockInterval";
export * from "./useExistentialDeposit";
export * from "./useTransfer";
