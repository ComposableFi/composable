import type { InjectedWindowProvider } from "@polkadot/extension-inject/types";
import { SupportedWalletId } from "substrate-react";

declare global {
  interface GlobalEventHandlersEventMap {
    TrackAnalytic: CustomEvent<{
      category: string;
      action: string;
      label?: string;
      value?: number;
      nonInteraction?: boolean;
    }>;

    PageViewAnalytic: CustomEvent<{
      path: string;
    }>;
  }

  interface Window {
    injectedWeb3: Record<SupportedWalletId, InjectedWindowProvider>;
  }
}
