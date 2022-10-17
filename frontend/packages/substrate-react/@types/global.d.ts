import type { InjectedWindowProvider } from "@polkadot/extension-inject/types";
import { SupportedWalletId } from "../src/dotsama/types";

declare global {
  interface Window {
    injectedWeb3: Record<SupportedWalletId, InjectedWindowProvider>;
  }
}
