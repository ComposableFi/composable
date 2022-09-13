import type { InjectedWindowProvider } from "@polkadot/extension-inject/types";
import { SupportedWalletId } from "@/dotsama";

declare global {
  interface Window {
    injectedWeb3: Record<SupportedWalletId, InjectedWindowProvider>;
  }
}
