import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { useConnectedAccounts } from "./useConnectedAccounts";

export const useKusamaAccounts = (): InjectedAccountWithMeta[] =>
  useConnectedAccounts("kusama");
