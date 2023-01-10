import { InjectedAccountWithMeta } from "@polkadot/extension-inject/types";
import { useConnectedAccounts } from "./useConnectedAccounts";

export const usePicassoAccounts = (): InjectedAccountWithMeta[] =>
  useConnectedAccounts("picasso");
