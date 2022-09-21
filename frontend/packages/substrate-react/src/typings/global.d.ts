import type { InjectedWindowProvider } from '@polkadot/extension-inject/types';

declare global {
  interface Window {
    injectedWeb3: Record<SupportedWalletId, InjectedWindowProvider>;
  }
}
