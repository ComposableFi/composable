import type { InjectedWindowProvider } from "@polkadot/extension-inject/types";
import { SupportedWalletId } from "substrate-react";

declare global {
  interface Window {
    injectedWeb3: Record<SupportedWalletId, InjectedWindowProvider>;
  }
}
