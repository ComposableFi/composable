import { ConntectedAccount, DotSamaContext, ParachainId, RelayChainId } from '../types';
export declare const useDotSamaContext: () => DotSamaContext;
export declare const useParachainApi: (parachainId: ParachainId) => import("../types").ParachainApi;
export declare const useRelayChainApi: (relaychainId: RelayChainId) => import("../types").RelaychainApi;
export declare const useSelectedAccount: (parachainId: ParachainId) => ConntectedAccount | undefined;
